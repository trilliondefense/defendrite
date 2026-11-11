<script lang="ts">
  let platform = $state<'mac' | 'windows'>('windows');
  let search = $state('');

  const shortcuts = [
    {
      category: 'Navigation',
      items: [
        { action: 'Open command palette', mac: ['⌘', 'K'], windows: ['Ctrl', 'K'] },
        { action: 'Toggle sidebar', mac: ['⌘', 'B'], windows: ['Ctrl', 'B'] },
        { action: 'Go to New Chat', mac: ['⌘', 'Shift', 'O'], windows: ['Ctrl', 'Shift', 'O'] },
      ],
    },
    {
      category: 'Command palette',
      items: [
        { action: 'Move to next result', mac: ['↓'], windows: ['↓'] },
        { action: 'Move to previous result', mac: ['↑'], windows: ['↑'] },
        { action: 'Open selected result', mac: ['Return'], windows: ['Enter'] },
        { action: 'Close command palette', mac: ['Esc'], windows: ['Esc'] },
      ],
    },
    {
      category: 'Chat',
      items: [
        { action: 'New line in message', mac: ['Shift', 'Return'], windows: ['Shift', 'Enter'] },
        { action: 'Send message', mac: ['⌘', 'Return'], windows: ['Ctrl', 'Enter'] },
      ],
    },
    {
      category: 'General',
      items: [
        { action: 'Close dialog or menu', mac: ['Esc'], windows: ['Esc'] },
        { action: 'Navigate between controls', mac: ['Tab'], windows: ['Tab'] },
        { action: 'Navigate backwards', mac: ['Shift', 'Tab'], windows: ['Shift', 'Tab'] },
      ],
    },
  ];

  let filtered = $derived(
    shortcuts
      .map((group) => ({
        ...group,
        items: group.items.filter((item) =>
          `${item.action} ${group.category}`.toLowerCase().includes(search.trim().toLowerCase())
        ),
      }))
      .filter((group) => group.items.length > 0)
  );
</script>

