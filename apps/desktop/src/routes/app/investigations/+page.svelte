<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  type DeleteOption = 'yesterday' | 'week' | 'custom';

  let showDeleteDiag = $state(false);
  let showDeleteConfirm = $state(false);

  let deleteOption = $state<DeleteOption>('yesterday');
  let customDate = $state('');

  const today = $derived(new Date().toISOString().split('T')[0]);

  const options: {
    value: DeleteOption;
    title: string;
    subtitle: string;
  }[] = [
    {
      value: 'yesterday',
      title: 'Yesterday',
      subtitle: 'Delete chats from yesterday and everything before',
    },
    {
      value: 'week',
      title: 'One week ago',
      subtitle: 'Delete chats from 7 days ago and everything before',
    },
    {
      value: 'custom',
      title: 'Custom date',
      subtitle: 'Delete chats from the selected date and everything before',
    },
  ];

  const description = $derived.by(() => {
    switch (deleteOption) {
      case 'yesterday':
        return 'All chats created yesterday and before will be permanently deleted.';

      case 'week':
        return 'All chats created 7 days ago and before will be permanently deleted.';

      case 'custom':
        return customDate
          ? `All chats created on ${new Date(
              `${customDate}T00:00:00`
            ).toLocaleDateString()} and before will be permanently deleted.`
          : 'Please select a date to continue.';
    }
  });

  const canContinue = $derived(deleteOption !== 'custom' || !!customDate);

  function getCutoffDate(): Date {
    const cutoff = new Date();

    switch (deleteOption) {
      case 'yesterday':
        cutoff.setDate(cutoff.getDate() - 1);
        cutoff.setHours(0, 0, 0, 0);
        break;

      case 'week':
        cutoff.setDate(cutoff.getDate() - 7);
        cutoff.setHours(0, 0, 0, 0);
        break;

      case 'custom':
        return new Date(`${customDate}T00:00:00`);
    }

    return cutoff;
  }

  function openConfirmation() {
    if (!canContinue) return;

    showDeleteConfirm = true;
  }
  async function confirmDelete() {
    const cutoffDate = getCutoffDate();

    if (isNaN(cutoffDate.getTime())) return;

    try {
      // Unix timestamp in seconds (i64)
      const cutoffTimestamp = Math.floor(cutoffDate.getTime() / 1000);

      await invoke('delete_chat_before', {
        cutoffTimestamp,
      });

      closeDialog();
      loadChats();
    } catch (error) {
      console.error('Failed to delete chats:', error);
      consoleLogTerminal(`Failed to delete chats ${error}`);
    }
  }

  async function consoleLogTerminal(message: String) {
    await invoke('log_to_terminal', {
      message,
    });
  }

  function closeDialog() {
    showDeleteDiag = false;
    showDeleteConfirm = false;
    deleteOption = 'yesterday';
    customDate = '';
  }

  interface Conversation {
    id: string;
    title: string;
    chatNumber: number;
    createdAt: number;
    updatedAt: number;
  }

  // State
  let cases = $state<Conversation[]>([]);
  let isLoading = $state(true);
  let error = $state<string | null>(null);
  let search = $state('');

  // Pagination
  const pageSize = 8;
  let currentPage = $state(1);

  // Derived data
  let filteredCases = $derived.by(() => {
    const query = search.trim().toLowerCase();

    if (!query) return cases;

    return cases.filter(
      (item) =>
        (item.title ?? '').toLowerCase().includes(query) ||
        (item.id ?? '').toLowerCase().includes(query) ||
        String(item.chatNumber ?? '').includes(query)
    );
  });

  let totalPages = $derived(Math.max(1, Math.ceil(filteredCases.length / pageSize)));

  let paginatedCases = $derived(
    filteredCases.slice((currentPage - 1) * pageSize, currentPage * pageSize)
  );

  let startItem = $derived(filteredCases.length === 0 ? 0 : (currentPage - 1) * pageSize + 1);

  let endItem = $derived(Math.min(currentPage * pageSize, filteredCases.length));

  // Load conversations
  onMount(() => {
    loadChats();
  });

  async function loadChats() {
    isLoading = true;
    error = null;

    try {
      const result = await invoke<Conversation[]>('list_active_chats');

      cases = result.sort((a, b) => b.updatedAt - a.updatedAt);

      currentPage = 1;
    } catch (err) {
      console.error('Failed to load chats:', err);
      error = 'Could not load conversation history.';
    } finally {
      isLoading = false;
    }
  }

  // Navigation
  async function openChat(chatId: string) {
    await goto(`/app/investigations/${encodeURIComponent(chatId)}`);
  }

  function createInvestigation() {
    goto('/app/newchat');
  }

  // Date formatting
  function formatDate(timestamp: number) {
    if (!timestamp || !Number.isFinite(timestamp)) {
      return 'Unknown';
    }

    return new Intl.DateTimeFormat('en', {
      dateStyle: 'medium',
      timeStyle: 'short',
    }).format(timestamp * 1000);
  }

  // Search
  function handleSearch(value: string) {
    search = value;
    currentPage = 1;
  }

  function clearSearch() {
    handleSearch('');
  }

  // Pagination
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

  function getCaseNumber(index: number) {
    return (currentPage - 1) * pageSize + index + 1;
  }

  // Keyboard-first navigation
  function handleGlobalKeydown(event: KeyboardEvent) {
    const isMac = /Mac|iPhone|iPad/.test(navigator.platform);
    const modifierPressed = isMac ? event.metaKey : event.ctrlKey;

    // Ignore combinations involving Alt or Shift
    if (event.altKey || event.shiftKey) return;

    const target = event.target as HTMLElement | null;

    const isTyping =
      target?.tagName === 'INPUT' || target?.tagName === 'TEXTAREA' || target?.isContentEditable;

    // Ctrl + F / Cmd + F: Focus search
    if (modifierPressed && event.key.toLowerCase() === 'f') {
      event.preventDefault();
      searchInput?.focus();
      return;
    }

    // Don't interfere with normal typing
    if (isTyping || !modifierPressed) return;

    // Ctrl + 1..8 / Cmd + 1..8: Open investigation
    if (/^[1-8]$/.test(event.key)) {
      event.preventDefault();

      const index = Number(event.key) - 1;
      const selectedCase = paginatedCases[index];

      if (selectedCase) {
        openChat(selectedCase.id);
      }

      return;
    }

    // Ctrl + Left / Cmd + Left: Previous page
    if (event.key === 'ArrowLeft') {
      event.preventDefault();
      previousPage();
      return;
    }

    // Ctrl + Right / Cmd + Right: Next page
    if (event.key === 'ArrowRight') {
      event.preventDefault();
      nextPage();
      return;
    }
  }

  let searchInput: HTMLInputElement;
  let isMac = $state(false);

  onMount(() => {
    isMac = /Mac|iPhone|iPad/.test(navigator.userAgent);
    loadChats();
  });

  let shortcutModifier = $derived(isMac ? '⌘' : 'Ctrl');
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class="investigations-page">
  <!-- Page Header -->
  <header class="page-header">
    <h1>Investigations</h1>

    <p>
      Manage forensic cases, track examination progress, and organize digital evidence in one
      workspace.
    </p>
  </header>

  <!-- Toolbar -->

  <!-- Toolbar -->
  <div class="toolbar">
    <label class="search-box">
      <span aria-hidden="true"
        ><svg
          width="12px"
          height="12px"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g><g id="SVGRepo_iconCarrier">
            <path
              d="M16.6725 16.6412L21 21M19 11C19 15.4183 15.4183 19 11 19C6.58172 19 3 15.4183 3 11C3 6.58172 6.58172 3 11 3C15.4183 3 19 6.58172 19 11Z"
              stroke="#000000"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
          </g></svg
        ></span
      >

      <input
        bind:this={searchInput}
        id="investigation-search"
        type="search"
        placeholder="Search investigations..."
        aria-label="Search investigations"
        value={search}
        oninput={(event) => handleSearch(event.currentTarget.value)}
        onkeydown={(event) => {
          if (event.key === 'Escape' && search) {
            event.preventDefault();
            clearSearch();
          }
        }}
      />
    </label>

    <button class="primary-button" onclick={createInvestigation}>
      <span aria-hidden="true">+</span>
      New investigation
    </button>
    <button
      class="primary-button"
      aria-label="delete"
      onclick={() => {
        showDeleteDiag = true;
      }}
    >
      <span aria-hidden="true"
        ><svg
          width="18px"
          height="18px"
          viewBox="0 0 24 24"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
            id="SVGRepo_tracerCarrier"
            stroke-linecap="round"
            stroke-linejoin="round"
          ></g><g id="SVGRepo_iconCarrier">
            <path
              d="M10 12V17"
              stroke="#ffffff"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
            <path
              d="M14 12V17"
              stroke="#ffffff"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
            <path
              d="M4 7H20"
              stroke="#ffffff"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
            <path
              d="M6 10V18C6 19.6569 7.34315 21 9 21H15C16.6569 21 18 19.6569 18 18V10"
              stroke="#ffffff"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
            <path
              d="M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z"
              stroke="#ffffff"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ></path>
          </g></svg
        ></span
      >
    </button>
  </div>

  <!-- Keyboard Shortcuts -->
  <div class="keyboard-shortcuts" aria-label="Keyboard shortcuts">
    <div class="shortcut-group">
      <div class="shortcut-item">
        <span>Search</span>
        <span class="shortcut-keys">
          <kbd class="shortcut-key">{shortcutModifier}</kbd>
          <kbd class="shortcut-key">F</kbd>
        </span>
      </div>

      <div class="shortcut-item">
        <span>Open investigation</span>
        <span class="shortcut-keys">
          <kbd class="shortcut-key">{shortcutModifier}</kbd>
          <kbd class="shortcut-key">1–8</kbd>
        </span>
      </div>

      <div class="shortcut-item">
        <span>Previous / Next page</span>
        <span class="shortcut-keys">
          <kbd class="shortcut-key">{shortcutModifier}</kbd>
          <kbd class="shortcut-key">←</kbd>
          <kbd class="shortcut-key">→</kbd>
        </span>
      </div>
    </div>

    <span class="shortcut-item">
      {isMac ? 'Mac shortcuts' : 'Windows / Linux shortcuts'}
    </span>
  </div>

  <!-- Investigation Section -->
  <section class="investigation-section" aria-labelledby="investigation-heading">
    <div class="section-header">
      <div>
        <h2 id="investigation-heading">All investigations</h2>

        <p>Review and continue your previous investigations.</p>
      </div>
    </div>

    <!-- Loading -->
    {#if isLoading}
      <div class="empty-state" role="status" aria-live="polite">Loading investigations...</div>

      <!-- Error -->
    {:else if error}
      <div class="empty-state" role="alert">
        <p>{error}</p>

        <button onclick={loadChats}> Try again </button>
      </div>

      <!-- Empty -->
    {:else if cases.length === 0}
      <div class="empty-state">
        <p>No investigations found.</p>

        <p>Create a new investigation to get started.</p>

        <button onclick={createInvestigation}> + New investigation </button>
      </div>

      <!-- No Search Results -->
    {:else if filteredCases.length === 0}
      <div class="empty-state">
        <p>No matching investigations.</p>

        <p>Try searching with another investigation title or ID.</p>

        <button onclick={clearSearch}> Clear search </button>
      </div>

      <!-- Investigation List -->
    {:else}
      <div
        class="investigation-list"
        role="table"
        aria-label="Investigation history"
        aria-rowcount={filteredCases.length + 1}
      >
        <!-- Table Header -->
        <div class="investigation-row table-header" role="row">
          <span role="columnheader">Investigation</span>
          <span role="columnheader">Case ID</span>
          <span role="columnheader">Last Updated</span>
          <span role="columnheader">Action</span>
        </div>

        <!-- Investigation Rows -->
        {#each paginatedCases as chat, index (chat.id)}
          <div class="investigation-row" role="row" aria-rowindex={startItem + index + 1}>
            <!-- Investigation -->
            <div class="case-name" role="cell">
              <div class="case-icon" aria-hidden="true">
                #{getCaseNumber(index)}
              </div>

              <div>
                <strong>
                  {chat.title || 'Untitled investigation'}
                </strong>
              </div>
            </div>

            <!-- Case ID -->
            <span class="case-id" role="cell" title={chat.id}>
              {chat.id}
            </span>

            <!-- Updated Date -->
            <span class="date" role="cell">
              {formatDate(chat.updatedAt)}
            </span>

            <!-- Continue -->
            <div role="cell">
              <button
                class="chat-action"
                onclick={() => openChat(chat.id)}
                aria-label={`Continue ${chat.title || 'investigation'}`}
              >
                Continue
              </button>
            </div>
          </div>
        {/each}
      </div>

      <!-- Pagination -->
      <footer class="list-footer">
        <span aria-live="polite">
          Showing {startItem}–{endItem}
          of {filteredCases.length} investigations
        </span>

        <nav class="pagination" aria-label="Investigation pages">
          <button onclick={previousPage} disabled={currentPage === 1} aria-label="Previous page">
            Previous
          </button>

          <button
            class="current-page"
            aria-current="page"
            disabled
            aria-label={`Page ${currentPage} of ${totalPages}`}
          >
            {currentPage}
          </button>

          <button onclick={nextPage} disabled={currentPage >= totalPages} aria-label="Next page">
            Next
          </button>
        </nav>
      </footer>
    {/if}
  </section>
  {#if showDeleteDiag}
    <div class="dialog-overlay">
      {#if !showDeleteConfirm}
        <div
          class="create-dialog delete-chats-dialog"
          role="dialog"
          aria-modal="true"
          aria-labelledby="delete-dialog-title"
        >
          <div class="dialog-header">
            <div>
              <h2 id="delete-dialog-title">Delete chat history</h2>
              <p>Choose a date to delete conversations from that date and everything before it.</p>
            </div>

            <button class="close-button" onclick={closeDialog} aria-label="Close dialog">
              ×
            </button>
          </div>

          <div class="dialog-body">
            <div class="delete-options">
              {#each options as option (option.value)}
                <label class:active={deleteOption === option.value}>
                  <input
                    type="radio"
                    name="deleteOption"
                    value={option.value}
                    checked={deleteOption === option.value}
                    onchange={() => {
                      deleteOption = option.value;
                    }}
                  />

                  <div class="option-content">
                    <strong>{option.title}</strong>
                    <span>{option.subtitle}</span>
                  </div>
                </label>
              {/each}
            </div>

            {#if deleteOption === 'custom'}
              <label class="custom-date-field">
                <span>Delete chats on or before</span>

                <input type="date" bind:value={customDate} max={today} />
              </label>
            {/if}

            <div class="delete-preview">
              <span class="preview-label">Deletion preview</span>
              <p>{description}</p>
            </div>

            <div class="delete-warning">
              <span class="warning-icon">!</span>

              <div>
                <strong>This action cannot be undone</strong>
                <p>
                  All chats matching your selected date will be permanently deleted. Once deleted,
                  they cannot be recovered.
                </p>
              </div>
            </div>
          </div>

          <div class="dialog-footer">
            <button class="cancel-button" onclick={closeDialog}> Cancel </button>

            <button class="danger-button" onclick={openConfirmation} disabled={!canContinue}>
              Continue
            </button>
          </div>
        </div>
      {:else}
        <div
          class="confirm-dialog"
          role="alertdialog"
          aria-modal="true"
          aria-labelledby="confirm-delete-title"
        >
          <div class="dialog-header">
            <div>
              <h2 id="confirm-delete-title">Permanently delete chats?</h2>

              <p>{description}</p>

              <p>
                <strong>
                  This action is permanent. Deleted conversations cannot be recovered.
                </strong>
              </p>
            </div>
          </div>

          <div class="dialog-footer">
            <button class="cancel-button" onclick={() => (showDeleteConfirm = false)}>
              Go back
            </button>

            <button class="danger-button" onclick={confirmDelete}> Delete permanently </button>
          </div>
        </div>
      {/if}
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
    color: #302f2a;
    font-family:
      Inter,
      -apple-system,
      BlinkMacSystemFont,
      'Segoe UI',
      sans-serif;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  :global(button) {
    cursor: pointer;
  }

  /* Design tokens */

  .investigations-page {
    --primary: #34332e;
    --primary-hover: #34332e;
    --primary-soft: #f2edfa;

    --line: #eae7df;
    --muted: #89867e;
    --ink: #302f2a;
    --panel: #fffefa;
    --border-color: #e5e2db;

    width: 100%;
    max-width: 1440px;
    margin: 0 auto;
    padding: 12px 16px 20px;
    color: var(--ink);
  }

  /* Header */

  .page-header {
    margin-bottom: 20px;
  }

  .page-header h1 {
    margin: 0;
    font-family: Georgia, 'Times New Roman', serif;
    font-size: 34px;
    font-weight: 400;
    line-height: 1.2;
    letter-spacing: -0.8px;
  }

  .page-header p {
    max-width: 560px;
    margin: 7px 0 0;
    color: var(--muted);
    font-size: 12px;
    line-height: 1.6;
  }

  /* Toolbar */

  .toolbar {
    display: flex;
    align-items: center;
    gap: 9px;
    margin-bottom: 20px;
  }

  /* Search */

  .search-box {
    display: flex;
    align-items: center;
    flex: 1;
    min-width: 180px;
    height: 38px;
    gap: 9px;
    padding: 0 11px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--panel);
    transition: border-color 0.15s ease;
  }

  .search-box:focus-within {
    border-color: var(--primary);
  }

  .search-box span {
    color: var(--muted);
    font-size: 16px;
  }

  .search-box input {
    width: 100%;
    min-width: 0;
    height: 100%;
    border: 0;
    outline: none;
    background: transparent;
    color: var(--ink);
    font-size: 12px;
  }

  .search-box input::placeholder {
    color: #969187;
  }

  /* Statistics */

  /* Buttons */

  .primary-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 7px;
    min-height: 38px;
    padding: 0 13px;
    border: 1px solid var(--primary);
    border-radius: 6px;
    background: var(--primary);
    color: #fff;
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    transition: background 0.15s ease;
  }

  .primary-button:hover {
    border-color: var(--primary-hover);
    background: var(--primary-hover);
  }

  .chat-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 30px;
    padding: 0 11px;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    background: var(--panel);
    color: var(--ink);
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    transition:
      border-color 0.15s ease,
      background 0.15s ease;
  }

  .chat-action:hover {
    border-color: var(--primary);
    background: var(--primary-soft);
    color: var(--primary);
  }

  /* Investigation section */

  .investigation-section {
    min-width: 0;
  }

  .section-header {
    margin-bottom: 11px;
  }

  .section-header h2 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
  }

  .section-header p {
    margin: 5px 0 0;
    color: var(--muted);
    font-size: 11px;
    line-height: 1.5;
  }

  /* Investigation list */

  .investigation-list {
    width: 100%;
    overflow-x: auto;
    border: 1px solid var(--line);
    border-radius: 7px;
    background: var(--panel);
  }

  .investigation-row {
    display: grid;
    grid-template-columns:
      minmax(220px, 2fr)
      minmax(130px, 1fr)
      minmax(160px, 1.2fr)
      100px;
    align-items: center;
    gap: 13px;
    min-width: 740px;
    min-height: 54px;
    padding: 9px 14px;
    border-bottom: 1px solid #eeece6;
  }

  .investigation-row:last-child {
    border-bottom: 0;
  }

  .investigation-row:not(.table-header):hover {
    background: #f8f6fc;
  }

  .table-header {
    min-height: 38px;
    background: #f5f4ef;
    color: #77746c;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.3px;
    text-transform: uppercase;
  }

  /* Case information */

  .case-name {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .case-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 32px;
    height: 32px;
    border: 1px solid #e5e2db;
    border-radius: 5px;
    background: #f2edfa;
    color: var(--primary);
    font-size: 11px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
  }

  .case-name > div:last-child {
    min-width: 0;
  }

  .case-name strong {
    display: block;
    overflow: hidden;
    color: var(--ink);
    font-size: 12px;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .case-id,
  .date {
    overflow: hidden;
    color: #69665e;
    font-size: 11px;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-variant-numeric: tabular-nums;
  }

  /* Empty and error states */

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 9px;
    min-height: 170px;
    padding: 20px;
    border: 1px dashed #dedbd3;
    border-radius: 7px;
    background: var(--panel);
    color: var(--muted);
    text-align: center;
    font-size: 12px;
  }

  .empty-state p {
    margin: 0;
    line-height: 1.6;
  }

  .empty-state button {
    min-height: 30px;
    padding: 0 11px;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    background: var(--panel);
    color: var(--ink);
    font-size: 11px;
    transition: all 0.15s ease;
  }

  .empty-state button:hover {
    border-color: var(--primary);
    background: var(--primary-soft);
    color: var(--primary);
  }

  /* Pagination */

  .list-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    margin-top: 12px;
    color: var(--muted);
    font-size: 11px;
  }

  .pagination {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .pagination button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 30px;
    min-height: 28px;
    padding: 0 9px;
    border: 1px solid var(--border-color);
    border-radius: 5px;
    background: var(--panel);
    color: #504e46;
    font-size: 10px;
    transition: all 0.15s ease;
  }

  .pagination button:hover:not(:disabled) {
    border-color: var(--primary);
    background: var(--primary-soft);
    color: var(--primary);
  }

  .pagination button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .pagination .current-page {
    border-color: var(--primary);
    background: var(--primary);
    color: #fff;
  }

  /* Keyboard shortcuts */

  .keyboard-shortcuts {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 10px 16px;
    margin: -8px 0 20px;
    padding: 9px 12px;
    border: 1px solid var(--line);
    border-radius: 7px;
    background: #f5f4ef;
  }

  .shortcut-group {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 13px;
  }

  .shortcut-item {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    color: var(--muted);
    font-size: 10px;
    white-space: nowrap;
  }

  .shortcut-keys {
    display: inline-flex;
    align-items: center;
    gap: 3px;
  }

  .shortcut-key {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 22px;
    height: 21px;
    padding: 0 5px;
    border: 1px solid var(--border-color);
    border-bottom-width: 2px;
    border-radius: 4px;
    background: var(--panel);
    color: #504e46;
    font-family: inherit;
    font-size: 9px;
    font-weight: 600;
  }

  /* Keyboard accessibility */

  :global(button:focus-visible),
  :global(input:focus-visible),
  :global(.search-box:focus-within) {
    outline: 2px solid var(--primary);
    outline-offset: 2px;
  }

  :global(button:focus-visible) {
    position: relative;
    z-index: 1;
  }

  /* Responsive */

  @media (max-width: 800px) {
    .investigations-page {
      padding: 14px 14px 18px;
    }

    .toolbar {
      flex-wrap: wrap;
    }

    .search-box {
      flex-basis: 100%;
    }
  }

  @media (max-width: 650px) {
    .keyboard-shortcuts {
      align-items: flex-start;
      flex-direction: column;
    }

    .shortcut-group {
      gap: 9px 12px;
    }
  }

  @media (max-width: 600px) {
    .investigations-page {
      padding: 12px 12px 18px;
    }

    .page-header {
      margin-bottom: 17px;
    }

    .page-header h1 {
      font-size: 29px;
    }

    .toolbar {
      align-items: stretch;
      flex-direction: column;
      gap: 9px;
    }

    .search-box {
      flex: none;
      width: 100%;
    }

    .primary-button {
      align-self: flex-start;
    }

    .investigation-row {
      min-width: 740px;
      padding: 9px 12px;
    }

    .list-footer {
      align-items: flex-start;
      flex-direction: column;
    }

    .pagination {
      width: 100%;
      flex-wrap: wrap;
    }
  }

  /* Reduced motion */

  @media (prefers-reduced-motion: reduce) {
    :global(*) {
      scroll-behavior: auto !important;
      transition: none !important;
      animation: none !important;
    }
  }

  /* =========================================
   Delete Chat History Dialog
========================================= */

  /* Dialog overlay */

  .dialog-overlay {
    position: fixed;
    z-index: 1000;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    background: rgba(35, 34, 29, 0.38);
    backdrop-filter: blur(2px);
    animation: overlayFade 0.18s ease;
  }

  /* Dialog containers */

  .create-dialog,
  .confirm-dialog {
    width: 100%;
    box-sizing: border-box;
    border: 1px solid var(--line, #e5e2da);
    border-radius: 9px;
    background: var(--panel, #ffffff);
    box-shadow: 0 15px 50px rgba(35, 34, 29, 0.14);
    animation: dialogEnter 0.2s ease;
  }

  .create-dialog {
    max-width: 460px;
  }

  .confirm-dialog {
    max-width: 400px;
  }

  /* Dialog header */

  .dialog-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 12px;
    padding: 17px 18px;
    border-bottom: 1px solid #efede7;
  }

  .dialog-header h2 {
    margin: 0;
    color: var(--ink, #292820);
    font-size: 16px;
    font-weight: 600;
  }

  .dialog-header p {
    margin: 6px 0 0;
    color: var(--muted, #858176);
    font-size: 11px;
    line-height: 1.6;
  }

  .dialog-header p strong {
    color: #983d35;
    font-weight: 600;
  }

  /* Close button */

  .close-button {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: center;
    width: 27px;
    height: 27px;
    padding: 0;
    border: 0;
    border-radius: 5px;
    background: transparent;
    color: var(--muted, #858176);
    font-size: 19px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .close-button:hover {
    background: var(--primary-soft, #f2f0e9);
    color: var(--primary, #68634e);
  }

  /* Dialog body */

  .dialog-body {
    display: flex;
    flex-direction: column;
    gap: 15px;
    padding: 17px 18px;
  }

  /* Date selection options */

  .delete-options {
    display: flex;
    flex-direction: column;
    gap: 7px;
  }

  .delete-options label {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 11px;
    padding: 11px 12px;
    border: 1px solid var(--border-color, #e5e2da);
    border-radius: 6px;
    background: var(--panel, #ffffff);
    cursor: pointer;
    transition:
      border-color 0.15s ease,
      background 0.15s ease;
  }

  .delete-options label:hover {
    border-color: var(--primary, #827b63);
    background: var(--primary-soft, #f6f4ee);
  }

  .delete-options label.active {
    border-color: var(--primary, #827b63);
    background: var(--primary-soft, #f6f4ee);
  }

  .delete-options input[type='radio'] {
    width: 14px;
    height: 14px;
    flex-shrink: 0;
    margin: 0;
    padding: 0;
    accent-color: var(--primary, #827b63);
    cursor: pointer;
  }

  .option-content {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .option-content strong {
    color: var(--ink, #292820);
    font-size: 12px;
    font-weight: 600;
  }

  .option-content span {
    color: var(--muted, #858176);
    font-size: 10px;
    line-height: 1.5;
  }

  /* Custom date picker */

  .custom-date-field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .custom-date-field span {
    color: #49473f;
    font-size: 11px;
    font-weight: 600;
  }

  .custom-date-field input {
    width: 100%;
    box-sizing: border-box;
    padding: 9px 11px;
    border: 1px solid var(--border-color, #e5e2da);
    border-radius: 6px;
    outline: none;
    background: var(--panel, #ffffff);
    color: var(--ink, #292820);
    font-family: inherit;
    font-size: 12px;
    transition: border-color 0.15s ease;
  }

  .custom-date-field input:focus {
    border-color: var(--primary, #827b63);
  }

  .custom-date-field input::-webkit-calendar-picker-indicator {
    cursor: pointer;
    opacity: 0.65;
  }

  /* Deletion preview */

  .delete-preview {
    padding: 11px 12px;
    border: 1px solid var(--border-color, #e5e2da);
    border-radius: 6px;
    background: var(--panel, #ffffff);
  }

  .preview-label {
    color: var(--muted, #858176);
    font-size: 10px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.4px;
  }

  .delete-preview p {
    margin: 5px 0 0;
    color: var(--ink, #292820);
    font-size: 11px;
    line-height: 1.6;
  }

  /* Permanent deletion warning */

  .delete-warning {
    display: flex;
    align-items: flex-start;
    gap: 10px;
    padding: 12px;
    border: 1px solid #e8c7c2;
    border-radius: 6px;
    background: #fcf4f2;
  }

  .warning-icon {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: center;
    width: 17px;
    height: 17px;
    margin-top: 1px;
    border-radius: 50%;
    background: #b44c42;
    color: #ffffff;
    font-size: 11px;
    font-weight: 700;
  }

  .delete-warning strong {
    display: block;
    color: #983d35;
    font-size: 11px;
    font-weight: 600;
  }

  .delete-warning p {
    margin: 4px 0 0;
    color: #79534e;
    font-size: 10px;
    line-height: 1.6;
  }

  /* Dialog footer */

  .dialog-footer {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 8px;
    padding: 12px 18px;
    border-top: 1px solid #efede7;
  }

  /* Cancel button */

  .cancel-button {
    min-height: 32px;
    padding: 0 11px;
    border: 1px solid var(--border-color, #e5e2da);
    border-radius: 6px;
    background: var(--panel, #ffffff);
    color: #69665e;
    font-family: inherit;
    font-size: 11px;
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .cancel-button:hover {
    background: #f4f2ec;
  }

  /* Danger button */

  .danger-button {
    min-height: 32px;
    padding: 0 12px;
    border: 1px solid #b44c42;
    border-radius: 6px;
    background: #b44c42;
    color: #ffffff;
    font-family: inherit;
    font-size: 11px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .danger-button:hover:not(:disabled) {
    border-color: #983d35;
    background: #983d35;
  }

  .danger-button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  /* Confirmation dialog */

  .confirm-dialog .dialog-header {
    padding: 18px;
  }

  .confirm-dialog .dialog-header p {
    margin-top: 8px;
  }

  .confirm-dialog .dialog-footer {
    padding: 12px 18px;
  }

  /* Keyboard focus accessibility */

  .close-button:focus-visible,
  .cancel-button:focus-visible,
  .danger-button:focus-visible,
  .delete-options input:focus-visible,
  .custom-date-field input:focus-visible {
    outline: 2px solid var(--primary, #827b63);
    outline-offset: 2px;
  }

  /* Animations */

  @keyframes overlayFade {
    from {
      opacity: 0;
    }

    to {
      opacity: 1;
    }
  }

  @keyframes dialogEnter {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.99);
    }

    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  /* Responsive layout */

  @media (max-width: 480px) {
    .dialog-overlay {
      padding: 12px;
    }

    .create-dialog,
    .confirm-dialog {
      max-width: 100%;
    }

    .dialog-header {
      padding: 15px;
    }

    .dialog-body {
      padding: 14px;
      gap: 12px;
    }

    .dialog-footer {
      padding: 12px 14px;
    }
  }

  /* Reduced motion */

  @media (prefers-reduced-motion: reduce) {
    .dialog-overlay,
    .create-dialog,
    .confirm-dialog {
      animation: none;
    }

    .delete-options label,
    .danger-button,
    .cancel-button,
    .close-button {
      transition: none;
    }
  }
</style>
