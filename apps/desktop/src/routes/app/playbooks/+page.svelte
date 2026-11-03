
<script lang="ts">
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

  // --------------------------------------------------
  // State
  // --------------------------------------------------
let showCreateDialog = $state(false);

let newPlaybookName = $state('');
let newPlaybookContent = $state('');

function createPlaybook() {
  showCreateDialog = true;
}

function closeCreateDialog() {
  showCreateDialog = false;
  newPlaybookName = '';
  newPlaybookContent = '';
}

function savePlaybook() {
  if (!newPlaybookName.trim() || !newPlaybookContent.trim()) {
    return;
  }

  const now = new Date().toISOString();

  const playbook: Playbook = {
    id: crypto.randomUUID(),
    name: newPlaybookName.trim(),
    description: 'Custom Markdown playbook',
    version: '1.0.0',
    author: 'Local',
    tags: [],
    filePath: `playbooks/${crypto.randomUUID()}.md`,
    createdAt: now,
    updatedAt: now,
    status: 'active'
  };

  playbooks = [playbook, ...playbooks];

  // TODO: Save Markdown content through Tauri command.

  closeCreateDialog();
}
  let playbooks = $state<Playbook[]>([
    
  ]);

  let searchQuery = $state('');
  let activeFilter = $state('all');
  let currentPage = $state(1);

  const pageSize = 5;

  // --------------------------------------------------
  // Derived data
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

  function handleSearch(value: string) {
    searchQuery = value;
    currentPage = 1;
  }

  function setFilter(value: string) {
    activeFilter = value;
    currentPage = 1;
  }

  // --------------------------------------------------
  // Actions
  // --------------------------------------------------

  function openPlaybook(playbook: Playbook) {
    // TODO: Load the Markdown file into the editor.
    console.log('Open playbook:', playbook.filePath);
  }

  function exportPlaybook(playbook: Playbook) {
    // TODO: Connect to the Rust/Tauri export command.
    console.log('Export playbook:', playbook.filePath);
  }

  function deletePlaybook(id: string) {
    // TODO: Add confirmation before permanent deletion.
    playbooks = playbooks.filter(
      (playbook) => playbook.id !== id
    );

    currentPage = Math.min(currentPage, totalPages);
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

    <button
      class="primary-button"
      onclick={createPlaybook}
    >
      + New playbook
    </button>

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
<div>
 <button class="playbook-name"
            onclick={() => openPlaybook(playbook)}
          >
            <div class="file-icon">MD</div>

            <div>
              <strong>{playbook.name}</strong>
            </div>
          </button>

</div>
         

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
                onclick={() => exportPlaybook(playbook)}
              >
                Export Markdown
              </button>

              <button
                onclick={() => deletePlaybook(playbook.id)}
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
              : 'Create your first Markdown playbook to get started.'}
          </p>

          <button
            class="primary-button"
            onclick={createPlaybook}
          >
            + New playbook
          </button>

        </div>

      {/each}

    </div>

    <!-- Pagination -->

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
    

  </section>

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
          <h2 id="dialog-title">New playbook</h2>
          <p>Create a reusable Markdown playbook for your agent.</p>
        </div>

        <button
          class="close-button"
          aria-label="Close dialog"
          onclick={closeCreateDialog}
        >
        X
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
        >
          Cancel
        </button>

        <button
          class="primary-button"
          disabled={!newPlaybookName.trim() || !newPlaybookContent.trim()}
          onclick={savePlaybook}
        >
          Create playbook
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

/* Header */

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 28px;
}

.page-header h1 {
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

/* Toolbar */

.toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  margin-bottom: 28px;
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

.search-box:focus-within {
  border-color: #aaa69b;
}

.search-box span {
  font-size: 19px;
  line-height: 1;
}

.search-box input {
  width: 100%;
  min-width: 0;
  border: 0;
  outline: none;
  background: transparent;
  color: #292824;
  font: inherit;
  font-size: 12px;
}

.search-box input::placeholder {
  color: #a09c93;
}

button {
  font: inherit;
  cursor: pointer;
}

.primary-button {
  padding: 10px 14px;
  border: 1px solid #34332e;
  border-radius: 7px;
  background: #34332e;
  color: #fffefa;
  font-size: 12px;
  white-space: nowrap;
  transition: background 0.2s ease;
}

.primary-button:hover {
  background: #514f47;
}

/* Section */

.playbook-section {
  width: 100%;
}

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
  outline: none;
  background: #fffefa;
  color: #555249;
  font-size: 11px;
  cursor: pointer;
}

/* Playbook Table */

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
    minmax(90px, 0.7fr)
    minmax(115px, 1fr)
    minmax(100px, 0.8fr)
    28px;
  align-items: center;
  gap: 14px;
  min-width: 640px;
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

/* Playbook Name */

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
  letter-spacing: 0.3px;
}

