
<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";

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
  let search = $state("");

  // Pagination
  const pageSize = 8;
  let currentPage = $state(1);

  // Load conversations on mount
  onMount(() => {
    loadChats();
  });

  async function loadChats() {
    isLoading = true;
    error = null;

    try {
      const result = await invoke<Conversation[]>("list_chats");

      cases = result.sort(
        (a, b) => b.updatedAt - a.updatedAt
      );

      currentPage = 1;
    } catch (err) {
      console.error("Failed to load chats:", err);
      error = "Could not load conversation history.";
    } finally {
      isLoading = false;
    }
  }

  // Open existing conversation
  async function openChat(chatId: string) {
    await goto(
      `/app/investigations/${encodeURIComponent(chatId)}`
    );
  }

  // Navigate to new investigation
  function createInvestigation() {
    goto("/app/investigations/new");
  }

  // Format timestamps
  function formatDate(timestamp: number) {
    if (!timestamp || !Number.isFinite(timestamp)) {
      return "Unknown";
    }

    return new Intl.DateTimeFormat("en", {
      dateStyle: "medium",
      timeStyle: "short"
    }).format(timestamp * 1000);
  }

  // Search filtering
  let filteredCases = $derived.by(() => {
    const query = search.trim().toLowerCase();

    if (!query) {
      return cases;
    }

    return cases.filter((item) => {
      return (
        (item.title ?? "").toLowerCase().includes(query) ||
        (item.id ?? "").toLowerCase().includes(query) ||
        String(item.chatNumber ?? "").includes(query)
      );
    });
  });

  // Pagination calculations
  let totalPages = $derived(
    Math.max(1, Math.ceil(filteredCases.length / pageSize))
  );

  let paginatedCases = $derived(
    filteredCases.slice(
      (currentPage - 1) * pageSize,
      currentPage * pageSize
    )
  );

  let startItem = $derived(
    filteredCases.length === 0
      ? 0
      : (currentPage - 1) * pageSize + 1
  );

  let endItem = $derived(
    Math.min(currentPage * pageSize, filteredCases.length)
  );

  // Reset pagination when search changes
  function handleSearch(value: string) {
    search = value;
    currentPage = 1;
  }

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

  // Generate a display number for each case
  function getCaseNumber(index: number) {
    return (currentPage - 1) * pageSize + index + 1;
  }
</script>

