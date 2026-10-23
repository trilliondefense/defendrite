
<script lang="ts">
  let platform = $state<"mac" | "windows">("windows");
  let search = $state("");

  const shortcuts = [
    {
      category: "Navigation",
      items: [
        { action: "Open command palette", mac: ["⌘", "K"], windows: ["Ctrl", "K"] },
        { action: "Toggle sidebar", mac: ["⌘", "B"], windows: ["Ctrl", "B"] },
        { action: "Go to New Chat", mac: ["⌘", "Shift", "O"], windows: ["Ctrl", "Shift", "O"] },
      ],
    },
    {
      category: "Command palette",
      items: [
        { action: "Move to next result", mac: ["↓"], windows: ["↓"] },
        { action: "Move to previous result", mac: ["↑"], windows: ["↑"] },
        { action: "Open selected result", mac: ["Return"], windows: ["Enter"] },
        { action: "Close command palette", mac: ["Esc"], windows: ["Esc"] },
      ],
    },
    {
      category: "Chat",
      items: [
        { action: "New line in message", mac: ["Shift", "Return"], windows: ["Shift", "Enter"] },
        { action: "Send message", mac: ["⌘", "Return"], windows: ["Ctrl", "Enter"] },
      ],
    },
    {
      category: "General",
      items: [
        { action: "Close dialog or menu", mac: ["Esc"], windows: ["Esc"] },
        { action: "Navigate between controls", mac: ["Tab"], windows: ["Tab"] },
        { action: "Navigate backwards", mac: ["Shift", "Tab"], windows: ["Shift", "Tab"] },
      ],
    },
  ];

  let filtered = $derived(
    shortcuts
      .map((group) => ({
        ...group,
        items: group.items.filter((item) =>
          `${item.action} ${group.category}`
            .toLowerCase()
            .includes(search.trim().toLowerCase())
        ),
      }))
      .filter((group) => group.items.length > 0)
  );
</script>

<svelte:head>
  <title>Keyboard shortcuts · Workspace Help</title>
  <meta
    name="description"
    content="Keyboard shortcuts for navigating and using Workspace on Mac and Windows."
  />
</svelte:head>

