
<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open, save } from '@tauri-apps/plugin-dialog';
  import { readTextFile } from '@tauri-apps/plugin-fs';

  // --------------------------------------------------
  // Types
  // --------------------------------------------------

  type PlaybookStatus = 'active' | 'draft' | 'archived';

  interface Playbook {
    id: string;
    name: string;
    description: string;
    version: string;
    author?: string;
    tags: string[];
    filePath: string;
    createdAt: string;
    updatedAt: string;
    status: PlaybookStatus;
  }

  interface PlaybookContent {
    metadata: Playbook;
    content: string;
  }

  // --------------------------------------------------
  // State
  // --------------------------------------------------

  let playbooks = $state<Playbook[]>([]);

  let isLoading = $state(true);
  let isSaving = $state(false);

  let searchQuery = $state('');
  let activeFilter = $state('all');
  let currentPage = $state(1);

  // Create / Edit dialog
  let showCreateDialog = $state(false);
  let isEditing = $state(false);

  let newPlaybookName = $state('');
  let newPlaybookContent = $state('');

  let selectedPlaybook = $state<Playbook | null>(null);

  // Delete dialog
  let showDeleteDialog = $state(false);

  const pageSize = 5;

  // --------------------------------------------------
  // Lifecycle
  // --------------------------------------------------

  onMount(() => {
    loadPlaybooks();
  });

  // --------------------------------------------------
  // Backend Operations
  // --------------------------------------------------

  async function loadPlaybooks() {
    isLoading = true;

    try {
      playbooks = await invoke<Playbook[]>('get_all_playbooks');
    } catch (error) {
      console.error('Failed to load playbooks:', error);
      alert(`Could not load playbooks: ${error}`);
    } finally {
      isLoading = false;
    }
  }

  // --------------------------------------------------
  // Create / Edit
  // --------------------------------------------------

  function createPlaybook() {
    isEditing = false;
    selectedPlaybook = null;

    newPlaybookName = '';
    newPlaybookContent = '';

    showCreateDialog = true;
  }

  function closeCreateDialog() {
    if (isSaving) return;

    showCreateDialog = false;
    isEditing = false;
    selectedPlaybook = null;

    newPlaybookName = '';
    newPlaybookContent = '';
  }

  async function openPlaybook(playbook: Playbook) {
    await editPlaybook(playbook);
  }

  async function editPlaybook(playbook: Playbook) {
    try {
      isSaving = true;

      const result = await invoke<PlaybookContent>(
        'get_playbook_by_id',
        { id: playbook.id }
      );

      selectedPlaybook = result.metadata;

      newPlaybookName = result.metadata.name;
      newPlaybookContent = result.content;

      isEditing = true;
      showCreateDialog = true;
    } catch (error) {
      console.error('Failed to open playbook:', error);
      alert(`Could not open playbook: ${error}`);
    } finally {
      isSaving = false;
    }
  }

  async function savePlaybook() {
    if (!newPlaybookName.trim() || !newPlaybookContent.trim()) {
      return;
    }

    isSaving = true;

    try {
      if (isEditing && selectedPlaybook) {
        await invoke<Playbook>('update_playbook', {
          id: selectedPlaybook.id,
          name: newPlaybookName.trim(),
          content: newPlaybookContent
        });
      } else {
        await invoke<Playbook>('create_playbook', {
          name: newPlaybookName.trim(),
          content: newPlaybookContent
        });
      }

      closeCreateDialog();

      await loadPlaybooks();

    } catch (error) {
      console.error('Failed to save playbook:', error);
      alert(`Could not save playbook: ${error}`);
    } finally {
      isSaving = false;
    }
  }

  // --------------------------------------------------
  // Import Markdown
  // --------------------------------------------------

  async function importPlaybook() {
    try {
      const selected = await open({
        multiple: false,
        filters: [
          {
            name: 'Markdown',
            extensions: ['md', 'markdown']
          }
        ]
      });

      if (!selected || Array.isArray(selected)) {
        return;
      }

      isLoading = true;

      const content = await readTextFile(selected);

      const filename = selected.split(/[\\/]/).pop() ?? 'Imported Playbook';

      const name = filename.replace(/\.(md|markdown)$/i, '');

      await invoke<Playbook>('create_playbook', {
        name,
        content
      });

      await loadPlaybooks();

    } catch (error) {
      console.error('Failed to import playbook:', error);
      alert(`Could not import playbook: ${error}`);
    } finally {
      isLoading = false;
    }
  }

  // --------------------------------------------------
  // Export Markdown
  // --------------------------------------------------

  async function exportPlaybook(playbook: Playbook) {
    try {
      const filename = playbook.name.replace(/[<>:"/\\|?*]/g, '-');

      const destination = await save({
        defaultPath: `${filename}.md`,
        filters: [
          {
            name: 'Markdown',
            extensions: ['md']
          }
        ]
      });

      if (!destination) return;

      await invoke('export_playbook', {
        id: playbook.id,
        destination
      });

    } catch (error) {
      console.error('Failed to export playbook:', error);
      alert(`Could not export playbook: ${error}`);
    }
  }

  // --------------------------------------------------
  // Delete Playbook
  // --------------------------------------------------

  function confirmDeletePlaybook(playbook: Playbook) {
    selectedPlaybook = playbook;
    showDeleteDialog = true;
  }

  function closeDeleteDialog() {
    if (isSaving) return;

    showDeleteDialog = false;
    selectedPlaybook = null;
  }

  async function deletePlaybook() {
    if (!selectedPlaybook) return;

    isSaving = true;

    try {
      await invoke('delete_playbook', {
        id: selectedPlaybook.id
      });

      playbooks = playbooks.filter(
        (playbook) => playbook.id !== selectedPlaybook!.id
      );

      showDeleteDialog = false;
      selectedPlaybook = null;

      currentPage = Math.min(
        currentPage,
        Math.max(1, Math.ceil(playbooks.length / pageSize))
      );

    } catch (error) {
      console.error('Failed to delete playbook:', error);
      alert(`Could not delete playbook: ${error}`);
    } finally {
      isSaving = false;
    }
  }

  // --------------------------------------------------
  // Search and Filtering
  // --------------------------------------------------

  function handleSearch(value: string) {
    searchQuery = value;
    currentPage = 1;
  }

  function setFilter(value: string) {
    activeFilter = value;
    currentPage = 1;
  }

  // --------------------------------------------------
  // Derived Data
  // --------------------------------------------------

  let filteredPlaybooks = $derived(
    playbooks.filter((playbook) => {
      const query = searchQuery.trim().toLowerCase();

      const searchableContent = [
        playbook.name,
        playbook.description,
        playbook.author ?? '',
        playbook.filePath,
        ...playbook.tags
      ].join(' ').toLowerCase();

      const matchesSearch =
        !query || searchableContent.includes(query);

      const matchesFilter =
        activeFilter === 'all' ||
        playbook.status === activeFilter;

      return matchesSearch && matchesFilter;
    })
  );

  let totalPages = $derived(
    Math.max(1, Math.ceil(filteredPlaybooks.length / pageSize))
  );

  let paginatedPlaybooks = $derived(
    filteredPlaybooks.slice(
      (currentPage - 1) * pageSize,
      currentPage * pageSize
    )
  );

  let showingStart = $derived(
    filteredPlaybooks.length === 0
      ? 0
      : (currentPage - 1) * pageSize + 1
  );

  let showingEnd = $derived(
    Math.min(currentPage * pageSize, filteredPlaybooks.length)
  );

  // --------------------------------------------------
  // Helpers
  // --------------------------------------------------

  function formatDate(date: string): string {
    return new Date(date).toLocaleDateString('en-US', {
      month: 'short',
      day: '2-digit',
      year: 'numeric'
    });
  }

  function formatStatus(status: PlaybookStatus): string {
    return status.charAt(0).toUpperCase() + status.slice(1);
  }

  // --------------------------------------------------
  // Pagination
  // --------------------------------------------------

  function previousPage() {
    if (currentPage > 1) {
      currentPage--;
    }
  }

  function nextPage() {
    if (currentPage < totalPages) {
      currentPage++;
    }
  }

  function goToPage(page: number) {
    if (page >= 1 && page <= totalPages) {
      currentPage = page;
    }
  }
</script>


<div class="playbooks-page">

  <!-- Page Header -->

  <header class="page-header">
    <div>
      <h1>Playbooks</h1>
      <p>
        Manage reusable instructions and workflows for your AI agent.
      </p>
    </div>
  </header>

  <!-- Toolbar -->

  <div class="toolbar">

    <label class="search-box">
      <span>⌕</span>

      <input
        type="search"
        placeholder="Search playbooks..."
        aria-label="Search playbooks"
        value={searchQuery}
        oninput={(event) => handleSearch(event.currentTarget.value)}
      />
    </label>

    <div class="toolbar-actions">
      <button
        class="secondary-button"
        onclick={importPlaybook}
        disabled={isLoading}
      >
        + Import Markdown
      </button>

      <button
        class="primary-button"
        onclick={createPlaybook}
        disabled={isLoading}
      >
        + New playbook
      </button>
    </div>

  </div>

  <!-- Playbook Section -->

  <section class="playbook-section">

    <div class="section-header">

      <div>
        <h2>All playbooks</h2>
        <p>
          {playbooks.length} playbooks in your library
        </p>
      </div>

      <select
        class="filter-button"
        value={activeFilter}
        onchange={(event) => setFilter(event.currentTarget.value)}
        aria-label="Filter playbooks"
      >
        <option value="all">All playbooks</option>
        <option value="active">Active</option>
        <option value="draft">Draft</option>
        <option value="archived">Archived</option>
      </select>

    </div>

    <!-- Table -->

    <div class="playbook-list">

      <div class="playbook-row table-header">
        <span>Playbook</span>
        <span>Version</span>
        <span>Last modified</span>
        <span>Status</span>
        <span></span>
      </div>

      {#each paginatedPlaybooks as playbook (playbook.id)}

        <div class="playbook-row">

          <!-- Playbook Information -->

          <button
            class="playbook-name"
            onclick={() => openPlaybook(playbook)}
          >
            <div class="file-icon">MD</div>

            <div>
              <strong>{playbook.name}</strong>
              <small>{playbook.description || 'Markdown playbook'}</small>
            </div>
          </button>

          <!-- Version -->

          <span class="version">
            v{playbook.version}
          </span>

          <!-- Modified Date -->

          <span class="date">
            {formatDate(playbook.updatedAt)}
          </span>

          <!-- Status -->

          <span
            class="status"
            class:completed={playbook.status === 'active'}
            class:draft={playbook.status === 'draft'}
            class:archived={playbook.status === 'archived'}
          >
            {formatStatus(playbook.status)}
          </span>

          <!-- Options -->

          <details class="more-menu">

            <summary aria-label="Playbook options">
              ···
            </summary>

            <div class="menu-content">

              <button
                onclick={() => openPlaybook(playbook)}
              >
                Open
              </button>

              <button
                onclick={() => editPlaybook(playbook)}
              >
                Edit
              </button>

              <button
                onclick={() => exportPlaybook(playbook)}
              >
                Export Markdown
              </button>

              <button
                class="danger-action"
                onclick={() => confirmDeletePlaybook(playbook)}
              >
                Delete
              </button>

            </div>

          </details>

        </div>

      {:else}

        <!-- Empty State -->

        <div class="empty-state">

          <div class="file-icon">MD</div>

          <h3>No playbooks found</h3>

          <p>
            {searchQuery
              ? 'Try another search term.'
              : 'Create or import your first Markdown playbook to get started.'}
          </p>

          <div class="empty-actions">
            <button
              class="primary-button"
              onclick={createPlaybook}
            >
              + New playbook
            </button>

            <button
              class="secondary-button"
              onclick={importPlaybook}
            >
              Import Markdown
            </button>
          </div>

        </div>

      {/each}

    </div>

    <!-- Pagination -->

    {#if filteredPlaybooks.length > 0}

      <footer class="list-footer">

        <span>
          Showing {showingStart}–{showingEnd}
          of {filteredPlaybooks.length} playbooks
        </span>

        <div class="pagination">

          <button
            disabled={currentPage <= 1}
            onclick={previousPage}
          >
            Previous
          </button>

          {#each Array.from(
            { length: totalPages },
            (_, index) => index + 1
          ) as page (page)}

            <button
              class:current-page={currentPage === page}
              aria-current={currentPage === page ? 'page' : undefined}
              onclick={() => goToPage(page)}
            >
              {page}
            </button>

          {/each}

          <button
            disabled={currentPage >= totalPages}
            onclick={nextPage}
          >
            Next
          </button>

        </div>

      </footer>

    {/if}

  </section>

  <!-- Create / Edit Playbook Dialog -->

  {#if showCreateDialog}

    <div
      class="dialog-overlay"
      role="presentation"
      onclick={(event) => {
        if (event.target === event.currentTarget) {
          closeCreateDialog();
        }
      }}
    >

      <section
        class="create-dialog"
        aria-labelledby="dialog-title"
      >

        <header class="dialog-header">
          <div>
            <h2 id="dialog-title">
              {isEditing ? 'Edit playbook' : 'New playbook'}
            </h2>

            <p>
              {isEditing
                ? 'Update your Markdown playbook.'
                : 'Create a reusable Markdown playbook for your agent.'}
            </p>
          </div>

          <button
            class="close-button"
            aria-label="Close dialog"
            onclick={closeCreateDialog}
          >
            ×
          </button>
        </header>

        <div class="dialog-body">

          <label>
            <span>Playbook name</span>

            <input
              type="text"
              placeholder="e.g. Code Review"
              bind:value={newPlaybookName}
            />
          </label>

          <label>
            <span>Markdown content</span>

            <textarea
              placeholder="# My Playbook

You are an expert assistant...

## Instructions

1. Understand the task.
2. Analyze the requirements.
3. Execute the workflow.
4. Report the results."
              bind:value={newPlaybookContent}
              rows="14"
            ></textarea>
          </label>

        </div>

        <footer class="dialog-footer">

          <button
            class="cancel-button"
            onclick={closeCreateDialog}
            disabled={isSaving}
          >
            Cancel
          </button>

          <button
            class="primary-button"
            disabled={!newPlaybookName.trim() || !newPlaybookContent.trim() || isSaving}
            onclick={savePlaybook}
          >
            {isSaving
              ? 'Saving...'
              : isEditing
                ? 'Save changes'
                : 'Create playbook'}
          </button>

        </footer>

      </section>

    </div>

  {/if}

  <!-- Delete Confirmation Dialog -->

  {#if showDeleteDialog && selectedPlaybook}

    <div
      class="dialog-overlay"
      role="presentation"
      onclick={(event) => {
        if (event.target === event.currentTarget) {
          closeDeleteDialog();
        }
      }}
    >

      <section
        class="confirm-dialog"
        aria-labelledby="delete-dialog-title"
      >

        <header class="dialog-header">
          <div>
            <h2 id="delete-dialog-title">Delete playbook?</h2>

            <p>
              This will permanently delete
              <strong>{selectedPlaybook.name}</strong>.
            </p>
          </div>
        </header>

        <footer class="dialog-footer">

          <button
            class="cancel-button"
            onclick={closeDeleteDialog}
            disabled={isSaving}
          >
            Cancel
          </button>

          <button
            class="danger-button"
            onclick={deletePlaybook}
            disabled={isSaving}
          >
            {isSaving ? 'Deleting...' : 'Delete playbook'}
          </button>

        </footer>

      </section>

    </div>

  {/if}

</div>


<style>

:global(*) {
  box-sizing: border-box;
}

:global(body) {
  margin: 0;
  background: #faf9f6;
  color: #292824;
  font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}

.playbooks-page {
  width: 100%;
  margin: 0 auto;
  padding: 32px;
}

/* ----------------------------------
   Header
---------------------------------- */

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 28px;
}

h1 {
  margin: 0;
  font-family: Georgia, "Times New Roman", serif;
  font-size: 34px;
  font-weight: 400;
  letter-spacing: -0.7px;
}

.page-header p,
.section-header p {
  color: #77746c;
  font-size: 13px;
  line-height: 1.7;
}

.page-header p {
  max-width: 560px;
  margin: 10px 0 0;
}

/* ----------------------------------
   Toolbar
---------------------------------- */

.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  margin-bottom: 28px;
}

.toolbar-actions {
  display: flex;
  align-items: center;
  gap: 9px;
}

.search-box {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 340px;
  max-width: 100%;
  padding: 10px 12px;
  border: 1px solid #e5e2db;
  border-radius: 7px;
  background: #fffefa;
  color: #89867d;
}

.search-box input {
  width: 100%;
  min-width: 0;
  border: 0;
  outline: none;
  background: transparent;
  color: #292824;
  font-size: 12px;
}

button {
  font: inherit;
  cursor: pointer;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.primary-button,
.secondary-button {
  padding: 10px 14px;
  border-radius: 7px;
  font-size: 12px;
  white-space: nowrap;
  transition: background 0.15s ease, border-color 0.15s ease;
}

.primary-button {
  border: 1px solid #34332e;
  background: #34332e;
  color: #fffefa;
}

.primary-button:hover:not(:disabled) {
  background: #4b4942;
}

.secondary-button {
  border: 1px solid #e5e2db;
  background: #fffefa;
  color: #555249;
}

.secondary-button:hover:not(:disabled) {
  background: #f4f2ec;
  border-color: #cbc7bd;
}

/* ----------------------------------
   Playbook Section
---------------------------------- */

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 14px;
}

.section-header h2 {
  margin: 0;
  font-size: 15px;
  font-weight: 600;
}

.section-header p {
  margin: 5px 0 0;
  font-size: 12px;
}

.filter-button {
  padding: 8px 11px;
  border: 1px solid #e5e2db;
  border-radius: 6px;
  background: #fffefa;
  color: #555249;
  font-size: 11px;
  outline: none;
}

/* ----------------------------------
   Playbook Table
---------------------------------- */

.playbook-list {
  overflow-x: auto;
  border: 1px solid #e8e5de;
  border-radius: 9px;
  background: #fffefa;
}

.playbook-row {
  display: grid;
  grid-template-columns:
    minmax(230px, 2fr)
    minmax(80px, 0.7fr)
    minmax(115px, 1fr)
    minmax(100px, 0.8fr)
    28px;
  align-items: center;
  gap: 14px;
  min-width: 760px;
  padding: 14px 18px;
  border-bottom: 1px solid #efede7;
}

.playbook-row:last-child {
  border-bottom: 0;
}

.table-header {
  min-height: 42px;
  background: #f8f7f3;
  color: #858177;
  font-size: 10px;
  font-weight: 600;
}

/* ----------------------------------
   Playbook Name
---------------------------------- */

.playbook-name {
  display: flex;
  align-items: center;
  gap: 11px;
  min-width: 0;
  padding: 0;
  border: 0;
  background: transparent;
  text-align: left;
}

.playbook-name:hover strong {
  color: #8d725c;
}

.file-icon {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  width: 34px;
  height: 38px;
  border: 1px solid #e7e1d7;
  border-radius: 5px;
  background: #f7f4ed;
  color: #8d725c;
  font-size: 9px;
  font-weight: 700;
}

.playbook-name strong {
  display: block;
  overflow: hidden;
  color: #393832;
  font-size: 12px;
  font-weight: 550;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color 0.15s ease;
}

.playbook-name small {
  display: block;
  overflow: hidden;
  max-width: 260px;
  margin-top: 5px;
  color: #89867d;
  font-size: 10px;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.version,
.date {
  color: #69665e;
  font-size: 11px;
}

/* ----------------------------------
   Status
---------------------------------- */

.status {
  justify-self: start;
  padding: 5px 8px;
  border-radius: 5px;
  font-size: 10px;
  white-space: nowrap;
}

.completed {
  background: #edf3eb;
  color: #54744f;
}

.draft {
  background: #f7f0df;
  color: #96783c;
}

.archived {
  background: #efeeeb;
  color: #77746c;
}

/* ----------------------------------
   More Options
---------------------------------- */

.more-menu {
  position: relative;
  justify-self: end;
}

.more-menu summary {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: 5px;
  color: #77746c;
  font-size: 19px;
  cursor: pointer;
  list-style: none;
}

.more-menu summary::-webkit-details-marker {
  display: none;
}

.more-menu summary:hover {
  background: #f1efe9;
}

.menu-content {
  position: absolute;
  z-index: 10;
  top: calc(100% + 5px);
  right: 0;
  display: flex;
  flex-direction: column;
  width: 160px;
  padding: 5px;
  border: 1px solid #e8e5de;
  border-radius: 7px;
  background: #fffefa;
  box-shadow: 0 6px 20px rgba(40, 38, 30, 0.08);
}

.menu-content button {
  padding: 9px 10px;
  border: 0;
  border-radius: 4px;
  background: transparent;
  color: #49473f;
  font-size: 11px;
  text-align: left;
}

.menu-content button:hover {
  background: #f5f3ed;
}

.menu-content .danger-action {
  color: #b44c42;
}

.menu-content .danger-action:hover {
  background: #fbefed;
}

/* ----------------------------------
   Empty State
---------------------------------- */

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 280px;
  padding: 35px 20px;
  text-align: center;
}

.empty-state h3 {
  margin: 14px 0 0;
  color: #393832;
  font-size: 14px;
  font-weight: 600;
}

.empty-state p {
  max-width: 340px;
  margin: 8px 0 18px;
  color: #89867d;
  font-size: 11px;
  line-height: 1.7;
}

.empty-actions {
  display: flex;
  align-items: center;
  gap: 9px;
}

/* ----------------------------------
   Pagination
---------------------------------- */

.list-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-top: 14px;
  color: #89867d;
  font-size: 11px;
}

.pagination {
  display: flex;
  gap: 5px;
}

.pagination button {
  padding: 7px 9px;
  border: 1px solid #e5e2db;
  border-radius: 5px;
  background: #fffefa;
  color: #69665e;
  font-size: 10px;
}

.pagination button:hover:not(:disabled) {
  background: #f3f1eb;
}

.pagination .current-page {
  border-color: #34332e;
  background: #34332e;
  color: white;
}

/* ----------------------------------
   Dialog Overlay
---------------------------------- */

.dialog-overlay {
  position: fixed;
  z-index: 100;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: rgba(35, 34, 29, 0.38);
  backdrop-filter: blur(2px);
}

/* ----------------------------------
   Create / Edit Dialog
---------------------------------- */

.create-dialog,
.confirm-dialog {
  width: 100%;
  border: 1px solid #e8e5de;
  border-radius: 11px;
  background: #fffefa;
  box-shadow: 0 15px 50px rgba(35, 34, 29, 0.14);
}

.create-dialog {
  max-width: 620px;
}

.dialog-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 15px;
  padding: 21px 23px;
  border-bottom: 1px solid #efede7;
}

.dialog-header h2 {
  margin: 0;
  color: #34332e;
  font-size: 17px;
  font-weight: 600;
}

.dialog-header p {
  margin: 6px 0 0;
  color: #89867d;
  font-size: 11px;
  line-height: 1.6;
}

.close-button {
  display: flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: 0;
  border-radius: 5px;
  background: transparent;
  color: #77746c;
  font-size: 19px;
}

.close-button:hover {
  background: #f2f0ea;
}

.dialog-body {
  display: flex;
  flex-direction: column;
  gap: 19px;
  padding: 22px 23px;
}

.dialog-body label {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.dialog-body label span {
  color: #49473f;
  font-size: 11px;
  font-weight: 600;
}

.dialog-body input,
.dialog-body textarea {
  width: 100%;
  padding: 11px 12px;
  border: 1px solid #e5e2db;
  border-radius: 6px;
  outline: none;
  background: #fffefa;
  color: #34332e;
  font-family: inherit;
  font-size: 12px;
  transition: border-color 0.15s ease;
}

.dialog-body input:focus,
.dialog-body textarea:focus {
  border-color: #a69a84;
}

.dialog-body input::placeholder,
.dialog-body textarea::placeholder {
  color: #aaa69c;
}

.dialog-body textarea {
  min-height: 240px;
  resize: vertical;
  font-family: "SFMono-Regular", Consolas, monospace;
  font-size: 11px;
  line-height: 1.7;
}

.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 9px;
  padding: 15px 23px;
  border-top: 1px solid #efede7;
}

.cancel-button {
  padding: 9px 13px;
  border: 1px solid #e5e2db;
  border-radius: 6px;
  background: #fffefa;
  color: #69665e;
  font-size: 11px;
}

.cancel-button:hover:not(:disabled) {
  background: #f4f2ec;
}

/* ----------------------------------
   Delete Confirmation
---------------------------------- */

.confirm-dialog {
  max-width: 400px;
}

.confirm-dialog .dialog-header {
  padding: 23px;
}

.confirm-dialog .dialog-header p {
  margin-top: 9px;
}

.confirm-dialog .dialog-header strong {
  color: #49473f;
}

.danger-button {
  padding: 9px 13px;
  border: 1px solid #b44c42;
  border-radius: 6px;
  background: #b44c42;
  color: white;
  font-size: 11px;
}

.danger-button:hover:not(:disabled) {
  background: #983d35;
}

/* ----------------------------------
   Responsive
---------------------------------- */

@media (max-width: 650px) {
  .playbooks-page {
    padding: 24px 16px;
  }

  .toolbar {
    align-items: stretch;
    flex-direction: column;
  }

  .search-box {
    width: 100%;
  }

  .toolbar-actions {
    flex-wrap: wrap;
  }

  .section-header {
    align-items: flex-start;
  }

  .list-footer {
    align-items: flex-start;
    flex-direction: column;
  }

  .dialog-overlay {
    align-items: flex-start;
    overflow-y: auto;
    padding: 16px;
  }

  .create-dialog {
    margin: auto 0;
  }

  .dialog-body textarea {
    min-height: 190px;
  }
}

@media (max-width: 400px) {
  .toolbar-actions {
    flex-direction: column;
    align-items: stretch;
  }

  .toolbar-actions button {
    width: 100%;
  }

  .empty-actions {
    flex-direction: column;
    width: 100%;
  }
}

</style>