<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { openPath } from '@tauri-apps/plugin-opener';

  interface Report {
    case_id: string;
    file_name: string;
    title: string | null;
    file_path: string;
    file_size: number;
    created_at: number;
    modified_at: number;
  }

  let reports = $state<Report[]>([]);
  let isLoading = $state(true);
  let error = $state('');

  let search = $state('');
  let dateFilter = $state('all');
  let sortBy = $state('modified');
  let currentPage = $state(1);

  const pageSize = 8;

  onMount(() => {
    loadReports();
  });

  async function loadReports() {
    isLoading = true;
    error = '';

    try {
      reports = await invoke<Report[]>('get_all_reports');
    } catch (err) {
      console.error('Failed to load reports:', err);
      error = `Could not load reports: ${err}`;
    } finally {
      isLoading = false;
    }
  }

  async function openReport(report: Report) {
    try {
      await openPath(report.file_path);
    } catch (err) {
      consoleLogTerminal(`Failed to open report ${err}`);
      console.error('Failed to open report:', err);
      alert(`Could not open report: ${err}`);
    }
  }

  function formatDate(timestamp: number): string {
    if (!timestamp) return 'Unknown';

    return new Intl.DateTimeFormat(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    }).format(new Date(timestamp * 1000));
  }

  async function consoleLogTerminal(message: String) {
    await invoke('log_to_terminal', {
      message,
    });
  }

  function formatFileSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;

    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  let filteredReports = $derived.by(() => {
    const query = search.trim().toLowerCase();

    let result = reports.filter((report) => {
      const matchesSearch =
        !query ||
        report.file_name.toLowerCase().includes(query) ||
        report.case_id.toLowerCase().includes(query) ||
        (report.title?.toLowerCase().includes(query) ?? false);

      const reportDate = new Date(report.created_at * 1000);
      const now = new Date();

      const matchesDate =
        dateFilter === 'all' ||
        (dateFilter === '7days' && reportDate >= new Date(now.getTime() - 7 * 86400000)) ||
        (dateFilter === '30days' && reportDate >= new Date(now.getTime() - 30 * 86400000));

      return matchesSearch && matchesDate;
    });

    result = [...result].sort((a, b) => {
      if (sortBy === 'created') {
        return b.created_at - a.created_at;
      }

      if (sortBy === 'name') {
        return a.file_name.localeCompare(b.file_name);
      }

      return b.modified_at - a.modified_at;
    });

    return result;
  });

  let totalPages = $derived(Math.max(1, Math.ceil(filteredReports.length / pageSize)));

  let paginatedReports = $derived(
    filteredReports.slice((currentPage - 1) * pageSize, currentPage * pageSize)
  );

  let totalSize = $derived(reports.reduce((total, report) => total + report.file_size, 0));

  let totalSizeFormatted = $derived(formatFileSize(totalSize));

  $effect(() => {
    search;
    dateFilter;
    sortBy;
    currentPage = 1;
  });
</script>