<div class="help-page">
  <header class="page-header">
    <div class="eyebrow">HELP CENTER</div>
    <h1>Keyboard shortcuts</h1>
    <p>
      Work faster with keyboard shortcuts. Everything you need to navigate
      your workspace without reaching for your mouse.
    </p>
  </header>

  <div class="toolbar">
    <label class="search-box">
      <span aria-hidden="true">⌕</span>
      <input
        bind:value={search}
        placeholder="Search shortcuts..."
        aria-label="Search keyboard shortcuts"
      />
      {#if search}
        <button
          class="clear-button"
          onclick={() => (search = "")}
          aria-label="Clear search"
        >
          ×
        </button>
      {/if}
    </label>

    <div class="platform-switch" aria-label="Operating system">
      <button
        class:active={platform === "mac"}
        aria-pressed={platform === "mac"}
        onclick={() => (platform = "mac")}
      >
        <span>⌘</span> Mac
      </button>
      <button
        class:active={platform === "windows"}
        aria-pressed={platform === "windows"}
        onclick={() => (platform = "windows")}
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

              <div class="keys" aria-label={item[platform].join(" + ")}>
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
        <button onclick={() => (search = "")}>Clear search</button>
      </div>
    {/each}
  </div>

  <footer class="help-footer">

    <div>
      <strong>Prefer using the keyboard?</strong>
      <p>
        Press
        <kbd>{platform === "mac" ? "⌘" : "Ctrl"}</kbd>
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

  .help-page {
    width: 100%;
    max-width: 820px;
    margin: 0 auto;
    padding: 20px 12px 48px;
    color: #292824;
  }

  .page-header {
    max-width: 570px;
    margin-bottom: 32px;
  }

  .eyebrow {
    margin-bottom: 12px;
    color: #b4775c;
    font-size: 10px;
    font-weight: 700;
    letter-spacing: 1.5px;
  }

  h1 {
    margin: 0;
    font-family: Georgia, "Times New Roman", serif;
    font-size: clamp(28px, 4vw, 38px);
    font-weight: 400;
    letter-spacing: -1px;
  }

  .page-header p {
    max-width: 510px;
    margin: 12px 0 0;
    color: #77746c;
    font-size: 13px;
    line-height: 1.8;
  }

  .toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 12px;
    margin-bottom: 28px;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 10px;
    flex: 1;
    min-width: 200px;
    max-width: 360px;
    padding: 10px 12px;
    border: 1px solid #e5e2db;
    border-radius: 8px;
    background: #fffefa;
    color: #89867d;
  }

  .search-box input {
    flex: 1;
    min-width: 0;
    outline: none;
    border: none;
    background: transparent;
    color: #292824;
    font-size: 12px;
  }

  .clear-button {
    border: 0;
    background: transparent;
    color: #77746c;
    cursor: pointer;
    font-size: 17px;
  }

  .platform-switch {
    display: flex;
    gap: 3px;
    padding: 3px;
    border: 1px solid #e5e2db;
    border-radius: 9px;
    background: #f3f1eb;
  }

  .platform-switch button {
    display: flex;
    align-items: center;
    gap: 7px;
    padding: 8px 11px;
    border: 0;
    border-radius: 6px;
    background: transparent;
    color: #77746c;
    font-size: 11px;
    cursor: pointer;
  }

  .platform-switch button.active {
    background: #fffefa;
    color: #302f2a;
    box-shadow: 0 1px 3px #29282412;
  }

  .shortcut-group {
    margin-bottom: 22px;
  }

  .shortcut-group h2 {
    margin: 0 0 9px;
    color: #77746c;
    font-size: 11px;
    font-weight: 650;
  }

  .shortcut-list {
    overflow: hidden;
    border: 1px solid #e8e5de;
    border-radius: 9px;
    background: #fffefa;
  }

  .shortcut-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    min-height: 49px;
    padding: 11px 14px;
  }

  .shortcut-row + .shortcut-row {
    border-top: 1px solid #efede7;
  }

  .action {
    color: #393832;
    font-size: 12px;
  }

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
    min-height: 24px;
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

  .empty-state {
    padding: 44px 16px;
    border: 1px dashed #dedad0;
    border-radius: 10px;
    text-align: center;
  }

  .empty-state > span {
    color: #b4775c;
    font-size: 28px;
  }

  .empty-state h2 {
    margin: 10px 0 5px;
    font-size: 15px;
  }

  .empty-state p {
    color: #77746c;
    font-size: 12px;
  }

  .empty-state button {
    padding: 8px 12px;
    border: 1px solid #e5e2db;
    border-radius: 6px;
    background: #fffefa;
    cursor: pointer;
    font-size: 11px;
  }

  .help-footer {
    display: flex;
    align-items: flex-start;
    gap: 13px;
    margin-top: 30px;
    padding: 18px;
    border: 1px solid #e8e2d6;
    border-radius: 10px;
    background: #f5f1e8;
  }


  .help-footer strong {
    font-size: 12px;
    font-weight: 600;
  }

  .help-footer p {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 5px;
    margin: 7px 0 0;
    color: #77746c;
    font-size: 11px;
    line-height: 1.8;
  }

  @media (max-width: 520px) {
    .help-page {
      padding-inline: 2px;
    }

    .toolbar {
      align-items: stretch;
      flex-direction: column;
    }

    .search-box {
      max-width: none;
    }

    .platform-switch {
      align-self: flex-start;
    }

    .shortcut-row {
      gap: 8px;
      padding-inline: 10px;
    }

    .action {
      font-size: 11px;
    }
  }
</style>
