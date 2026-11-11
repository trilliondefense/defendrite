<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { McpTool } from '#lib/utils/types.ts';

  let tools = $state<McpTool[]>([]);
  let isLoading = $state(true);
  let error = $state('');

  let search = $state('');
  let selectedServer = $state('all');
  let sortBy = $state('name-asc');
  let visibleCount = $state(6);

  const pageSize = 6;

  onMount(() => {
    loadTools();
  });

  async function loadTools() {
    isLoading = true;
    error = '';

    try {
      tools = await invoke<McpTool[]>('get_all_tools');
    } catch (err) {
      console.error('Failed to fetch tools:', err);
      error = 'Unable to load tools. Please try again.';
    } finally {
      isLoading = false;
    }
  }

  // Dynamically extract available servers.
  let servers = $derived([...new Set(tools.map((tool) => tool.server).filter(Boolean))].sort());

  // Count tools belonging to each server.
  let serverCounts = $derived.by(() => {
    const counts: Record<string, number> = {};

    for (const tool of tools) {
      counts[tool.server] = (counts[tool.server] || 0) + 1;
    }

    return counts;
  });

  let filteredTools = $derived.by(() => {
    const query = search.trim().toLowerCase();

    let result = tools.filter((tool) => {
      const matchesSearch =
        !query ||
        tool.name.toLowerCase().includes(query) ||
        tool.description.toLowerCase().includes(query) ||
        tool.server.toLowerCase().includes(query);

      const matchesServer = selectedServer === 'all' || tool.server === selectedServer;

      return matchesSearch && matchesServer;
    });

    result = [...result].sort((a, b) => {
      switch (sortBy) {
        case 'name-desc':
          return b.name.localeCompare(a.name);

        case 'server':
          return a.server.localeCompare(b.server) || a.name.localeCompare(b.name);

        case 'name-asc':
        default:
          return a.name.localeCompare(b.name);
      }
    });

    return result;
  });

  let visibleTools = $derived(filteredTools.slice(0, visibleCount));

  let hasMore = $derived(visibleCount < filteredTools.length);

  let remainingCount = $derived(filteredTools.length - visibleCount);

  // Reset visible cards when filters change.
  $effect(() => {
    search;
    selectedServer;
    sortBy;

    visibleCount = pageSize;
  });

  function loadMore() {
    visibleCount += pageSize;
  }

  function resetFilters() {
    search = '';
    selectedServer = 'all';
    sortBy = 'name-asc';
    visibleCount = pageSize;
  }

  function getInputCount(schema: unknown): number {
    if (!schema || typeof schema !== 'object') return 0;

    const inputSchema = schema as {
      properties?: Record<string, unknown>;
    };

    return Object.keys(inputSchema.properties || {}).length;
  }
</script>