<div class="help-page">
  <header class="page-header">
    <h1>Keyboard shortcuts</h1>
    <p>
      Work faster with keyboard shortcuts. Everything you need to navigate your workspace without
      reaching for your mouse.
    </p>
  </header>

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
        bind:value={search}
        placeholder="Search shortcuts..."
        aria-label="Search keyboard shortcuts"
      />
      {#if search}
        <button class="clear-button" onclick={() => (search = '')} aria-label="Clear search">
          ×
        </button>
      {/if}
    </label>

    <div class="platform-switch" aria-label="Operating system">
      <button
        class:active={platform === 'mac'}
        aria-pressed={platform === 'mac'}
        onclick={() => (platform = 'mac')}
      >
        <span>⌘</span> Mac
      </button>
      <button
        class:active={platform === 'windows'}
        aria-pressed={platform === 'windows'}
        onclick={() => (platform = 'windows')}
      >
        <span>⊞</span> Windows
      </button>
    </div>
  </div>

  <div class="shortcut-groups">
    {#each filtered as group (group.category)}
      <section class="shortcut-group">
        <h2>{group.category}</h2>

        <div class="shortcut-list">
          {#each group.items as item (item.action)}
            <div class="shortcut-row">
              <span class="action">{item.action}</span>

              <div class="keys" aria-label={item[platform].join(' + ')}>
                {#each item[platform] as key, i (i)}
                  {#if i > 0}
                    <span class="plus">+</span>
                  {/if}
                  <kbd>{key}</kbd>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </section>
    {:else}
      <div class="empty-state">
        <span>⌕</span>
        <h2>No shortcuts found</h2>
        <p>Try a different search term.</p>
        <button onclick={() => (search = '')}>Clear search</button>
      </div>
    {/each}
  </div>

  <footer class="help-footer">
    <div>
      <strong>Prefer using the keyboard?</strong>
      <p>
        Press
        <kbd>{platform === 'mac' ? '⌘' : 'Ctrl'}</kbd>
        <span class="plus">+</span>
        <kbd>K</kbd>
        anywhere in your workspace to jump to a page.
      </p>
    </div>
  </footer>
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

  .help-page {
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
    max-width: 600px;
    margin-bottom: 22px;
  }

  h1 {
    margin: 0;
    font-family: Georgia, 'Times New Roman', serif;
    font-size: 34px;
    font-weight: 400;
    letter-spacing: -0.8px;
  }

  .page-header p {
    max-width: 510px;
    margin: 8px 0 0;
    color: var(--muted);
    font-size: 12px;
    line-height: 1.7;
  }

  /* Toolbar */

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 10px;
    margin-bottom: 20px;
  }

  /* Search */

  .search-box {
    display: flex;
    align-items: center;
    gap: 9px;
    flex: 1;
    min-width: 200px;
    max-width: 360px;
    min-height: 38px;
    padding: 7px 11px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--panel);
    color: var(--muted);
    transition: border-color 0.15s ease;
  }

  .search-box:focus-within {
    border-color: var(--primary);
  }

  .search-box input {
    flex: 1;
    min-width: 0;
    outline: none;
    border: 0;
    background: transparent;
    color: var(--ink);
    font-size: 12px;
  }

  .search-box input::placeholder {
    color: #aaa69c;
  }

  .clear-button {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2px 5px;
    border: 0;
    border-radius: 4px;
    background: transparent;
    color: var(--muted);
    cursor: pointer;
    font-size: 16px;
  }

  .clear-button:hover {
    background: #f2f0ea;
    color: var(--ink);
  }

  /* Platform switch */

  .platform-switch {
    display: flex;
    align-items: center;
    gap: 3px;
    padding: 3px;
    border: 1px solid var(--border-color);
    border-radius: 7px;
    background: #f3f1eb;
  }

  .platform-switch button {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 7px 10px;
    border: 0;
    border-radius: 5px;
    background: transparent;
    color: var(--muted);
    font-size: 11px;
    transition:
      background 0.15s ease,
      color 0.15s ease;
  }

  .platform-switch button:hover:not(.active) {
    background: #eae7df;
    color: var(--ink);
  }

  .platform-switch button.active {
    background: var(--panel);
    color: var(--primary);
    box-shadow: 0 1px 3px #29282412;
  }

  /* Shortcut groups */

  .shortcut-group {
    margin-bottom: 18px;
  }

  .shortcut-group h2 {
    margin: 0 0 8px;
    color: #77746c;
    font-size: 11px;
    font-weight: 600;
  }

  /* Shortcut list */

  .shortcut-list {
    overflow: hidden;
    border: 1px solid var(--line);
    border-radius: 7px;
    background: var(--panel);
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 14px;
    min-height: 42px;
    padding: 9px 12px;
    transition: background 0.15s ease;
  }

  .shortcut-row + .shortcut-row {
    border-top: 1px solid #efede7;
  }

  .shortcut-row:hover {
    background: #faf9f6;
  }

  .action {
    color: #393832;
    font-size: 12px;
    line-height: 1.5;
  }

  /* Keyboard keys */

  .keys {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 5px;
    flex-wrap: wrap;
  }

  kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 25px;
    min-height: 23px;
    padding: 3px 7px;
    border: 1px solid #e3e0d8;
    border-bottom-width: 2px;
    border-radius: 5px;
    background: #f8f7f3;
    color: #555249;
    font-family: inherit;
    font-size: 10px;
    white-space: nowrap;
  }

  .plus {
    color: #a5a096;
    font-size: 10px;
  }

  /* Empty state */

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 32px 16px;
    border: 1px dashed #dedad0;
    border-radius: 8px;
    background: var(--panel);
    text-align: center;
  }

  .empty-state > span {
    color: var(--primary);
    font-size: 26px;
  }

  .empty-state h2 {
    margin: 9px 0 4px;
    color: var(--ink);
    font-size: 14px;
    font-weight: 600;
  }

  .empty-state p {
    margin: 4px 0 12px;
    color: var(--muted);
    font-size: 11px;
    line-height: 1.6;
  }

  .empty-state button {
    padding: 7px 11px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--panel);
    color: var(--ink);
    font-size: 11px;
    transition:
      background 0.15s ease,
      border-color 0.15s ease;
  }

  .empty-state button:hover {
    border-color: var(--primary);
    background: var(--primary-soft);
    color: var(--primary);
  }

  /* Help footer */

  .help-footer {
    display: flex;
    align-items: flex-start;
    gap: 11px;
    margin-top: 22px;
    padding: 13px 15px;
    border: 1px solid #e8e2d6;
    border-radius: 8px;
    background: #f5f1e8;
  }

  .help-footer strong {
    color: var(--ink);
    font-size: 12px;
    font-weight: 600;
  }

  .help-footer p {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 5px;
    margin: 5px 0 0;
    color: var(--muted);
    font-size: 11px;
    line-height: 1.7;
  }
  /* 
  .help-footer a {
    color: var(--primary);
    text-decoration: none;
  }

  .help-footer a:hover {
    text-decoration: underline;
  } */

  /* Accessibility */

  :global(button:focus-visible),
  :global(input:focus-visible) {
    outline: 2px solid #7653b5;
    outline-offset: 2px;
  }

  :global(.search-box:focus-within) {
    outline: 2px solid #7653b5;
    outline-offset: 2px;
  }

  /* Responsive */

  @media (max-width: 760px) {
    .help-page {
      padding: 16px 14px 20px;
    }
  }

  @media (max-width: 520px) {
    .help-page {
      padding: 12px 12px 18px;
    }

    .page-header {
      margin-bottom: 18px;
    }

    h1 {
      font-size: 29px;
    }

    .toolbar {
      align-items: stretch;
      flex-direction: column;
      gap: 9px;
    }

    .search-box {
      width: 100%;
      max-width: none;
    }

    .platform-switch {
      align-self: flex-start;
    }

    .shortcut-group {
      margin-bottom: 15px;
    }

    .shortcut-row {
      gap: 8px;
      padding: 9px 10px;
    }

    .action {
      font-size: 11px;
    }

    .keys {
      gap: 3px;
    }

    kbd {
      min-width: 22px;
      padding: 3px 5px;
      font-size: 9px;
    }

    .help-footer {
      padding: 11px;
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
