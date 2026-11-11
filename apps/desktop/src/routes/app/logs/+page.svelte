<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  interface LogEntry {
    id: string;
    timestamp: string;
    level: string;
    source: string;
    message: string;
    details?: string;
  }

  let logs = $state<LogEntry[]>([]);
  let isLoading = $state(true);
  let error = $state('');

  let search = $state('');
  let selectedLevel = $state('all');
  let selectedSource = $state('all');
  let sortBy = $state('newest');
  let visibleCount = $state(6);
  let expandedLog = $state<string | null>(null);

  const pageSize = 6;

  onMount(() => {
    loadLogs();
  });

  async function loadLogs() {
    isLoading = true;
    error = '';

    try {
      // Replace with your actual Tauri command.
      //   logs = await invoke<LogEntry[]>('get_all_logs');
    } catch (err) {
      console.error('Failed to fetch logs:', err);
      error = 'Unable to load logs. Please try again.';
    } finally {
      isLoading = false;
    }
  }

  // Available log levels.
  let levels = $derived([...new Set(logs.map((log) => log.level).filter(Boolean))].sort());

  // Available log sources.
  let sources = $derived([...new Set(logs.map((log) => log.source).filter(Boolean))].sort());

  // Log counts by level.
  let levelCounts = $derived.by(() => {
    const counts: Record<string, number> = {};

    for (const log of logs) {
      const level = log.level.toLowerCase();
      counts[level] = (counts[level] || 0) + 1;
    }

    return counts;
  });

  // Search and filtering.
  let filteredLogs = $derived.by(() => {
    const query = search.trim().toLowerCase();

    let result = logs.filter((log) => {
      const matchesSearch =
        !query ||
        log.message.toLowerCase().includes(query) ||
        log.level.toLowerCase().includes(query) ||
        log.source.toLowerCase().includes(query) ||
        (log.details || '').toLowerCase().includes(query);

      const matchesLevel =
        selectedLevel === 'all' || log.level.toLowerCase() === selectedLevel.toLowerCase();

      const matchesSource = selectedSource === 'all' || log.source === selectedSource;

      return matchesSearch && matchesLevel && matchesSource;
    });

    result = [...result].sort((a, b) => {
      const dateA = new Date(a.timestamp).getTime();
      const dateB = new Date(b.timestamp).getTime();

      return sortBy === 'oldest' ? dateA - dateB : dateB - dateA;
    });

    return result;
  });

  // Visible logs.
  let visibleLogs = $derived(filteredLogs.slice(0, visibleCount));

  let hasMore = $derived(visibleCount < filteredLogs.length);

  let remainingCount = $derived(filteredLogs.length - visibleCount);

  // Summary statistics.
  let totalErrors = $derived(levelCounts['error'] || 0);

  let totalWarnings = $derived(levelCounts['warning'] || 0);

  let totalInfo = $derived(levelCounts['info'] || 0);

  // Reset visible count when filters change.
  $effect(() => {
    search;
    selectedLevel;
    selectedSource;
    sortBy;

    visibleCount = pageSize;
    expandedLog = null;
  });

  function loadMore() {
    visibleCount += pageSize;
  }

  function toggleLog(id: string) {
    expandedLog = expandedLog === id ? null : id;
  }

  function resetFilters() {
    search = '';
    selectedLevel = 'all';
    selectedSource = 'all';
    sortBy = 'newest';
    visibleCount = pageSize;
    expandedLog = null;
  }

  function formatTimestamp(timestamp: string) {
    const date = new Date(timestamp);

    if (Number.isNaN(date.getTime())) {
      return timestamp;
    }

    return date.toLocaleString();
  }

  function getLevelClass(level: string) {
    return level.toLowerCase().replace(/\s+/g, '-');
  }
</script>

