<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event"; 
  import { onMount, tick } from "svelte";

  // Strict structural typing for chat messages
  interface Message {
    id: string;
    sender: "user" | "agent";
    text: string;
  }

  // 1. Core Reactive States utilizing modern Svelte 5 Runes (\$state)
  let messages = $state<Message[]>([
    { 
      id: "welcome", 
      sender: "agent", 
      text: "Hello! I am your AI agent workspace harness running on Svelte 5 Runes. How can I assist you with your project today?" 
    }
  ]);
  
  let input = $state("");
  let isLoading = $state(false);
  let viewportElement = $state<HTMLElement | null>(null);
  let activeAgentMsgIndex = -1;
  // 2. Smoothly scroll container as the agent processes tokens
  async function scrollToBottom() {
    await tick();
    if (viewportElement) {
      viewportElement.scrollTo({
        top: viewportElement.scrollHeight,
        behavior: "smooth"
      });
    }
  }

  onMount(() => {
    const unlisten = listen<string>("agent-token", (event) => {
      const token = event.payload;
      
      // Append token safely into our active message state pointer
      if (activeAgentMsgIndex !== -1) {
        messages[activeAgentMsgIndex].text += token;
        scrollToBottom();
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  });
  // 3. Action Dispatcher
 async function handleSendMessage(event: SubmitEvent) {
    event.preventDefault();
    if (!input.trim() || isLoading) return;

    messages.push({
      id: crypto.randomUUID(),
      sender: "user",
      text: input.trim(),
    });
    
    const currentPrompt = input;
    input = "";
    isLoading = true;

    // Create an empty agent message block first to catch incoming stream chunks
    messages.push({
      id: crypto.randomUUID(),
      sender: "agent",
      text: ""
    });
    
    // Set pointer index pointing to the newly generated empty block
    activeAgentMsgIndex = messages.length - 1;
    scrollToBottom();

    try {
      // Fire-and-forget: The Rust command will now return instantly,
      // and tokens will arrive via the "agent-token" event listener.
      await invoke("ask_agent", { prompt: currentPrompt });
    } catch (error) {
      if (activeAgentMsgIndex !== -1) {
        messages[activeAgentMsgIndex].text = `Stream Error: ${String(error)}`;
      }
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="app-container">
  <!-- Top Navigation Bar -->
  <header class="header">
    <div class="header-title">Claude Harness Workspace (Runes Mode)</div>
    <div class="badge">Local Engine</div>
  </header>

  <!-- Conversation Viewport -->
  <main class="chat-viewport" bind:this={viewportElement}>
    <div class="message-list">
      {#each messages as msg (msg.id)}
        <div class="message-row {msg.sender}">
          <div class="bubble {msg.sender}">
            <div class="sender-label">
              {msg.sender === "user" ? "You" : "Agent Core"}
            </div>
            <div class="message-text">{msg.text}</div>
          </div>
        </div>
      {/each}

      {#if isLoading}
        <div class="message-row agent loading">
          <div class="bubble agent">
            <div class="loading-dots">Thinking...</div>
          </div>
        </div>
      {/if}
    </div>
  </main>

  <!-- Persistent Prompt Control Dock -->
  <footer class="footer">
    <!-- Modern Svelte 5 uses standard HTML 'onsubmit' syntax instead of legacy 'on:submit' -->
    <form onsubmit={handleSendMessage} class="input-form">
      <input
        type="text"
        bind:value={input}
        placeholder="Ask your agent or describe changes..."
        disabled={isLoading}
        class="text-input"
      />
      <button type="submit" disabled={isLoading || !input.trim()} class="send-button">
        Send
      </button>
    </form>
  </footer>
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
    margin: 0;
    padding: 0;
    overflow: hidden;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 24px;
    border-bottom: 1px solid rgba(0,0,0,0.1);
    background: var(--background-color, #ffffff);
  }

  .header-title {
    font-weight: 600;
    font-size: 0.95em;
    opacity: 0.85;
  }

  .badge {
    font-size: 0.75em;
    padding: 3px 8px;
    border-radius: 12px;
    background: rgba(100, 108, 255, 0.15);
    color: #646cff;
    font-weight: 600;
  }

  .chat-viewport {
    flex: 1;
    overflow-y: auto;
    padding: 20px 0;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .message-list {
    width: 100%;
    max-width: 720px;
    padding: 0 20px;
    display: flex;
    flex-direction: column;
    gap: 24px;
  }

  .message-row {
    display: flex;
    width: 100%;
  }

  .message-row.user {
    justify-content: flex-end;
  }

  .message-row.agent {
    justify-content: flex-start;
  }

  .bubble {
    max-width: 85%;
    padding: 16px 20px;
    border-radius: 12px;
    box-shadow: 0 1px 3px rgba(0,0,0,0.05);
    line-height: 1.5;
  }

  .bubble.user {
    background-color: #646cff;
    color: #ffffff;
    border-radius: 16px 16px 2px 16px;
  }

  .bubble.agent {
    background-color: rgba(0,0,0,0.03);
    border: 1px solid rgba(0,0,0,0.05);
    color: var(--color, #0f0f0f);
    border-radius: 16px 16px 16px 2px;
  }

  .message-row.loading {
    opacity: 0.7;
  }

  .sender-label {
    font-size: 0.75em;
    font-weight: 700;
    margin-bottom: 4px;
    opacity: 0.5;
  }

  .message-text {
    font-size: 0.95em;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .footer {
    padding: 20px;
    display: flex;
    justify-content: center;
    border-top: 1px solid rgba(0,0,0,0.05);
    background: var(--background-color, #f6f6f6);
  }

  .input-form {
    display: flex;
    width: 100%;
    max-width: 720px;
    gap: 10px;
  }

  .text-input {
    flex: 1;
    padding: 14px 18px;
    border-radius: 24px;
    border: 1px solid rgba(0,0,0,0.15);
    font-size: 0.95em;
  }

  .send-button {
    border-radius: 24px;
    padding: 0 24px;
    background-color: #646cff;
    color: #ffffff;
    border: none;
    cursor: pointer;
  }

  .send-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .loading-dots {
    font-size: 0.9em;
    font-style: italic;
  }
</style>
