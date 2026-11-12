<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';

  interface Conversation {
    id: string;
    title: string;
    chatNumber: number;
    createdAt: number;
    updatedAt: number;
  }

  // State
  let showDeleteDiag = $state<boolean>(false);
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
</style>