<div class="tools-page">
  <!-- Page Header -->
  <header class="page-header">
    <div>
      <h1>Tools</h1>
      <p>Browse and access tools generated from your forensic investigations.</p>
    </div>
  </header>

  <!-- Overview -->
  <!-- Search and Filters -->
  <section class="tools-toolbar">
    <div class="search-box">
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
        placeholder="Search tools, descriptions or servers..."
        aria-label="Search tools"
        bind:value={search}
      />
    </div>

    <div class="filter-controls">
      <select bind:value={selectedServer} aria-label="Filter by server">
        <option value="all">All servers ({tools.length})</option>

        {#each servers as server}
          <option value={server}>
            {server} ({serverCounts[server] || 0})
          </option>
        {/each}
      </select>

      <select bind:value={sortBy} aria-label="Sort tools">
        <option value="name-asc">Name: A to Z</option>
        <option value="name-desc">Name: Z to A</option>
        <option value="server">Server</option>
      </select>
    </div>
  </section>

  <!-- Tool Library -->
  <section class="tool-section">
    <div class="section-header">
      <div>
        <h2>All tools</h2>
        <p>
          {filteredTools.length} available tools
        </p>
      </div>
      <button class="filter-button" onclick={loadTools} disabled={isLoading}> ↻ Refresh </button>
    </div>

    {#if isLoading}
      <div class="tools-grid">
        {#each Array.from({ length: 6 }) as _, i}
          <div class="tool-card skeleton-card" aria-hidden="true">
            <div class="skeleton-icon"></div>
            <div class="skeleton-line"></div>
            <div class="skeleton-line short"></div>
          </div>
        {/each}
      </div>
    {:else if error}
      <div class="empty-state error-state">
        <h3>Something went wrong</h3>
        <p>{error}</p>
        <button class="filter-button" onclick={loadTools}> Try again </button>
      </div>
    {:else if filteredTools.length === 0}
      <div class="empty-state">
        <div class="empty-icon">
          <svg
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
          >
        </div>

        <h3>
          {search || selectedServer !== 'all' ? 'No matching tools' : 'No tools available yet'}
        </h3>

        <p>
          {search || selectedServer !== 'all'
            ? 'Try changing your search or filters.'
            : 'Tools generated from your investigations will appear here.'}
        </p>

        {#if search || selectedServer !== 'all'}
          <button class="filter-button" onclick={resetFilters}> Clear filters </button>
        {/if}
      </div>
    {:else}
      <!-- Responsive Card Grid -->
      <div class="tools-grid">
        {#each visibleTools as tool (tool.server + ':' + tool.name)}
          <article class="tool-card">
            <div class="tool-card-header">
              <div class="tool-icon">
                <span
                  ><svg
                    fill="#000000"
                    height="24px"
                    width="24px"
                    version="1.1"
                    id="XMLID_191_"
                    xmlns="http://www.w3.org/2000/svg"
                    xmlns:xlink="http://www.w3.org/1999/xlink"
                    viewBox="0 0 24 24"
                    enable-background="new 0 0 24 24"
                    xml:space="preserve"
                    ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                      id="SVGRepo_tracerCarrier"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                    ></g><g id="SVGRepo_iconCarrier">
                      <g id="tools">
                        <g>
                          <path
                            d="M3.9,22L0,18.1l9.8-9.7L9.7,8.3C7.9,6.5,5,5.9,5,5.8L3.5,5.4l1.2-1.2c1.5-1.5,3.2-2.3,5-2.3c3,0,5.2,2.3,5.3,2.4l3.5,3.5 l2-0.4L24,11l-7.2,7.1l-3.6-3.6l0.6-1.9l-0.3-0.3L3.9,22z M2.5,18.2l1.3,1.3l8.5-8.5L11,9.7L2.5,18.2z M15.3,14l1.6,1.6l4.6-4.6 l-1.6-1.6l-2,0.6l-4.3-4.2c0,0-1.8-1.9-4-1.9c-0.8,0-1.7,0.3-2.5,0.8C8.3,5.1,9.8,5.9,11,7.1l0,0l4.9,5L15.3,14z"
                          ></path>
                        </g>
                      </g>
                    </g></svg
                  ></span
                >
              </div>

              <span class="server-badge">
                {tool.server}
              </span>
            </div>

            <div class="tool-card-content">
              <h3>{tool.name}</h3>

              <p>
                {tool.description || 'No description available.'}
              </p>
            </div>

            <div class="tool-card-footer">
              <span class="input-count">
                {getInputCount(tool.input_schema)}
                input parameters
              </span>
            </div>
          </article>
        {/each}
      </div>

      <!-- Load More -->
      {#if hasMore}
        <div class="load-more-container">
          <button class="load-more-button" onclick={loadMore}>
            Load more tools
            <span>({Math.min(pageSize, remainingCount)} more)</span>
          </button>

          <p>
            Showing {visibleTools.length} of {filteredTools.length} tools
          </p>
        </div>
      {:else if filteredTools.length > pageSize}
        <div class="load-more-container">
          <p>You've reached the end of the tools list.</p>
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

  .tools-page {
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

  /* Toolbar */

  .tools-toolbar {
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

  .filter-controls {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .filter-controls select {
    min-height: 36px;
    max-width: 220px;
    padding: 7px 10px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    outline: none;
    background: #fffefa;
    color: #555249;
    font-size: 11px;
  }

  .filter-controls select:focus {
    border-color: #34332e;
  }

  /* Tool Section */

  .tool-section {
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

  /* Tool Grid */

  .tools-grid {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 12px;
    width: 100%;
  }

  /* Tool Card */

  .tool-card {
    display: flex;
    flex-direction: column;
    min-width: 0;
    min-height: 190px;
    padding: 15px;
    border: 1px solid #e8e5de;
    border-radius: 7px;
    background: #fffefa;
    transition:
      border-color 0.15s ease,
      background 0.15s ease,
      transform 0.15s ease;
  }

  .tool-card:hover {
    border-color: #d5d1c7;
    background: #fff;
    transform: translateY(-2px);
  }

  .tool-card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    margin-bottom: 15px;
  }

  .tool-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 35px;
    height: 35px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background: #f4f2ec;
    color: #34332e;
  }

  .tool-icon span {
    font-size: 16px;
  }

  .server-badge {
    display: inline-block;
    overflow: hidden;
    max-width: 65%;
    padding: 5px 8px;
    border: 1px solid #eae7df;
    border-radius: 4px;
    background: #f8f7f2;
    color: #77736a;
    font-size: 9px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  /* Tool Card Content */

  .tool-card-content {
    flex: 1;
    min-width: 0;
  }

  .tool-card-content h3 {
    overflow: hidden;
    margin: 0 0 7px;
    color: #393832;
    font-size: 13px;
    font-weight: 600;
    line-height: 1.5;
    text-overflow: ellipsis;
    overflow-wrap: anywhere;
  }

  .tool-card-content p {
    display: -webkit-box;
    overflow: hidden;
    margin: 0;
    color: #89867d;
    font-size: 11px;
    line-height: 1.7;
    overflow-wrap: anywhere;
    -webkit-box-orient: vertical;
    -webkit-line-clamp: 3;
    line-clamp: 3;
  }

  /* Tool Card Footer */

  .tool-card-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    margin-top: 15px;
    padding-top: 11px;
    border-top: 1px solid #efede7;
  }

  .input-count {
    color: #89867d;
    font-size: 10px;
    font-variant-numeric: tabular-nums;
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
    gap: 14px;
    pointer-events: none;
  }

  .skeleton-icon,
  .skeleton-line {
    background: linear-gradient(90deg, #f0eee8 25%, #f7f6f1 50%, #f0eee8 75%);
    background-size: 200% 100%;
    animation: skeleton-loading 1.5s ease infinite;
  }

  .skeleton-icon {
    width: 35px;
    height: 35px;
    border-radius: 6px;
  }

  .skeleton-line {
    width: 75%;
    height: 11px;
    border-radius: 4px;
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

  .empty-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    margin-bottom: 13px;
    border: 1px solid #e8e5de;
    border-radius: 50%;
    background: #f4f2ec;
    color: #89867d;
    font-size: 20px;
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
    .tools-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }

  @media (max-width: 800px) {
    .tools-toolbar {
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
    .tools-page {
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

    .tools-grid {
      grid-template-columns: 1fr;
      gap: 9px;
    }

    .tool-card {
      min-height: 175px;
      padding: 13px;
    }

    .section-header {
      align-items: flex-start;
    }

    .filter-controls {
      align-items: stretch;
      flex-direction: column;
    }

    .filter-controls select {
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
