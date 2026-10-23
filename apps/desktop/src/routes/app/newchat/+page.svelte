
<script lang="ts">

  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount, tick } from "svelte";
  import { goto } from "$app/navigation";

  interface Message {
    id: string;
    sender: "user" | "agent";
    text: string;
  }

  let messages = $state<Message[]>([
    {
      id: "welcome",
      sender: "agent",
      text: "Hello! I'm your AI agent. What would you like to work on today?"
    }
  ]);

  let input = $state("");
  let isLoading = $state(false);
  let viewportElement = $state<HTMLElement | null>(null);
  let inputElement = $state<HTMLInputElement | null>(null);

  let activeAgentMsgId: string | null = $state(null);

  async function scrollToBottom() {
    await tick();

    viewportElement?.scrollTo({
      top: viewportElement.scrollHeight,
      behavior: "smooth"
    });
  }

  function focusInput() {
    inputElement?.focus();
  }

  function newChat() {
    if (isLoading) return;

    messages = [
      {
        id: crypto.randomUUID(),
        sender: "agent",
        text: "Hello! I'm your AI agent. What would you like to work on today?"
      }
    ];

    activeAgentMsgId = null;
    input = "";
    focusInput();
  }

  function handleGlobalKeydown(event: KeyboardEvent) {
    const modifier = event.metaKey || event.ctrlKey;
    const target = event.target;

    const typing =
      target instanceof HTMLInputElement ||
      target instanceof HTMLTextAreaElement ||
      (target instanceof HTMLElement && target.isContentEditable);

    if (modifier && event.key.toLowerCase() === "n") {
      event.preventDefault();
      newChat();
      return;
    }

    if (event.key === "Escape" && !typing) {
      focusInput();
    }

    if (event.key === "/" && !modifier && !event.altKey && !typing) {
      event.preventDefault();
      focusInput();
    }
  }

  onMount(() => {
    let disposed = false;
    let unlisten: (() => void) | undefined;

    listen<string>("agent-token", (event) => {
      if (disposed || !activeAgentMsgId) return;

      const activeMessage = messages.find(
        (message) => message.id === activeAgentMsgId
      );

      if (activeMessage) {
        activeMessage.text += event.payload;
        void scrollToBottom();
      }
    }).then((unsubscribe) => {
      if (disposed) {
        unsubscribe();
      } else {
        unlisten = unsubscribe;
      }
    }).catch((error) => {
      console.error("Failed to register agent event listener:", error);
    });

    window.addEventListener("keydown", handleGlobalKeydown);

    focusInput();

    return () => {
      disposed = true;
      unlisten?.();
      window.removeEventListener("keydown", handleGlobalKeydown);
    };
  });
async function handleSendMessage(event: SubmitEvent) {
  event.preventDefault();

  const prompt = input.trim();

  // Guard loop to protect execution boundaries
  if (!prompt || isLoading) return;

  // Svelte 5 Rune-safe state array updating
  messages = [
    ...messages,
    {
      id: crypto.randomUUID(),
      sender: "user",
      text: prompt
    }
  ];

  const agentMessageId = crypto.randomUUID();

  // Initialize placeholder for real-time streaming tokens
  messages = [
    ...messages,
    {
      id: agentMessageId,
      sender: "agent",
      text: ""
    }
  ];

  activeAgentMsgId = agentMessageId;
  input = "";
  isLoading = true;

  void scrollToBottom();

  try {
    // Dispatches the background Tokio runtime agent runner
    await invoke("ask_agent", { prompt });
  } catch (error) {
    const errorString = String(error);
    
    // Find the message index to trigger Svelte 5 fine-grained array reactivity
    const targetIdx = messages.findIndex((m) => m.id === agentMessageId);

    if (targetIdx !== -1) {
      if (errorString.includes("SETUP_REQUIRED")) {
        messages[targetIdx].text = "⚠️ Workspace Configuration Required: Please navigate to Settings and specify your Google AI Studio API Key to communicate with local models.";
        
        // Hook: Proactively launch your configuration UI here if desired
        // showWorkspaceSettingsModal = true;
      } else {
        messages[targetIdx].text = `Runtime Execution Failure: ${errorString}`;
      }
    }
  } finally {
    isLoading = false;
    activeAgentMsgId = null;
    void scrollToBottom();
    focusInput();
  }
}

