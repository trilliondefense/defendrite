
<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { page } from "$app/state";

  interface Message {
    role: string;
    content: string;
  }

  let messages = $state<Message[]>([]);
  let input = $state("");
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  const conversationId = $derived(page.params.id);

  // Load selected conversation.
  onMount(() => {
    loadMessages();
  });

  async function loadMessages() {
    try {
      error = null;

      messages = await invoke<Message[]>("get_chat_messages", {
        conversationId
      });

    } catch (err) {
      console.error("Failed to load conversation:", err);
      error = "Could not load this conversation.";
    }
  }

  // Send a message to the existing conversation.
  async function sendMessage(event: SubmitEvent) {
    event.preventDefault();

    const prompt = input.trim();

    if (!prompt || isLoading) return;

    isLoading = true;
    input = "";

    messages = [
      ...messages,
      { role: "user", content: prompt },
      { role: "model", content: "Generating response..." }
    ];

    try {
      await invoke("ask_agent", {
        prompt,
        conversationId
      });

      await loadMessages();

    } catch (err) {
      console.error("Agent request failed:", err);
      error = String(err);

    } finally {
      isLoading = false;
    }
  }
</script>

<svelte:head>
  <title>Chat | Harness</title>
</svelte:head>

<div class="chat-container">

  <header>
    <h2>Conversation</h2>
    <p>{conversationId}</p>
  </header>

  <main>
    {#if error}
      <p class="error">{error}</p>
    {/if}

    {#each messages as message, i}
      <div class:agent={message.role === "model"} class="message">
        <strong>
          {message.role === "user" ? "You" : "Assistant"}
        </strong>

        <p>{message.content}</p>
      </div>
    {/each}
  </main>

  <form onsubmit={sendMessage}>
    <input
      bind:value={input}
      placeholder="Continue this conversation..."
      disabled={isLoading}
    />

    <button type="submit" disabled={isLoading || !input.trim()}>
      {isLoading ? "Sending..." : "Send"}
    </button>
  </form>

</div>

<style>
  .chat-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 20px;
  }

  header {
    border-bottom: 1px solid var(--border-color, #444);
    padding-bottom: 12px;
  }

  header p {
    font-size: 12px;
    opacity: 0.5;
  }

  main {
    flex: 1;
    overflow-y: auto;
    padding: 20px 0;
  }

  .message {
    padding: 14px;
    margin-bottom: 12px;
    border-radius: 10px;
    background: var(--message-bg, #fff);
  }

  .message.agent {
    background: var(--agent-message-bg, #fff);
  }

  .message p {
    white-space: pre-wrap;
    overflow-wrap: anywhere;
  }

  form {
    display: flex;
    gap: 10px;
  }

  input {
    flex: 1;
    padding: 12px;
  }

  button {
    padding: 12px 20px;
    cursor: pointer;
  }

  .error {
    color: #ef4444;
  }
</style>
