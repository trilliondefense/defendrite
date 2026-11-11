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
      <span>⌕</span>
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
    color: #292824;
    font-family:
      Inter,
      -apple-system,
      BlinkMacSystemFont,
      'Segoe UI',
      sans-serif;
  }

  .reports-page {
    width: 100%;
    margin: 0 auto;
    padding: 32px;
  }

  .page-header {
    margin-bottom: 28px;
  }

  h1 {
    margin: 0;
    font-family: Georgia, 'Times New Roman', serif;
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
    margin-bottom: 24px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 10px;
    width: 380px;
    max-width: 100%;
    padding: 10px 12px;
    border: 1px solid #e5e2db;
    border-radius: 7px;
    background: #fffefa;
    color: #89867d;
    transition: border-color 0.2s;
  }

  .search-box:focus-within {
    border-color: #b4775c;
  }

  .search-box span {
    font-size: 18px;
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

  button,
  select {
    font: inherit;
    cursor: pointer;
  }

  .filter-button {
    padding: 9px 12px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background: #fffefa;
    color: #555249;
    font-size: 11px;
    transition: background 0.2s;
  }

  .filter-button:hover {
    background: #f3f1eb;
  }

  .filter-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Report section */

  .report-section {
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

  .filters {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .filters select {
    padding: 8px 11px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    outline: none;
    background: #fffefa;
    color: #555249;
    font-size: 11px;
  }

  .filters select:focus {
    border-color: #b4775c;
  }

  /* Report list */

  .report-list {
    overflow-x: auto;
    border: 1px solid #e8e5de;
    border-radius: 9px;
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
    gap: 14px;
    min-width: 760px;
    padding: 14px 18px;
    border-bottom: 1px solid #efede7;
  }

  .report-row:last-child {
    border-bottom: 0;
  }

  .table-header {
    min-height: 42px;
    background: #f8f7f3;
    color: #858177;
    font-size: 10px;
    font-weight: 600;
  }

  .clickable-row {
    cursor: pointer;
    transition: background 0.15s ease;
  }

  .clickable-row:hover {
    background: #f9f7f1;
  }

  .clickable-row:focus-visible {
    outline: 2px solid #b4775c;
    outline-offset: -2px;
  }

  .report-name {
    display: flex;
    align-items: center;
    gap: 11px;
    min-width: 0;
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
    margin-top: 5px;
    color: #89867d;
    font-size: 10px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .case-id,
  .date {
    color: #69665e;
    font-size: 11px;
  }

  .open-indicator {
    color: #aaa397;
    font-size: 15px;
    text-align: center;
    transition: color 0.2s;
  }

  .clickable-row:hover .open-indicator {
    color: #b4775c;
  }

  /* Empty and loading states */

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 180px;
    padding: 30px 20px;
    color: #89867d;
    font-size: 12px;
    text-align: center;
  }

  .empty-state strong {
    margin-bottom: 7px;
    color: #393832;
    font-size: 13px;
    font-weight: 600;
  }

  .empty-state p {
    margin: 0 0 12px;
    line-height: 1.6;
  }

  .error-state {
    color: #a34f42;
  }

  .error-state p {
    max-width: 450px;
  }

  /* Footer and pagination */

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
    flex-wrap: wrap;
    gap: 5px;
  }

  .pagination button {
    min-width: 30px;
    padding: 7px 9px;
    border: 1px solid #e5e2db;
    border-radius: 5px;
    background: #fffefa;
    color: #69665e;
    font-size: 10px;
    transition: background 0.15s;
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
      padding: 24px 16px;
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
</style>