</script>
<div class="chat-page">
  <header class="chat-header">
    <div class="header-left">
      <button
        class="icon-button mobile-back"
        aria-label="Back to workspace"
        onclick={() => goto("/app")}
      >
        ←
      </button>

      <div class="header-title">
                            <img src="/favicon.png" alt="Brand icon" class="brand-mark" />

        <div>
          <h1>New Chat</h1>
          <p>AI workspace</p>
        </div>
      </div>
    </div>

    <div class="header-actions">
      <div class="engine-status">
        <span class="status-dot"></span>
        Local Engine
      </div>

      <button
        class="header-button"
        onclick={newChat}
        disabled={isLoading}
        title="New chat (Ctrl/⌘ N)"
      >
        <span>＋</span>
        <span>New chat</span>
        <kbd>⌘ N</kbd>
      </button>
    </div>
  </header>

  <main class="chat-viewport" bind:this={viewportElement}>
    <div class="message-list">
      {#each messages as msg (msg.id)}
        <div class="message-row" class:user={msg.sender === "user"} class:agent={msg.sender === "agent"}>
          {#if msg.sender === "agent"}
                    <img src="/favicon.png" alt="Brand icon" class="message-avatar" />

            
          {/if}

          <div class="message-content">
            <div class="sender-label">
              {msg.sender === "user" ? "You" : "Assistant"}
            </div>

            <div class="message-text">
              {#if msg.text}
                {msg.text}
              {:else if isLoading && msg.id === activeAgentMsgId}
                <span class="thinking-indicator">
                  <span></span>
                  <span></span>
                  <span></span>
                </span>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  </main>

  <footer class="chat-footer">
    <form onsubmit={handleSendMessage} class="input-form">
      <input
        bind:this={inputElement}
        bind:value={input}
        placeholder="Ask anything, or describe what you're building..."
        disabled={isLoading}
        class="text-input"
        aria-label="Message your AI agent"
        autocomplete="off"
      />

      <div class="input-toolbar">
        <div class="input-hints">
          <span>↵ Send</span>
          <span>⌘ N New chat</span>
        </div>

        <button
          type="submit"
          disabled={isLoading || !input.trim()}
          class="send-button"
          aria-label="Send message"
        >
          {#if isLoading}
            <span class="spinner"></span>
          {:else}
            <span>↑</span>
          {/if}
        </button>
      </div>
    </form>

    <p class="disclaimer">
      AI can make mistakes. Review important information.
    </p>
  </footer>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  .chat-page {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
    background: #fbfaf7;
    color: #302f2a;
  }

  .chat-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    flex-shrink: 0;
    min-height: 70px;
    padding: 12px 25px;
    border-bottom: 1px solid #eae7df;
  }

  .header-left,
  .header-title,
  .header-actions {
    display: flex;
    align-items: center;
  }

  .header-title {
    gap: 11px;
  }

  .brand-mark {
  width: 36px;
  height: 36px;
  object-fit: contain;
}
  .header-title h1 {
    margin: 0;
    font-size: 13px;
    font-weight: 600;
  }

  .header-title p {
    margin: 4px 0 0;
    color: #89867e;
    font-size: 10px;
  }

  .header-actions {
    gap: 16px;
  }

  .engine-status {
    display: flex;
    align-items: center;
    gap: 7px;
    color: #77746d;
    font-size: 10px;
  }

  .status-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: #4b9c70;
  }

  .header-button,
  .icon-button {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 8px 10px;
    border: 1px solid #e5e2db;
    border-radius: 7px;
    background: #fff;
    color: #55534d;
    font-size: 11px;
    cursor: pointer;
  }

  .header-button:hover,
  .icon-button:hover {
    background: #f1efe9;
  }

  .header-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  kbd {
    color: #99958c;
    font-family: inherit;
    font-size: 10px;
  }

  .mobile-back {
    display: none;
  }

  .chat-viewport {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    overscroll-behavior: contain;
    scroll-behavior: smooth;
  }

  .message-list {
    display: flex;
    flex-direction: column;
    gap: 30px;
    width: 100%;
    max-width: 780px;
    min-height: 100%;
    margin: 0 auto;
    padding: 36px 32px;
  }

  .message-row {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    width: 100%;
  }

  .message-row.user {
    justify-content: flex-end;
  }

  .message-avatar {
    display: grid;
    flex-shrink: 0;
    width: 29px;
    height: 29px;
    place-items: center;
    border: 1px solid #e9dfd4;
    border-radius: 9px;
    color: #d78261;
    font-size: 18px;
  }

  .message-content {
    max-width: min(85%, 620px);
    min-width: 0;
  }

  .sender-label {
    margin-bottom: 8px;
    color: #89867e;
    font-size: 10px;
    font-weight: 550;
  }

  .message-text {
    color: #45433d;
    font-size: 13px;
    line-height: 1.8;
    overflow-wrap: anywhere;
    white-space: pre-wrap;
  }

  .message-row.user .message-content {
    padding: 12px 15px;
    border: 1px solid #e9e5dc;
    border-radius: 12px 12px 3px 12px;
    background: #f2f0e9;
  }

  .message-row.user .sender-label {
    text-align: right;
  }

  .thinking-indicator {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 5px 0;
  }

  .thinking-indicator span {
    width: 5px;
    height: 5px;
    border-radius: 50%;
    background: #d78261;
    animation: thinking 1s infinite ease-in-out;
  }

  .thinking-indicator span:nth-child(2) {
    animation-delay: 0.15s;
  }

  .thinking-indicator span:nth-child(3) {
    animation-delay: 0.3s;
  }

  @keyframes thinking {
    0%, 60%, 100% {
      opacity: 0.35;
      transform: translateY(0);
    }

    30% {
      opacity: 1;
      transform: translateY(-3px);
    }
  }

  .chat-footer {
    flex-shrink: 0;
    width: 100%;
    max-width: 780px;
    margin: 0 auto;
    padding: 12px 32px 16px;
  }

  .input-form {
    padding: 13px;
    border: 1px solid #e3e0d8;
    border-radius: 13px;
    background: #fff;
    box-shadow: 0 4px 16px rgb(40 35 25 / 3%);
    transition: border-color 150ms;
  }

  .input-form:focus-within {
    border-color: #cfc6b7;
  }

  .text-input {
    display: block;
    width: 100%;
    padding: 4px 3px 16px;
    border: 0;
    outline: 0;
    background: transparent;
    color: #302f2a;
    font-size: 12px;
  }

  .text-input::placeholder {
    color: #a09c93;
  }

  .text-input:disabled {
    opacity: 0.65;
  }

  .input-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .input-hints {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    color: #969188;
    font-size: 10px;
  }

  .send-button {
    display: grid;
    flex-shrink: 0;
    width: 31px;
    height: 31px;
    place-items: center;
    border: 0;
    border-radius: 8px;
    background: #d78261;
    color: white;
    font-size: 22px;
    line-height: 1;
    cursor: pointer;
    transition: background 120ms;
  }

  .send-button:hover:not(:disabled) {
    background: #c47150;
  }

  .send-button:disabled {
    background: #e5e1d9;
    color: #aaa69d;
    cursor: not-allowed;
  }

  .spinner {
    width: 13px;
    height: 13px;
    border: 2px solid rgb(255 255 255 / 45%);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.7s linear infinite;
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  .disclaimer {
    margin: 10px 0 0;
    color: #a09c93;
    text-align: center;
    font-size: 9px;
  }

  @media (max-width: 650px) {
    .chat-header {
      min-height: 60px;
      padding: 10px 14px;
    }

    .mobile-back {
      display: flex;
    }

    .header-left {
      gap: 10px;
    }

    .engine-status {
      display: none;
    }

    .header-button kbd {
      display: none;
    }

    .message-list {
      padding: 24px 16px;
    }

    .chat-footer {
      padding: 10px 14px 12px;
    }

    .message-content {
      max-width: 90%;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .thinking-indicator span,
    .spinner {
      animation: none;
    }
  }
</style>