<div class="logs-page">
  <!-- Page Header -->
  <header class="page-header">
    <div>
      <h1>Logs</h1>
      <p>Monitor application activity, events, warnings, and errors.</p>
    </div>

    <button class="filter-button" onclick={loadLogs} disabled={isLoading}> ↻ Refresh </button>
  </header>

  <!-- Overview -->
  <section class="logs-overview">
    <div class="overview-item">
      <span>Total logs</span>
      <strong>{logs.length}</strong>
    </div>

    <div class="overview-item">
      <span>Errors</span>
      <strong>{totalErrors}</strong>
    </div>

    <div class="overview-item">
      <span>Warnings</span>
      <strong>{totalWarnings}</strong>
    </div>

    <div class="overview-item">
      <span>Info</span>
      <strong>{totalInfo}</strong>
    </div>
  </section>

  <!-- Search and Filters -->
  <section class="logs-toolbar">
    <div class="search-box">
      <span>⌕</span>

      <input
        type="search"
        placeholder="Search logs, messages or sources..."
        aria-label="Search logs"
        bind:value={search}
      />
    </div>

    <div class="filter-controls">
      <select bind:value={selectedLevel} aria-label="Filter by log level">
        <option value="all">All levels</option>

        {#each levels as level}
          <option value={level.toLowerCase()}>
            {level} ({levelCounts[level.toLowerCase()] || 0})
          </option>
        {/each}
      </select>

      <select bind:value={selectedSource} aria-label="Filter by source">
        <option value="all">All sources</option>

        {#each sources as source}
          <option value={source}>
            {source}
          </option>
        {/each}
      </select>

      <select bind:value={sortBy} aria-label="Sort logs">
        <option value="newest">Newest first</option>
        <option value="oldest">Oldest first</option>
      </select>
    </div>
  </section>

  <!-- Logs List -->
  <section class="logs-section">
    <div class="section-header">
      <div>
        <h2>Activity logs</h2>
        <p>{filteredLogs.length} matching entries</p>
      </div>
    </div>

    {#if isLoading}
      <div class="logs-list">
        {#each Array.from({ length: 6 }) as _, i}
          <div class="log-card skeleton-card" aria-hidden="true">
            <div class="skeleton-line"></div>
            <div class="skeleton-line short"></div>
          </div>
        {/each}
      </div>
    {:else if error}
      <div class="empty-state error-state">
        <h3>Unable to load logs</h3>
        <p>{error}</p>

        <button class="filter-button" onclick={loadLogs}> Try again </button>
      </div>
    {:else if filteredLogs.length === 0}
      <div class="empty-state">
        <h3>
          {search || selectedLevel !== 'all' || selectedSource !== 'all'
            ? 'No matching logs'
            : 'No logs available'}
        </h3>

        <p>
          {search || selectedLevel !== 'all' || selectedSource !== 'all'
            ? 'Try changing your search or filters.'
            : 'Application activity will appear here.'}
        </p>

        {#if search || selectedLevel !== 'all' || selectedSource !== 'all'}
          <button class="filter-button" onclick={resetFilters}> Clear filters </button>
        {/if}
      </div>
    {:else}
      <div class="logs-list">
        {#each visibleLogs as log (log.id)}
          <article class="log-card">
            <div class="log-card-main">
              <div class="log-card-header">
                <span class="log-level" class:expanded={expandedLog === log.id}>
                  {log.level}
                </span>

                <span class="log-source">
                  {log.source}
                </span>

                <time class="log-timestamp">
                  {formatTimestamp(log.timestamp)}
                </time>
              </div>

              <div class="log-message">
                {log.message}
              </div>

              <div class="log-card-footer">
                <button
                  class="expand-button"
                  aria-expanded={expandedLog === log.id}
                  onclick={() => toggleLog(log.id)}
                >
                  {expandedLog === log.id ? 'Hide details' : 'View details'}
                  <span>
                    {expandedLog === log.id ? '−' : '+'}
                  </span>
                </button>
              </div>

              {#if expandedLog === log.id}
                <div class="log-details">
                  <div>
                    <span>Log ID</span>
                    <strong>{log.id}</strong>
                  </div>

                  <div>
                    <span>Level</span>
                    <strong>{log.level}</strong>
                  </div>

                  <div>
                    <span>Source</span>
                    <strong>{log.source}</strong>
                  </div>

                  <div>
                    <span>Timestamp</span>
                    <strong>{formatTimestamp(log.timestamp)}</strong>
                  </div>

                  {#if log.details}
                    <div>
                      <span>Additional details</span>
                      <pre>{log.details}</pre>
                    </div>
                  {/if}
                </div>
              {/if}
            </div>
          </article>
        {/each}
      </div>

      <!-- Load More -->
      {#if hasMore}
        <div class="load-more-container">
          <button class="load-more-button" onclick={loadMore}>
            Load more logs
            <span>
              ({Math.min(pageSize, remainingCount)} more)
            </span>
          </button>

          <p>
            Showing {visibleLogs.length} of {filteredLogs.length} logs
          </p>
        </div>
      {:else if filteredLogs.length > pageSize}
        <div class="load-more-container">
          <p>You've reached the end of the logs.</p>
        </div>
      {/if}
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

  .logs-page {
    width: 100%;
    max-width: 1440px;
    margin: 0 auto;
    padding: 12px 16px 24px;
    color: #302f2a;
  }

  /* Header */

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 22px;
    padding-bottom: 14px;
    border-bottom: 1px solid #eae7df;
  }

  .page-header h1 {
    margin: 0;
    font-family: Georgia, 'Times New Roman', serif;
    font-size: 34px;
    font-weight: 400;
    letter-spacing: -0.8px;
  }

  .page-header p {
    max-width: 560px;
    margin: 8px 0 0;
    color: #89867e;
    font-size: 12px;
    line-height: 1.6;
  }

  /* Common Buttons */

  .filter-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    gap: 6px;
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

  /* Overview */

  .logs-overview {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 12px;
    margin-bottom: 22px;
  }

  .overview-item {
    display: flex;
    flex-direction: column;
    gap: 8px;
    min-width: 0;
    padding: 14px 16px;
    border: 1px solid #e8e5de;
    border-radius: 7px;
    background: #fffefa;
  }

  .overview-item span {
    color: #89867e;
    font-size: 10px;
    font-weight: 500;
  }

  .overview-item strong {
    color: #393832;
    font-family: Georgia, 'Times New Roman', serif;
    font-size: 25px;
    font-weight: 400;
    font-variant-numeric: tabular-nums;
  }

  /* Toolbar */

  .logs-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 22px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 9px;
    width: 360px;
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
    flex-shrink: 0;
    font-size: 17px;
  }

  .search-box input {
    width: 100%;
    min-width: 0;
    padding: 0;
    border: 0;
    outline: none;
    background: transparent;
    color: #302f2a;
    font-size: 12px;
  }

  .search-box input::placeholder {
    color: #a09c92;
  }

  /* Filters */

  .filter-controls {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .filter-controls select {
    appearance: none;
    -webkit-appearance: none;
    min-height: 36px;
    max-width: 220px;
    padding: 7px 32px 7px 11px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background-color: #fffefa;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%2389867e' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpath d='m6 9 6 6 6-6'/%3E%3C/svg%3E");
    background-repeat: no-repeat;
    background-position: right 11px center;
    color: #555249;
    font-size: 11px;
    cursor: pointer;
    outline: none;
    transition: border-color 0.15s ease;
  }

  .filter-controls select:hover {
    border-color: #c9c5ba;
  }

  .filter-controls select:focus {
    border-color: #34332e;
    box-shadow: 0 0 0 2px rgba(52, 51, 46, 0.06);
  }

  .filter-controls select option {
    background: #fffefa;
    color: #393832;
  }

  /* Logs Section */

  .logs-section {
    width: 100%;
    min-width: 0;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    margin-bottom: 13px;
  }

  .section-header h2 {
    margin: 0;
    color: #393832;
    font-size: 14px;
    font-weight: 600;
  }

  .section-header p {
    margin: 5px 0 0;
    color: #89867e;
    font-size: 11px;
    line-height: 1.5;
  }

  /* Logs List */

  .logs-list {
    display: flex;
    flex-direction: column;
    gap: 9px;
    width: 100%;
  }

  /* Log Card */

  .log-card {
    min-width: 0;
    padding: 14px 15px;
    border: 1px solid #e8e5de;
    border-radius: 7px;
    background: #fffefa;
    transition:
      border-color 0.15s ease,
      background 0.15s ease;
  }

  .log-card:hover {
    border-color: #d5d1c7;
    background: #fff;
  }

  .log-card-main {
    min-width: 0;
  }

  /* Log Header */

  .log-card-header {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 9px;
    margin-bottom: 11px;
  }

  /* Log Levels */

  .log-level {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-height: 21px;
    padding: 3px 8px;
    border: 1px solid #e5e2db;
    border-radius: 4px;
    background: #f4f2ec;
    color: #69665e;
    font-size: 9px;
    font-weight: 600;
    letter-spacing: 0.3px;
    text-transform: uppercase;
  }

  .log-level:where([class~='error']) {
    border-color: #ecd8d3;
    background: #fbf0ed;
    color: #a34f42;
  }

  .log-level:where([class~='warning']) {
    border-color: #eee3c9;
    background: #faf5e8;
    color: #987338;
  }

  .log-level:where([class~='info']) {
    border-color: #dce5e5;
    background: #eff5f4;
    color: #527777;
  }

  .log-level:where([class~='debug']) {
    border-color: #e5e2db;
    background: #f4f2ec;
    color: #77736a;
  }

  .log-level:where([class~='success']) {
    border-color: #dce7d9;
    background: #f0f5ed;
    color: #58774f;
  }

  /* Source and Timestamp */

  .log-source {
    overflow: hidden;
    max-width: 45%;
    color: #77736a;
    font-size: 10px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .log-timestamp {
    margin-left: auto;
    color: #a09c92;
    font-size: 10px;
    font-variant-numeric: tabular-nums;
  }

  /* Log Message */

  .log-message {
    color: #393832;
    font-size: 12px;
    line-height: 1.7;
    overflow-wrap: anywhere;
  }

  /* Card Footer */

  .log-card-footer {
    display: flex;
    justify-content: flex-end;
    margin-top: 10px;
  }

  .expand-button {
    display: inline-flex;
    align-items: center;
    gap: 7px;
    padding: 4px 0;
    border: 0;
    background: transparent;
    color: #89867d;
    font-size: 10px;
    transition: color 0.15s ease;
  }

  .expand-button:hover {
    color: #34332e;
  }

  .expand-button span {
    font-size: 14px;
  }

  /* Expanded Log Details */

  .log-details {
    display: flex;
    flex-direction: column;
    gap: 12px;
    margin-top: 13px;
    padding: 13px;
    border: 1px solid #efede7;
    border-radius: 5px;
    background: #f8f7f2;
  }

  .log-details > div {
    display: flex;
    flex-direction: column;
    gap: 5px;
    min-width: 0;
  }

  .log-details span {
    color: #89867d;
    font-size: 10px;
  }

  .log-details strong {
    color: #393832;
    font-size: 11px;
    font-weight: 500;
    overflow-wrap: anywhere;
  }

  .log-details pre {
    max-width: 100%;
    margin: 0;
    padding: 10px;
    overflow-x: auto;
    border: 1px solid #e8e5de;
    border-radius: 4px;
    background: #fffefa;
    color: #555249;
    font-family: 'SFMono-Regular', Consolas, monospace;
    font-size: 10px;
    line-height: 1.7;
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }

  /* Load More */

  .load-more-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 9px;
    margin-top: 24px;
    padding-bottom: 8px;
    text-align: center;
  }

  .load-more-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 7px;
    min-height: 37px;
    padding: 8px 16px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background: #fffefa;
    color: #393832;
    font-size: 11px;
    font-weight: 550;
    transition: all 0.15s ease;
  }

  .load-more-button:hover {
    border-color: #34332e;
    background: #f4f2ec;
  }

  .load-more-button span {
    color: #89867d;
    font-size: 10px;
    font-weight: 400;
  }

  .load-more-container p {
    margin: 0;
    color: #a09c92;
    font-size: 10px;
  }

  /* Loading Skeleton */

  .skeleton-card {
    display: flex;
    flex-direction: column;
    gap: 14px;
    min-height: 90px;
    pointer-events: none;
  }

  .skeleton-line {
    width: 75%;
    height: 11px;
    border-radius: 4px;
    background: linear-gradient(90deg, #f0eee8 25%, #f7f6f1 50%, #f0eee8 75%);
    background-size: 200% 100%;
    animation: skeleton-loading 1.5s ease infinite;
  }

  .skeleton-line.short {
    width: 45%;
  }

  @keyframes skeleton-loading {
    from {
      background-position: 100% 0;
    }

    to {
      background-position: -100% 0;
    }
  }

  /* Empty and Error States */

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 210px;
    padding: 30px 16px;
    border: 1px solid #e8e5de;
    border-radius: 7px;
    background: #fffefa;
    color: #89867d;
    font-size: 12px;
    text-align: center;
  }

  .empty-state h3 {
    margin: 0 0 7px;
    color: #393832;
    font-size: 14px;
    font-weight: 600;
  }

  .empty-state p {
    max-width: 420px;
    margin: 0 0 14px;
    line-height: 1.7;
  }

  .error-state {
    color: #a34f42;
  }

  .error-state h3 {
    color: #a34f42;
  }

  /* Accessibility */

  :global(button:focus-visible),
  :global(input:focus-visible),
  :global(select:focus-visible) {
    outline: 2px solid #34332e;
    outline-offset: 2px;
  }

  /* Responsive */

  @media (max-width: 1050px) {
    .logs-overview {
      gap: 9px;
    }
  }

  @media (max-width: 800px) {
    .logs-toolbar {
      align-items: stretch;
      flex-direction: column;
    }

    .search-box {
      width: 100%;
    }

    .filter-controls {
      width: 100%;
    }

    .filter-controls select {
      flex: 1;
      min-width: 0;
      max-width: none;
    }
  }

  @media (max-width: 650px) {
    .logs-page {
      padding: 8px 8px 16px;
    }

    .page-header {
      align-items: flex-start;
    }

    .page-header h1 {
      font-size: 29px;
    }

    .page-header p {
      font-size: 11px;
    }

    .logs-overview {
      grid-template-columns: repeat(2, minmax(0, 1fr));
      gap: 7px;
      margin-bottom: 18px;
    }

    .overview-item {
      gap: 7px;
      padding: 11px 10px;
    }

    .overview-item span {
      font-size: 9px;
    }

    .overview-item strong {
      font-size: 21px;
    }

    .filter-controls {
      align-items: stretch;
      flex-direction: column;
    }

    .filter-controls select {
      width: 100%;
    }

    .log-card {
      padding: 12px;
    }

    .log-card-header {
      gap: 7px;
    }

    .log-source {
      max-width: 100%;
    }

    .log-timestamp {
      width: 100%;
      margin-left: 0;
    }

    .log-message {
      font-size: 11px;
    }

    .log-details {
      padding: 10px;
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
