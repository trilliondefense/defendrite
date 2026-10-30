
<script lang="ts">
  import { goto } from "$app/navigation";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount, tick } from "svelte";
  import type { Snippet } from "svelte";

  interface Conversation {
    id: string;
    title: string;
    chatNumber: number;
    createdAt: number;
    updatedAt: number;
  }

  let { children }: { children: Snippet } = $props();

  let collapsed = $state(false);
  let paletteOpen = $state(false);
  let query = $state("");
  let selectedIndex = $state(0);
  let searchInput: HTMLInputElement | undefined= $state(undefined);
  let cases = $state<Conversation[]>([]);


  const navItems = [
    { label: "Investigations", href: "/app/investigations", icon: "🔍", keywords: "history conversations" ,shortcut:"p"},
    { label: "Evidence Vault", href: "/app/evidence-vault", icon: "🏛️", keywords: "history conversations" ,shortcut:"y"},
    { label: "Reports", href: "/app/reports", icon: "📊", keywords: "workspace files" ,shortcut:"o"},
    { label: "Customize", href: "/app/customize", icon: "⚙️", keywords: "settings appearance" ,shortcut:"u"},
    { label: "Help", href: "/app/help", icon: "❓", keywords: "help" ,shortcut:"i"},
  ];
  type CaseStatus = "Active" | "Review" | "Closed";
  interface RecentCase{
    label:string, 
    caseId:string ,
    status : CaseStatus
  }
  let recentCases : RecentCase[]= $state([
    
  ])
  const MAX_RECENT_CASES = 4 
  let visibleRecentCases = $derived(recentCases.slice(0,MAX_RECENT_CASES)); 

  let filteredItems = $derived(
    navItems.filter((item) =>
      `${item.label} ${item.keywords}`
        .toLowerCase()
        .includes(query.trim().toLowerCase())
    )
  );let isMac = $state(false);

  onMount(() => {
      isMac = /Mac|iPhone|iPad|iPod/i.test(navigator.platform);
    collapsed = localStorage.getItem("sidebar-collapsed") === "true";
      loadChats();
  });
  
  const pageSize = 4;

  let paginatedCases = $derived(
    cases.slice(0,pageSize
    )
  );



  async function loadChats() {
    try {
      const result = await invoke<Conversation[]>("list_chats");
      cases = result.sort(
        (a, b) => b.updatedAt - a.updatedAt
      );

    } catch (err) {
      console.error("Failed to load chats:", err);
    } 
  }


  function toggleSidebar() {
    collapsed = !collapsed;
    localStorage.setItem("sidebar-collapsed", String(collapsed));
  }

  async function openPalette() {
    paletteOpen = true;
    query = "";
    selectedIndex = 0;
    await tick();
    searchInput?.focus();
  }

  function closePalette() {
    paletteOpen = false;
    query = "";
    selectedIndex = 0;
  }

  function handleGlobalKeydown(event: KeyboardEvent) {
  const modifier = event.ctrlKey || event.metaKey;
  const key = event.key.toLowerCase();

  // Don't intercept shortcuts while using browser-like text editing.
  const target = event.target;
  const isTyping =
    target instanceof HTMLElement &&
    (target.isContentEditable ||
      ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName));

  if (modifier && key === "n") {
    event.preventDefault();
    window.location.href = "/app/newchat";
    return;
  }

    if (modifier && key === "u") {
    event.preventDefault();
    window.location.href = "/app/customize";
    return;
  }


    if (modifier && key === "o") {
    event.preventDefault();
    window.location.href = "/app/reports";
    return;
  }

    if (modifier && key === "y") {
    event.preventDefault();
    window.location.href = "/app/evidence-vault";
    return;
  }


    if (modifier && key === "p") {
    event.preventDefault();
    window.location.href = "/app/investigations";
    return;
  }


  if (modifier && key === "i") {
    event.preventDefault();
    window.location.href = "/app/help";
    return;
  }

  if (modifier && key === "b") {
    event.preventDefault();
    toggleSidebar();
    return;
  }

  if (modifier && key === "k") {
    event.preventDefault();

    if (paletteOpen) {
      closePalette();
    } else {
      void openPalette();
    }
  }
}

  function handlePaletteKeydown(event: KeyboardEvent) {
    if (event.key === "ArrowDown") {
      event.preventDefault();
      selectedIndex =
        filteredItems.length > 0
          ? (selectedIndex + 1) % filteredItems.length
          : 0;
    } else if (event.key === "ArrowUp") {
      event.preventDefault();
      selectedIndex =
        filteredItems.length > 0
          ? (selectedIndex - 1 + filteredItems.length) %
            filteredItems.length
          : 0;
    } else if (event.key === "Enter" && filteredItems.length > 0) {
      event.preventDefault();
      window.location.href = filteredItems[selectedIndex].href;
    } else if (event.key === "Escape") {
      event.preventDefault();
      closePalette();
    }
  }
