<script lang="ts">
  import "./global.css"
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