.playbook-name strong {
  display: block;
  overflow: hidden;
  color: #393832;
  font-size: 12px;
  font-weight: 550;
  text-overflow: ellipsis;
  white-space: nowrap;
  transition: color 0.2s ease;
}


/* Version and Date */

.version {
  color: #69665e;
  font-size: 11px;
  font-variant-numeric: tabular-nums;
}

.date {
  color: #69665e;
  font-size: 11px;
}

/* Status */

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
  color: #96743b;
}

.archived {
  background: #efeeeb;
  color: #77746c;
}

/* More Options */

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
  line-height: 1;
  list-style: none;
  cursor: pointer;
}

.more-menu summary::-webkit-details-marker {
  display: none;
}

.more-menu summary:hover {
  background: #f2f0ea;
}

.menu-content {
  position: absolute;
  z-index: 10;
  top: 30px;
  right: 0;
  display: flex;
  flex-direction: column;
  width: 155px;
  padding: 5px;
  border: 1px solid #e8e5de;
  border-radius: 7px;
  background: #fffefa;
  box-shadow: 0 5px 18px rgba(40, 38, 30, 0.08);
}

.menu-content button {
  padding: 9px 10px;
  border: 0;
  border-radius: 4px;
  background: transparent;
  color: #555249;
  font-size: 11px;
  text-align: left;
}

.menu-content button:hover {
  background: #f5f3ed;
}

.menu-content button:last-child {
  color: #a44e43;
}

/* Empty State */

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-width: 760px;
  padding: 55px 20px;
  text-align: center;
}

.empty-state .file-icon {
  margin-bottom: 12px;
}

.empty-state h3 {
  margin: 0;
  color: #393832;
  font-size: 14px;
  font-weight: 600;
}

.empty-state p {
  margin: 7px 0 18px;
  color: #89867d;
  font-size: 11px;
}

/* Footer */

.list-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-top: 14px;
  color: #89867d;
  font-size: 11px;
}

/* Pagination */

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

.pagination button:hover:not(:disabled):not(.current-page) {
  background: #f3f1eb;
}

.pagination button:disabled {
  opacity: 0.45;
  cursor: default;
}

.pagination .current-page {
  border-color: #34332e;
  background: #34332e;
  color: white;
}

/* Focus Accessibility */

button:focus-visible,
summary:focus-visible,
select:focus-visible,
input:focus-visible {
  outline: 2px solid #9b8b70;
  outline-offset: 3px;
}

.dialog-overlay {
  position: fixed;
  inset: 0;
  z-index: 100;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
  background: rgba(35, 33, 28, 0.35);
}

.create-dialog {
  width: 100%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  border: 1px solid #e8e5de;
  border-radius: 10px;
  background: #fffefa;
  box-shadow: 0 15px 50px rgba(0, 0, 0, 0.12);
}

.dialog-header,
.dialog-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 18px 20px;
}

.dialog-header {
  border-bottom: 1px solid #efede7;
}

.dialog-header h2 {
  margin: 0;
  font-size: 17px;
  font-weight: 600;
}

.dialog-header p {
  margin: 6px 0 0;
  color: #89867d;
  font-size: 11px;
}

.close-button {
  border: 0;
  background: transparent;
  color: #77746c;
  font-size: 23px;
}

.dialog-body {
  display: flex;
  flex-direction: column;
  gap: 18px;
  padding: 20px;
}

.dialog-body label {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.dialog-body label span {
  color: #555249;
  font-size: 12px;
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
  color: #292824;
  font: inherit;
  font-size: 12px;
}

.dialog-body textarea {
  min-height: 260px;
  resize: vertical;
  font-family: "SFMono-Regular", Consolas, monospace;
  line-height: 1.6;
}

.dialog-body input:focus,
.dialog-body textarea:focus {
  border-color: #aaa69b;
}

.dialog-footer {
  justify-content: flex-end;
  gap: 10px;
  border-top: 1px solid #efede7;
}

.cancel-button {
  padding: 10px 14px;
  border: 1px solid #e5e2db;
  border-radius: 7px;
  background: #fffefa;
  color: #555249;
  font-size: 12px;
}

.dialog-footer .primary-button:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}

@media (max-width: 600px) {
  .create-dialog {
    max-height: 95vh;
  }

  .dialog-body textarea {
    min-height: 200px;
  }
}

/* Responsive */

@media (max-width: 650px) {
  .playbooks-page {
    padding: 24px 16px;
  }

  .page-header {
    margin-bottom: 22px;
  }

  .page-header h1 {
    font-size: 29px;
  }

  .toolbar {
    align-items: stretch;
    flex-direction: column;
  }

  .search-box {
    width: 100%;
  }

  .primary-button {
    align-self: flex-start;
  }

  .section-header {
    align-items: flex-start;
  }

  .playbook-list {
    overflow-x: auto;
  }

  .list-footer {
    align-items: flex-start;
    flex-direction: column;
  }

  .pagination {
    flex-wrap: wrap;
  }
}

</style>