</script>

<svelte:window onkeydown={handleGlobalKeydown} />

<div class:collapsed class="app-layout">
  <aside class="sidebar" aria-label="Main navigation">
    <div class="sidebar-header">
      <a class="brand" href="/app/newchat" title="Home">
        <img src="/favicon.png" alt="Brand icon" class="brand-mark" />
        {#if !collapsed}
          <span class="brand-name">Workspace</span>
        {/if}
      </a>

      <button
        class="icon-button collapse-button"
        onclick={toggleSidebar}
        aria-label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
        title={collapsed ? "Expand sidebar (Ctrl+B)" : "Collapse sidebar (Ctrl+B)"}
      >
        {collapsed ? "›" : "‹"}
      </button>
    </div>

    <button
      class="new-chat"
      onclick={() => (window.location.href = "/app/newchat")}
      title="New Investigation"
    >
      <span class="nav-icon">➕</span>
      {#if !collapsed}
        <span>New Chat</span>
        <kbd>{isMac ? "⌘" : "Ctrl"} N</kbd>
        {/if}
    </button>

    <button
      class="search-trigger"
      onclick={() => void openPalette()}
      title="Search navigation (Ctrl+K)"
    >
      <span class="nav-icon">🌐</span>
      {#if !collapsed}
        <span>Search</span>
  <kbd>{isMac ? "⌘" : "Ctrl"} K</kbd>      {/if}
    </button>

    <div class="section-label">
      {#if !collapsed}
        <span>WORKSPACE</span>
      {:else}
        <span class="section-dot">·</span>
      {/if}
    </div>

    <nav class="nav-list">
      {#each navItems as item (item.href)}
        <a
          class="nav-item"
          href={item.href}
          title={collapsed ? item.label : undefined}
          aria-label={item.label}
        >
          <span class="nav-icon">{item.icon}</span>
          {#if !collapsed}
            <span class="nav-label">{item.label}</span>
            <kbd>{isMac ? "⌘" : "Ctrl"} {item.shortcut.toLocaleUpperCase()}</kbd>
          {/if}
        </a>
      {/each}
    </nav>

    {#if paginatedCases.length>0}

    <div class="section-label">
      {#if !collapsed}
        <span>RECENT CASES</span>
      {:else}
        <span class="section-dot">·</span>
      {/if}
    </div>

    <nav class="nav-list">
      {#each paginatedCases as item,index (item.id)}
        <a
          class="nav-item"
          href="/app/investigations/{encodeURIComponent(item.id)}"
          title={collapsed ? item.title : undefined}
          aria-label={item.title}
          data-sveltekit-replacestate
        >
          {#if !collapsed}
            <span class="nav-case-label">{item.title.toLocaleUpperCase()}</span>
            <kbd>{isMac ? "⌘" : "Ctrl"} {index+1}</kbd>
          {:else}
          <span class="nav-case-label">{index+1}</span>
            
          {/if}
        </a>
      {/each}
    </nav>

      
    {/if}

    <div class="sidebar-spacer"></div>

    <button class="profile" title="Account settings">
      <span class="avatar">B</span>
      {#if !collapsed}
        <span class="profile-info">
          <span class="profile-name">{localStorage.getItem("userName")}</span>
          <span class="profile-plan">Investigator</span>
        </span>
        <span class="profile-menu">···</span>
      {/if}
    </button>
  </aside>

  <main class="content">
    {@render children()}
  </main>
</div>

{#if paletteOpen}
  <div
    class="palette-backdrop"
    role="presentation"
    onclick={(event) => {
      if (event.target === event.currentTarget) closePalette();
    }}
    onkeydown={(event) => {
      if (event.key === "Escape") closePalette();
    }}
  >
    <section
      class="palette"
      aria-label="Quick navigation"
    >
      <div class="palette-search">
        <span class="search-icon">⌕</span>
        <input
          bind:this={searchInput}
          bind:value={query}
          oninput={() => (selectedIndex = 0)}
          onkeydown={handlePaletteKeydown}
          placeholder="Where do you want to go?"
          aria-label="Search pages"
          aria-controls="palette-results"
          aria-activedescendant={
            filteredItems.length > 0
              ? `palette-item-${selectedIndex}`
              : undefined
          }
        />
        <kbd>ESC</kbd>
      </div>

      <div class="palette-section">NAVIGATION</div>

      <div class="palette-results" id="palette-results">
        {#each filteredItems as item, index (item.href)}
          <a
            id="palette-item-{index}"
            class:active={selectedIndex === index}
            class="palette-item"
            href={item.href}
            onclick={closePalette}
            onmouseenter={() => (selectedIndex = index)}
          >
            <span class="nav-icon">{item.icon}</span>
            <span>{item.label}</span>
            <span class="result-arrow">↵</span>
          </a>
        {:else}
          <div class="empty-state">No matching pages found.</div>
        {/each}
      </div>

      <footer class="palette-footer">
        <span>↑↓ Navigate</span>
        <span>↵ Open</span>
        <span>Esc Close</span>
      </footer>
    </section>
  </div>
{/if}

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI",
      sans-serif;
    background: #faf9f6;
    color: #292824;
  }

  :global(button),
  :global(input) {
    font: inherit;
  }

  .app-layout {
    display: flex;
    height: 100dvh;
    overflow: hidden;
  }

  .sidebar {
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
    
    width: 220px;
    padding: 14px 10px 10px;
    gap: 5px;
    background: #f7f6f2;
    border-right: 1px solid #e9e6df;
    transition: width 160ms ease;
    overflow-y: auto; 

  }

  .collapsed .sidebar {
    width: 58px;
    padding-inline: 8px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 34px;
    margin-bottom: 12px;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 9px;
    min-width: 0;
    color: inherit;
    text-decoration: none;
    padding-left: 5px;
  }

 .brand-mark {
  width: 36px;
  height: 36px;
  object-fit: contain;
}
  .brand-name {
    font-size: 14px;
    font-weight: 600;
    white-space: nowrap;
  }

  .icon-button {
    border: 0;
    background: transparent;
    color: #77746c;
    border-radius: 6px;
    cursor: pointer;
    width: 26px;
    height: 28px;
  }

  .icon-button:hover,
  .search-trigger:hover,
  .nav-item:hover {
    background: #ebe9e2;
  }

  .collapsed .sidebar-header {
    flex-direction: column;
    gap: 12px;
  }

  .collapsed .brand {
    padding: 0;
  }

  .new-chat,
  .search-trigger,
  .nav-item {
    display: flex;
    align-items: center;
    gap: 11px;
    width: 100%;
    min-height: 35px;
    padding: 7px 9px;
    border: 0;
    border-radius: 7px;
    color: #45443f;
    background: transparent;
    text-decoration: none;
    text-align: left;
    font-size: 12px;
    cursor: pointer;
    white-space: nowrap;
  }

  .new-chat {
    color: #292824;
    font-weight: 550;
  }

  .new-chat:hover {
    background: #e3e0d7;
  }

  .collapsed .new-chat,
  .collapsed .search-trigger,
  .collapsed .nav-item {
    justify-content: center;
    padding-inline: 0;
  }

  .nav-icon {
    flex-shrink: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 17px;
    font-size: 16px;
    line-height: 1;
  }

  kbd {
    margin-left: auto;
    padding: 2px 4px;
    border: 1px solid #e4e1d9;
    border-radius: 4px;
    background: #f8f7f3;
    color: #88857d;
    font-family: inherit;
    font-size: 10px;
    white-space: nowrap;
  }

  .section-label {
    margin: 19px 0 5px;
    padding: 0 9px;
    color: #969188;
    font-size: 9px;
    font-weight: 650;
    letter-spacing: 0.8px;
  }

  .section-dot {
    padding-left: 7px;
    font-size: 18px;
  }

  .nav-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-label {
    overflow: hidden;
    text-overflow: ellipsis;
  }




.nav-case-label {
  font-family: monospace; /* Keeps the case numbers perfectly aligned */
  font-size: 11px;
  font-weight: 600;
  color: #333332;
  letter-spacing: 0.5px;
}

  .sidebar-spacer {
    flex: 1;
  }

  .profile {
    display: flex;
    align-items: center;
    gap: 9px;
    width: 100%;
    padding: 8px 4px;
    border: 0;
    border-top: 1px solid #e9e6df;
    background: transparent;
    text-align: left;
    cursor: pointer;
    border-radius: 5px;
  }

  .collapsed .profile {
    justify-content: center;
    padding-inline: 0;
  }

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: 26px;
    height: 26px;
    border-radius: 50%;
    background: #393a35;
    color: white;
    font-size: 12px;
  }

  .profile-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .profile-name {
    font-size: 11px;
    font-weight: 600;
  }

  .profile-plan {
    color: #89867d;
    font-size: 10px;
  }

  .profile-menu {
    margin-left: auto;
    color: #89867d;
  }

  .content {
    flex: 1;
    min-width: 0;
    overflow: auto;
    padding: 32px;
  }

  .palette-backdrop {
    position: fixed;
    z-index: 1000;
    inset: 0;
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding: min(18vh, 140px) 20px 20px;
    background: rgb(28 27 24 / 30%);
    backdrop-filter: blur(2px);
  }

  .palette {
    width: 100%;
    max-width: 510px;
    overflow: hidden;
    border: 1px solid #e7e3da;
    border-radius: 12px;
    background: #fffefa;
    box-shadow: 0 20px 70px rgb(0 0 0 / 15%);
  }

  .palette-search {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 14px;
    border-bottom: 1px solid #eeece6;
  }

  .search-icon {
    color: #8e8a80;
    font-size: 20px;
  }

  .palette-search input {
    flex: 1;
    min-width: 0;
    outline: none;
    border: none;
    background: transparent;
    color: #292824;
    font-size: 14px;
  }

  .palette-search kbd {
    margin-left: 0;
  }

  .palette-section {
    padding: 12px 14px 6px;
    color: #989389;
    font-size: 10px;
    font-weight: 650;
    letter-spacing: 0.7px;
  }

  .palette-results {
    max-height: 320px;
    overflow: auto;
    padding: 4px 7px 8px;
  }

  .palette-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px;
    border-radius: 7px;
    color: #393832;
    font-size: 13px;
    text-decoration: none;
  }

  .palette-item.active,
  .palette-item:hover {
    background: #f0eee7;
  }

  .result-arrow {
    margin-left: auto;
    color: #8e8a80;
    opacity: 0;
  }

  .palette-item.active .result-arrow {
    opacity: 1;
  }

  .empty-state {
    padding: 25px 12px;
    color: #88857d;
    text-align: center;
    font-size: 13px;
  }

  .palette-footer {
    display: flex;
    gap: 15px;
    padding: 10px 14px;
    border-top: 1px solid #eeece6;
    color: #89867d;
    font-size: 10px;
  }

  :global(a:focus-visible),
  :global(button:focus-visible),
  :global(input:focus-visible) {
    outline: 2px solid #bd795e;
    outline-offset: 2px;
  }

  @media (prefers-reduced-motion: reduce) {
    .sidebar {
      transition: none;
    }
  }
</style>