<div class="investigations-page">

  <!-- Page Header -->
  <header class="page-header">
    <h1>Investigations</h1>

    <p>
      Manage forensic cases, track examination progress, and organize
      digital evidence in one workspace.
    </p>
  </header>

  <!-- Toolbar -->
  <div class="toolbar">

    <label class="search-box">
      <span aria-hidden="true">⌕</span>

      <input
        type="search"
        placeholder="Search investigations..."
        aria-label="Search investigations"
        value={search}
        oninput={(event) =>
          handleSearch(event.currentTarget.value)}
      />
    </label>

    <div class="stat-card">
      <span>Total investigations</span>

      <strong>{cases.length}</strong>

      <small>Across your workspace</small>
    </div>


    <button
      class="primary-button"
      onclick={createInvestigation}
    >
      <span aria-hidden="true">+</span>
      New investigation
    </button>

  </div>


  <!-- Investigation Section -->
  <section class="investigation-section">

    <div class="section-header">
      <div>
        <h2>All investigations</h2>

        <p>
          Review and continue your previous investigations.
        </p>
      </div>
    </div>

    <!-- Loading State -->
    {#if isLoading}

      <div class="empty-state" role="status">
        Loading investigations...
      </div>

    <!-- Error State -->
    {:else if error}

      <div class="empty-state" role="alert">
        <p>{error}</p>

        <button onclick={loadChats}>
          Try again
        </button>
      </div>

    <!-- No Conversations -->
    {:else if cases.length === 0}

      <div class="empty-state">
        <p>No investigations found.</p>

        <p>
          Create a new investigation to get started.
        </p>

        <button onclick={createInvestigation}>
          + New investigation
        </button>
      </div>

    <!-- No Search Results -->
    {:else if filteredCases.length === 0}

      <div class="empty-state">
        <p>No matching investigations.</p>

        <p>
          Try searching with another investigation title or ID.
        </p>

        <button onclick={() => handleSearch("")}>
          Clear search
        </button>
      </div>

    <!-- Investigation List -->
    {:else}

      <div class="investigation-list">

        <!-- Table Header -->
        <div class="investigation-row table-header">
          <span>Investigation</span>
          <span>Case ID</span>
          <span>Last Updated</span>
          <span>Action</span>
        </div>

        <!-- Dynamic Investigation Rows -->
        {#each paginatedCases as chat, index (chat.id)}

          <div class="investigation-row">

            <!-- Investigation Details -->
            <div class="case-name">

              <div class="case-icon">
                #{getCaseNumber(index)}
              </div>

              <div>
                <strong>
                  {chat.title || "Untitled investigation"}
                </strong>

              </div>

            </div>

            <!-- Case ID -->
            <span class="case-id" title={chat.id}>
              {chat.id}
            </span>

            <!-- Updated Date -->
            <span class="date">
              {formatDate(chat.updatedAt)}
            </span>

            <!-- Continue Action -->
            <button
              class="chat-action"
              onclick={() => openChat(chat.id)}
              aria-label={`Continue ${chat.title || "investigation"}`}
            >
              Continue
            </button>

          </div>

        {/each}

      </div>

      <!-- Pagination Footer -->
      <footer class="list-footer">

        <span>
          Showing {startItem}–{endItem}
          of {filteredCases.length} investigations
        </span>

        <div class="pagination">

          <button
            onclick={previousPage}
            disabled={currentPage === 1}
            aria-label="Previous page"
          >
            Previous
          </button>

          <button
            class="current-page"
            aria-current="page"
            disabled
          >
            {currentPage}
          </button>

          <button
            onclick={nextPage}
            disabled={currentPage >= totalPages}
            aria-label="Next page"
          >
            Next
          </button>

        </div>

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
    color: #292824;
    font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    -webkit-font-smoothing: antialiased;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  :global(button) {
    cursor: pointer;
  }

  .investigations-page {
    width: 100%;
    max-width: 1440px;
    margin: 0 auto;
    padding: 36px 40px 48px;
  }

  /* Page header */

  .page-header {
    margin-bottom: 30px;
  }

  .page-header h1 {
    margin: 0;
    color: #292824;
    font-family: Georgia, "Times New Roman", serif;
    font-size: clamp(28px, 3vw, 36px);
    font-weight: 400;
    letter-spacing: -0.8px;
    line-height: 1.2;
  }

  .page-header p {
    max-width: 560px;
    margin: 10px 0 0;
    color: #858177;
    font-size: 13px;
    line-height: 1.75;
  }

  /* Toolbar */

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 26px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 11px;
    width: 360px;
    max-width: 100%;
    min-height: 42px;
    padding: 0 13px;
    border: 1px solid #e7e4dc;
    border-radius: 7px;
    background: #fffefa;
    color: #89867d;
    transition: border-color 160ms ease, box-shadow 160ms ease;
  }
  .toolbar .stat-card {
  flex-direction: row;
  align-items: center;
  gap: 10px;
  padding: 8px 14px;
  min-height: 42px;
  border-radius: 7px;
}

.toolbar .stat-card strong {
  font-size: 17px;
}

.toolbar .stat-card small {
  display: none;
}

.toolbar .stat-card > span {
  font-size: 11px;
  white-space: nowrap;
}

.toolbar .search-box {
  flex: 1;
  width: auto;
}

  .search-box:focus-within {
    border-color: #aaa397;
    box-shadow: 0 0 0 3px rgba(141, 114, 92, 0.09);
  }

  .search-box span {
    flex-shrink: 0;
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
    font-size: 12px;
  }

  .search-box input::placeholder {
    color: #a19d94;
  }

  .search-box input::-webkit-search-cancel-button {
    cursor: pointer;
  }

  .primary-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    min-height: 40px;
    padding: 0 15px;
    border: 1px solid #34332e;
    border-radius: 7px;
    background: #34332e;
    color: #fffefa;
    font-size: 12px;
    font-weight: 500;
    white-space: nowrap;
    transition: background 160ms ease, transform 160ms ease;
  }

  .primary-button:hover {
    background: #504e46;
  }

  .primary-button:active {
    transform: translateY(1px);
  }

  .primary-button:focus-visible,
  .chat-action:focus-visible,
  .pagination button:focus-visible {
    outline: 2px solid #9b8068;
    outline-offset: 3px;
  }

  /* Overview */

  .overview {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 14px;
    margin-bottom: 34px;
  }

  .stat-card {
    display: flex;
    flex-direction: column;
    gap: 9px;
    min-width: 0;
    padding: 19px 20px;
    border: 1px solid #e8e5de;
    border-radius: 9px;
    background: #fffefa;
    transition: border-color 160ms ease, box-shadow 160ms ease;
  }

  .stat-card:hover {
    border-color: #d9d3c9;
    box-shadow: 0 3px 12px rgba(45, 42, 34, 0.035);
  }

  .stat-card > span {
    color: #77746c;
    font-size: 12px;
    font-weight: 450;
  }

  .stat-card strong {
    color: #34332e;
    font-size: 29px;
    font-weight: 500;
    letter-spacing: -0.8px;
    line-height: 1.2;
    font-variant-numeric: tabular-nums;
  }

  .stat-card small {
    color: #969187;
    font-size: 11px;
  }

  /* Investigation section */

  .investigation-section {
    min-width: 0;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 15px;
  }

  .section-header h2 {
    margin: 0;
    color: #393832;
    font-size: 15px;
    font-weight: 600;
    letter-spacing: -0.15px;
  }

  .section-header p {
    margin: 5px 0 0;
    color: #858177;
    font-size: 12px;
    line-height: 1.5;
  }

  /* Investigation table */

  .investigation-list {
    width: 100%;
    overflow-x: auto;
    border: 1px solid #e8e5de;
    border-radius: 9px;
    background: #fffefa;
    scrollbar-width: thin;
    scrollbar-color: #d8d3ca transparent;
  }

  .investigation-row {
    display: grid;
    grid-template-columns:
      minmax(240px, 2fr)
      minmax(110px, 0.85fr)
      minmax(150px, 1fr)
      minmax(90px, auto);
    align-items: center;
    gap: 18px;
    min-width: 760px;
    padding: 14px 20px;
    border-bottom: 1px solid #efede7;
    transition: background 140ms ease;
  }

  .investigation-row:last-child {
    border-bottom: 0;
  }

  .investigation-row:not(.table-header):hover {
    background: #fbfaf6;
  }

  .table-header {
    min-height: 43px;
    background: #f8f7f3;
    color: #858177;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.35px;
    text-transform: uppercase;
  }

  .table-header:hover {
    background: #f8f7f3;
  }

  /* Case information */

  .case-name {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }

  .case-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 36px;
    height: 36px;
    border: 1px solid #e7e1d7;
    border-radius: 7px;
    background: #f7f4ed;
    color: #8d725c;
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
    color: #393832;
    font-size: 12px;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .case-name small {
    display: block;
    overflow: hidden;
    margin-top: 5px;
    color: #89867d;
    font-size: 10px;
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

  /* Continue action */

  .chat-action {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    justify-self: start;
    min-height: 30px;
    padding: 0 12px;
    border: 1px solid #e5e2db;
    border-radius: 5px;
    background: #fffefa;
    color: #504e46;
    font-size: 11px;
    font-weight: 500;
    white-space: nowrap;
    transition: all 150ms ease;
  }

  .chat-action:hover {
    border-color: #34332e;
    background: #34332e;
    color: #fffefa;
  }

  /* Empty, loading and error states */

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    min-height: 190px;
    padding: 32px 20px;
    border: 1px dashed #e3dfd6;
    border-radius: 9px;
    background: #fffefa;
    color: #858177;
    text-align: center;
    font-size: 12px;
  }

  .empty-state p {
    margin: 0;
    line-height: 1.6;
  }

  .empty-state button {
    padding: 7px 12px;
    border: 1px solid #ded9cf;
    border-radius: 5px;
    background: #fffefa;
    color: #504e46;
    font-size: 11px;
    transition: background 150ms ease;
  }

  .empty-state button:hover {
    background: #f4f1e9;
  }

  /* Footer and pagination */

  .list-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-top: 15px;
    color: #89867d;
    font-size: 11px;
  }

  .pagination {
    display: flex;
    align-items: center;
    gap: 5px;
  }

  .pagination button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 30px;
    min-height: 29px;
    padding: 0 9px;
    border: 1px solid #e5e2db;
    border-radius: 5px;
    background: #fffefa;
    color: #69665e;
    font-size: 10px;
    transition: all 150ms ease;
  }

  .pagination button:not(:disabled):hover {
    border-color: #aaa397;
    background: #f7f5ef;
  }

  .pagination button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .pagination .current-page {
    border-color: #34332e;
    background: #34332e;
    color: #fffefa;
  }

  .pagination .current-page:hover {
    border-color: #34332e;
    background: #34332e;
  }

  /* Responsive */

  @media (max-width: 900px) {
    .investigations-page {
      padding: 30px 24px 40px;
    }

    .overview {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 650px) {
    .investigations-page {
      padding: 26px 16px 32px;
    }

    .page-header {
      margin-bottom: 23px;
    }

    .page-header p {
      font-size: 12px;
    }

    .toolbar {
      align-items: stretch;
      flex-direction: column;
      gap: 10px;
      margin-bottom: 22px;
    }

    .search-box {
      width: 100%;
      min-height: 42px;
    }

    .primary-button {
      align-self: flex-start;
    }

    .overview {
      grid-template-columns: 1fr;
      gap: 10px;
      margin-bottom: 28px;
    }

    .stat-card {
      padding: 15px 16px;
      gap: 7px;
    }

    .stat-card strong {
      font-size: 25px;
    }

    .section-header {
      align-items: flex-start;
    }

    .investigation-row {
      min-width: 700px;
      padding: 13px 14px;
      gap: 12px;
    }

    .list-footer {
      align-items: flex-start;
      flex-direction: column;
      gap: 12px;
    }
  }

  @media (max-width: 380px) {
    .investigations-page {
      padding-right: 12px;
      padding-left: 12px;
    }

    .page-header h1 {
      font-size: 28px;
    }

    .pagination button {
      min-width: 28px;
      padding: 0 7px;
    }
  }

  /* Reduced motion accessibility */

  @media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
      scroll-behavior: auto !important;
      transition-duration: 0.01ms !important;
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
    }
  }
</style>