<div class="reports-page">
  <header class="page-header">
    <h1>Reports</h1>
    <p>Browse, search, and access reports generated from your forensic investigations.</p>
  </header>

  <div class="toolbar">
    <label class="search-box">
      <span
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
        type="search"
        placeholder="Search by report name or case ID..."
        aria-label="Search reports"
        bind:value={search}
      />
    </label>

    <button class="filter-button" onclick={loadReports} disabled={isLoading}> ↻ Refresh </button>
  </div>

  <section class="report-section">
    <div class="section-header">
      <div>
        <h2>All reports</h2>
        <p>Reports from your forensic investigations</p>
      </div>

      <div class="filters">
        <select bind:value={dateFilter} aria-label="Filter by date">
          <option value="all">All dates</option>
          <option value="7days">Last 7 days</option>
          <option value="30days">Last 30 days</option>
        </select>

        <select bind:value={sortBy} aria-label="Sort reports">
          <option value="modified">Recently modified</option>
          <option value="created">Recently created</option>
          <option value="name">Name</option>
        </select>
      </div>
    </div>

    <div class="report-list">
      <div class="report-row table-header">
        <span>Report</span>
        <span>Case ID</span>
        <span>Last modified</span>
        <span>File size</span>
        <span></span>
      </div>

      {#if isLoading}
        <div class="empty-state">Loading reports...</div>
      {:else if error}
        <div class="empty-state error-state">
          <p>{error}</p>
          <button class="filter-button" onclick={loadReports}> Try again </button>
        </div>
      {:else if filteredReports.length === 0}
        <div class="empty-state">
          <strong>
            {search ? 'No matching reports' : 'No reports found'}
          </strong>
          <p>
            {search
              ? 'Try changing your search or filters.'
              : 'Generated PDF reports will appear here.'}
          </p>
        </div>
      {:else}
        {#each paginatedReports as report (report.file_path)}
          <div
            class="report-row clickable-row"
            role="button"
            tabindex="0"
            onclick={() => openReport(report)}
            onkeydown={(event) => {
              if (event.key === 'Enter' || event.key === ' ') {
                event.preventDefault();
                openReport(report);
              }
            }}
            title="Open PDF report"
          >
            <div class="report-name">
              <div class="file-icon">PDF</div>
              <div>
                <strong>{report.title || report.file_name}</strong>
                <small>{report.file_name}</small>
              </div>
            </div>

            <span class="case-id">{report.case_id}</span>

            <span class="date">
              {formatDate(report.modified_at)}
            </span>

            <span class="date">
              {formatFileSize(report.file_size)}
            </span>

            <span class="open-indicator">↗</span>
          </div>
        {/each}
      {/if}
    </div>

    <footer class="list-footer">
      <span>
        Showing
        {filteredReports.length === 0 ? 0 : (currentPage - 1) * pageSize + 1}
        –
        {Math.min(currentPage * pageSize, filteredReports.length)}
        of {filteredReports.length} reports
      </span>

      <div class="pagination">
        <button disabled={currentPage === 1} onclick={() => currentPage--}> Previous </button>

        {#each Array.from({ length: totalPages }, (_, i) => i + 1) as page}
          <button class:current-page={currentPage === page} onclick={() => (currentPage = page)}>
            {page}
          </button>
        {/each}

        <button disabled={currentPage >= totalPages} onclick={() => currentPage++}> Next </button>
      </div>
    </footer>
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
  :global(input),
  :global(select) {
    font: inherit;
  }

  :global(button) {
    cursor: pointer;
  }

  :global(button:disabled),
  :global(select:disabled) {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Page */

  .reports-page {
    width: 100%;
    max-width: 1440px;
    margin: 0 auto;
    padding: 12px 16px 20px;
    color: #302f2a;
  }

  /* Header */

  .page-header {
    margin-bottom: 22px;
    padding-bottom: 10px;
    border-bottom: 1px solid #eae7df;
  }

  h1 {
    margin: 0;
    font-family: Georgia, 'Times New Roman', serif;
    font-size: 34px;
    font-weight: 400;
    letter-spacing: -0.8px;
  }

  .page-header p,
  .section-header p {
    color: #89867e;
    font-size: 12px;
    line-height: 1.6;
  }

  .page-header p {
    max-width: 560px;
    margin: 8px 0 0;
  }

  /* Toolbar */

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    margin-bottom: 22px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 9px;
    width: 340px;
    max-width: 100%;
    min-height: 36px;
    padding: 7px 11px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background: #fffefa;
    color: #89867d;
    transition: border-color 0.15s ease;
  }

  .search-box:focus-within {
    border-color: #34332e;
    outline: 2px solid rgba(139, 92, 246, 0.12);
  }

  .search-box span {
    font-size: 16px;
  }

  .search-box input {
    width: 100%;
    min-width: 0;
    border: 0;
    outline: none;
    background: transparent;
    color: #302f2a;
    font-size: 12px;
  }

  .search-box input::placeholder {
    color: #a09c92;
  }

  .filter-button {
    min-height: 36px;
    padding: 7px 12px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background: #fffefa;
    color: #555249;
    font-size: 11px;
    transition: all 0.15s ease;
  }

  .filter-button:hover:not(:disabled) {
    border-color: #34332e;
    background: #f4f2ec;
    color: #34332e;
  }

  /* Report Section */

  .report-section {
    width: 100%;
    min-width: 0;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 10px;
  }

  .section-header h2 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
  }

  .section-header p {
    margin: 5px 0 0;
    font-size: 11px;
  }

  .filters {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .filters select {
    min-height: 34px;
    padding: 6px 10px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    outline: none;
    background: #fffefa;
    color: #555249;
    font-size: 11px;
  }

  .filters select:focus {
    border-color: #34332e;
  }

  /* Report List */

  .report-list {
    width: 100%;
    overflow-x: auto;
    border: 1px solid #e8e5de;
    border-radius: 7px;
    background: #fffefa;
  }

  .report-row {
    display: grid;
    grid-template-columns:
      minmax(230px, 2fr)
      minmax(100px, 0.8fr)
      minmax(115px, 1fr)
      minmax(85px, 0.7fr)
      28px;
    align-items: center;
    gap: 12px;
    min-width: 740px;
    min-height: 56px;
    padding: 9px 14px;
    border-bottom: 1px solid #efede7;
  }

  .report-row:last-child {
    border-bottom: 0;
  }

  .table-header {
    min-height: 38px;
    background: #f5f4ef;
    color: #89867e;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.3px;
    text-transform: uppercase;
  }

  /* Clickable Rows */

  .clickable-row {
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .clickable-row:hover {
    background: #f9f8f4;
  }

  .clickable-row:focus-visible {
    outline: 2px solid #34332e;
    outline-offset: -2px;
  }

  /* Report Name */

  .report-name {
    display: flex;
    align-items: center;
    gap: 10px;
    min-width: 0;
  }

  .file-icon {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: center;
    width: 32px;
    height: 34px;
    border: 1px solid #e5e2db;
    border-radius: 5px;
    background: #f4f2ec;
    color: #34332e;
    font-size: 9px;
    font-weight: 700;
  }

  .report-name strong {
    display: block;
    overflow: hidden;
    color: #393832;
    font-size: 12px;
    font-weight: 550;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .report-name small {
    display: block;
    overflow: hidden;
    margin-top: 4px;
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

  .open-indicator {
    color: #a09c92;
    font-size: 15px;
    text-align: center;
    transition: color 0.15s ease;
  }

  .clickable-row:hover .open-indicator {
    color: #34332e;
  }

  /* Empty and Loading States */

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 170px;
    padding: 24px 16px;
    color: #89867d;
    font-size: 12px;
    text-align: center;
  }

  .empty-state strong {
    margin-bottom: 6px;
    color: #393832;
    font-size: 13px;
    font-weight: 600;
  }

  .empty-state p {
    max-width: 420px;
    margin: 0 0 12px;
    line-height: 1.6;
  }

  .error-state {
    color: #a34f42;
  }

  .error-state p {
    max-width: 450px;
  }

  /* Footer and Pagination */

  .list-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-top: 12px;
    color: #89867d;
    font-size: 10px;
  }

  .pagination {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
  }

  .pagination button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 30px;
    min-height: 29px;
    padding: 5px 9px;
    border: 1px solid #e5e2db;
    border-radius: 5px;
    background: #fffefa;
    color: #69665e;
    font-size: 10px;
    transition: all 0.15s ease;
  }

  .pagination button:hover:not(:disabled):not(.current-page) {
    border-color: #dedbd3;
    background: #f4f2ec;
    color: #34332e;
  }

  .pagination button:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .pagination .current-page {
    border-color: #34332e;
    background: #34332e;
    color: #fff;
  }

  /* Accessibility */

  :global(button:focus-visible),
  :global(input:focus-visible),
  :global(select:focus-visible) {
    outline: 2px solid #34332e;
    outline-offset: 2px;
  }

  /* Responsive */

  @media (max-width: 800px) {
    .report-row {
      min-width: 700px;
    }

    .section-header {
      align-items: flex-start;
      flex-direction: column;
    }

    .filters {
      width: 100%;
    }
  }

  @media (max-width: 650px) {
    .reports-page {
      padding: 8px 8px 16px;
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

    .toolbar .filter-button {
      align-self: flex-start;
    }

    .filters {
      align-items: stretch;
      flex-direction: column;
    }

    .filters select {
      width: 100%;
    }

    .list-footer {
      align-items: flex-start;
      flex-direction: column;
    }

    .pagination {
      width: 100%;
    }
  }

  /* Reduced Motion */

  @media (prefers-reduced-motion: reduce) {
    :global(*) {
      scroll-behavior: auto !important;
      transition: none !important;
      animation: none !important;
    }
  }
</style>
