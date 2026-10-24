
<script lang="ts">
  import { goto } from "$app/navigation";

  interface Chat {
    id: string;
    title: string;
    preview: string;
    updatedAt: string;
  }

  let searchQuery = $state("");

  // Placeholder data until chat persistence is implemented.
  let chats = $state<Chat[]>([]);

  let filteredChats = $derived(
    chats.filter((chat) =>
      `${chat.title} ${chat.preview}`
        .toLowerCase()
        .includes(searchQuery.trim().toLowerCase())
    )
  );

  function openChat(chat: Chat) {
    // Replace with the persisted chat route when implemented.
    goto("/app/newchat");
  }

  function createChat() {
    goto("/app/newchat");
  }
</script>

<div class="chats-page">
  <header class="page-header">
    <div>
      <p class="eyebrow">YOUR WORKSPACE</p>
      <h1>Chats</h1>
      <p class="subtitle">Your conversations, all in one place.</p>
    </div>

    <button class="primary-button" onclick={createChat}>
      <span>＋</span>
      New chat
      <kbd>⌘ N</kbd>
    </button>
  </header>

  <div class="toolbar">
    <label class="search-box">
      <span class="search-icon">⌕</span>
      <input
        bind:value={searchQuery}
        placeholder="Search conversations..."
        aria-label="Search conversations"
      />
      <kbd>/</kbd>
    </label>
  </div>

  {#if chats.length === 0}
    <section class="empty-state">
      <div class="empty-icon">◷</div>
      <h2>Your conversations will appear here</h2>
      <p>
        Start a conversation with your AI agent. Your chats will be available
        here once conversation history is saved.
      </p>
      <button class="secondary-button" onclick={createChat}>
        Start a conversation <span>→</span>
      </button>
    </section>
  {:else if filteredChats.length === 0}
    <section class="empty-state">
      <div class="empty-icon">⌕</div>
      <h2>No matching conversations</h2>
      <p>Try another search term.</p>
      <button
        class="secondary-button"
        onclick={() => (searchQuery = "")}
      >
        Clear search
      </button>
    </section>
  {:else}
    <section class="chat-list" aria-label="Conversations">
      {#each filteredChats as chat (chat.id)}
        <button
          class="chat-item"
          onclick={() => openChat(chat)}
        >
          <span class="chat-icon">✳</span>
          <span class="chat-details">
            <span class="chat-title">{chat.title}</span>
            <span class="chat-preview">{chat.preview}</span>
          </span>
          <span class="chat-date">{chat.updatedAt}</span>
          <span class="chat-arrow">→</span>
        </button>
      {/each}
    </section>
  {/if}

  <footer class="page-footer">
    <span>Golden Hour Workspace</span>
    <span>Conversations stay on this page until persistence is added.</span>
  </footer>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  .chats-page {
    min-height: 100%;
    max-width: 1000px;
    margin: 0 auto;
    padding: 42px 36px 24px;
    display: flex;
    flex-direction: column;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
  }

  .eyebrow {
    margin: 0 0 12px;
    color: #d78261;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.13em;
  }

  h1 {
    margin: 0;
    color: #302f2a;
    font-family: Georgia, "Times New Roman", serif;
    font-size: 34px;
    font-weight: 400;
    letter-spacing: -0.8px;
  }

  .subtitle {
    margin: 10px 0 0;
    color: #89867e;
    font-size: 12px;
    line-height: 1.6;
  }

  button,
  input {
    font: inherit;
  }

  button {
    cursor: pointer;
  }

  .primary-button {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border: 1px solid #302f2a;
    border-radius: 8px;
    background: #302f2a;
    color: white;
    font-size: 11px;
  }

  .primary-button:hover {
    background: #48463f;
  }

  kbd {
    color: #a5a196;
    font-family: inherit;
    font-size: 10px;
  }

  .toolbar {
    margin-top: 34px;
    padding-bottom: 16px;
    border-bottom: 1px solid #eae7df;
  }

  .search-box {
    display: flex;
    align-items: center;
    gap: 10px;
    max-width: 440px;
    padding: 11px 12px;
    border: 1px solid #e5e2db;
    border-radius: 9px;
    background: #fff;
  }

  .search-icon {
    color: #89867e;
    font-size: 18px;
  }

  .search-box input {
    flex: 1;
    min-width: 0;
    border: 0;
    outline: 0;
    background: transparent;
    color: #302f2a;
    font-size: 12px;
  }

  .search-box input::placeholder {
    color: #a09c93;
  }

  .search-box:focus-within {
    border-color: #cfc6b7;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    min-height: 300px;
    padding: 50px 20px;
    text-align: center;
  }

  .empty-icon {
    display: grid;
    width: 54px;
    height: 54px;
    place-items: center;
    margin-bottom: 20px;
    border: 1px solid #e7e3da;
    border-radius: 15px;
    background: #f5f3ed;
    color: #d78261;
    font-size: 25px;
  }

  .empty-state h2 {
    margin: 0;
    color: #393831;
    font-size: 15px;
    font-weight: 550;
  }

  .empty-state p {
    max-width: 350px;
    margin: 10px 0 20px;
    color: #89867e;
    font-size: 11px;
    line-height: 1.8;
  }

  .secondary-button {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 13px;
    border: 1px solid #e5e2db;
    border-radius: 8px;
    background: #fff;
    color: #48463f;
    font-size: 11px;
  }

  .secondary-button:hover {
    background: #f2f0e9;
  }

  .chat-list {
    display: flex;
    flex-direction: column;
    padding-top: 8px;
  }

  .chat-item {
    display: flex;
    align-items: center;
    gap: 14px;
    width: 100%;
    padding: 15px 10px;
    border: 0;
    border-bottom: 1px solid #efede7;
    background: transparent;
    text-align: left;
  }

  .chat-item:hover {
    background: #f3f1eb;
  }

  .chat-icon {
    display: grid;
    flex-shrink: 0;
    width: 34px;
    height: 34px;
    place-items: center;
    border: 1px solid #e9dfd4;
    border-radius: 9px;
    color: #d78261;
    font-size: 18px;
  }

  .chat-details {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 5px;
    min-width: 0;
  }

  .chat-title {
    overflow: hidden;
    color: #393831;
    font-size: 12px;
    font-weight: 550;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chat-preview {
    overflow: hidden;
    color: #89867e;
    font-size: 11px;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chat-date {
    color: #969188;
    font-size: 10px;
  }

  .chat-arrow {
    color: #969188;
    opacity: 0;
  }

  .chat-item:hover .chat-arrow {
    opacity: 1;
  }

  .page-footer {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: 10px;
    margin-top: auto;
    padding-top: 20px;
    color: #a09c93;
    font-size: 10px;
  }

  @media (max-width: 600px) {
    .chats-page {
      padding: 28px 18px 20px;
    }

    .page-header {
      align-items: flex-start;
    }

    .primary-button kbd {
      display: none;
    }

    .chat-date {
      display: none;
    }
  }
</style>
