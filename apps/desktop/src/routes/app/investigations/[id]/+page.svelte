
<script lang="ts">
  import { onMount, tick } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { page } from "$app/state";
  import { marked } from "marked";
import DOMPurify from "dompurify";
function renderMarkdown(content: string) {
  return DOMPurify.sanitize(marked.parse(content) as string);
}

  interface Message {
    role: string;
    content: string;
  }

  const conversationId = $derived(page.params.id);

  let messages = $state<Message[]>([]);
  let input = $state("");
  let isLoading = $state(false);
  let error = $state<string | null>(null);

  let modelName = $state<string | null>(null);

  let bottomElement = $state<HTMLDivElement | null>(null);
  let inputElement = $state<HTMLTextAreaElement | null>(null);

  // --------------------------------------------------
  // LOAD MESSAGES
  // --------------------------------------------------

  async function loadMessages() {
    try {
      error = null;

      messages = await invoke<Message[]>("get_chat_messages", {
        conversationId
      });

      await scrollToBottom();

    } catch (err) {
      console.error("Failed to load conversation:", err);
      error = "Could not load this conversation.";
    }
  }

  // --------------------------------------------------
  // SCROLLING
  // --------------------------------------------------

  async function scrollToBottom() {
    await tick();

    bottomElement?.scrollIntoView({
      behavior: "smooth",
      block: "end"
    });
  }


  function focusInput() {
    inputElement?.focus();
  }

  function handleGlobalKeydown(event: KeyboardEvent) {
    const modifier = event.metaKey || event.ctrlKey;
    const target = event.target;

    const typing =
      target instanceof HTMLInputElement ||
      target instanceof HTMLTextAreaElement ||
      (target instanceof HTMLElement && target.isContentEditable);

    // Escape: focus chat input
    if (event.key === "Escape" && !typing) {
      focusInput();
    }

    // Slash: focus chat input
    if (
      event.key === "/" &&
      !modifier &&
      !event.altKey &&
      !typing
    ) {
      event.preventDefault();
      focusInput();
    }
  }

  // --------------------------------------------------
  // SEND MESSAGE
  // --------------------------------------------------

  async function handleSendMessage(event: SubmitEvent) {
    event.preventDefault();

    const prompt = input.trim();

    if (!prompt || isLoading) return;

    isLoading = true;
    error = null;
    input = "";

    // Immediately display user message and loading placeholder.
    messages = [
      ...messages,
      { role: "user", content: prompt },
      { role: "model", content: "Generating response..." }
    ];

    await scrollToBottom();

    try {
      // Send prompt to existing conversation.
      await invoke("ask_agent", {
        prompt,
        conversationId
      });

      // Retrieve updated conversation from Rust backend.
      await loadMessages();

    } catch (err) {
      console.error("Agent request failed:", err);

      error = String(err);

      // Replace loading placeholder with error.
      messages = messages.map((message, index) =>
        index === messages.length - 1 &&
        message.role === "model" &&
        message.content === "Generating response..."
          ? {
              role: "model",
              content: String(err)
            }
          : message
      );

    } finally {
      isLoading = false;

      await scrollToBottom();
      focusInput();
    }
  }

  onMount(() => {
    let disposed = false;

    // Read saved model preference.
    modelName = localStorage.getItem("modelName");

    // Load conversation.
    void loadMessages();

    // Global keyboard shortcuts.
    window.addEventListener("keydown", handleGlobalKeydown);

    // Focus input after mounting.
    void tick().then(() => {
      if (!disposed) {
        focusInput();
      }
    });

    // Cleanup.
    return () => {
      disposed = true;

      window.removeEventListener(
        "keydown",
        handleGlobalKeydown
      );
    };
  });
</script>


<div class="chat-page">
  {#if messages.length === 0}
    <main class="chat-home">
    </main>
  {:else}
<main class="chat-conversation">
  <div class="message-list">
    {#each messages as msg, i (`${msg.role}-${i}`)}
      <div
        class="message-row"
        class:user={msg.role === "user"}
        class:agent={msg.role === "model"}
      >
        {#if msg.role === "model"}
          <img src="/favicon.png" alt="" class="message-avatar" />
        {/if}

        <div class="message-content">
          <div class="sender-label">
            {msg.role === "user" ? "You" : "Assistant"}
          </div>

          <div class="message-text">
            {#if msg.content === "Generating response..." && isLoading}
              <span class="thinking-indicator">
                <span></span>
                <span></span>
                <span></span>
              </span>
            {:else}
            {@html renderMarkdown(msg.content)}
            {/if}
          </div>
        </div>
      </div>
    {/each}

    <div bind:this={bottomElement} class="scroll-anchor"></div>
  </div>

  <footer class="conversation-footer">
    <form class="composer" onsubmit={handleSendMessage}>
      <textarea
        bind:this={inputElement}
        bind:value={input}
        placeholder="How can I help you today?"
        rows="2"
        aria-label="Message your AI assistant"
        disabled={isLoading}
        onkeydown={(event) => {
          if (
            event.key === "Enter" &&
            !event.shiftKey &&
            !event.isComposing
          ) {
            event.preventDefault();

            if (!isLoading && input.trim()) {
              event.currentTarget.form?.requestSubmit();
            }
          }
        }}
      ></textarea>

      <div class="composer-bottom">
        <button
          class="add-button"
          type="button"
          aria-label="Add attachment"
        >
          +
        </button>

        <button
          type="submit"
          class="send-button"
          disabled={isLoading || !input.trim()}
          aria-label="Send message"
        >
          {#if isLoading}
            …
          {:else}
            ↑
          {/if}
        </button>
      </div>
    </form>

    <p class="disclaimer">
      AI can make mistakes. Review important information.
    </p>
  </footer>
</main>

  {/if}
</div>

<style>

.chat-page {
  display: flex;
  flex: 1;
  flex-direction: column;
  width: 100%;

  height: 100%;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
  color: #383731;
  background: #faf9f6;
}

.chat-home {
  display: flex;
  flex: 1;
  flex-direction: column;
  align-items: center;
  min-width: 0;
  min-height: 0;
  overflow-y: auto;
  padding: clamp(75px, 25.5vh, 135px) 24px 36px;
}




.composer {
  padding: 12px 13px 9px;
  border: 1px solid #e5e3dc;
  border-radius: 17px;
  background: #fff;
  box-shadow: 0 4px 12px #00000008;
  transition: border-color 160ms, box-shadow 160ms;
}

.composer:focus-within {
  border-color: #d9b8a7;
  box-shadow: 0 0 0 3px #d7836210, 0 4px 12px #00000008;
}

.composer textarea {
  display: block;
  width: 100%;
  min-height: 35px;
  max-height: 220px;
  resize: vertical;
  border: 0;
  outline: 0;
  background: transparent;
  color: #383731;
  font-family: inherit;
  font-size: 12px;
  line-height: 1.6;
}

.composer textarea::placeholder {
  color: #89877f;
  opacity: 1;
}

.composer-bottom {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 25px;
}


.composer button {
  display: grid;
  place-items: center;
  padding: 3px;
  border: 0;
  border-radius: 5px;
  background: transparent;
  color: #65645d;
  cursor: pointer;
}

.composer button:hover {
  background: #f4f2ed;
}

.add-button {
  font-size: 19px;
  line-height: 1;
}

/* Existing chat after sending a message */

.chat-conversation {
  display: flex;
  flex: 1;
  flex-direction: column;
  min-width: 0;
  min-height: 0;
  overflow: hidden;
}

.message-list {
  flex: 1;
  width: min(100% - 40px, 700px);
  margin: 0 auto;
  padding: 28px 0;
  overflow-y: auto;
}

.message-row {
  display: flex;
  align-items: flex-start;
  gap: 11px;
  margin-bottom: 25px;
}

.message-row.user {
  justify-content: flex-end;
}

.message-avatar {
  width: 27px;
  height: 27px;
  object-fit: contain;
}

.message-content {
  max-width: 85%;
  min-width: 0;
}

.sender-label {
  margin-bottom: 6px;
  color: #89877f;
  font-size: 11px;
}

.message-text {
  color: #383731;
  font-size: 13px;
  line-height: 1.75;
  overflow-wrap: anywhere;
  white-space: pre-wrap;
}

.message-row.user .message-text {
  padding: 10px 13px;
  border-radius: 13px 13px 3px 13px;
  background: #f0eee7;
}

.conversation-footer {
  width: min(100%, 700px);
  margin: 0 auto;
  padding: 12px 20px 18px;
}

.disclaimer {
  margin: 11px 0 0;
  color: #a09b91;
  font-size: 10px;
  text-align: center;
}

.send-button {
  width: 30px;
  height: 30px;
  border-radius: 8px !important;
  background: #ce876b !important;
  color: white !important;
  font-size: 20px;
}
.chat-conversation {
  width: 100%;
  flex: 1;
  min-width: 0;
}

.message-list {
  width: 100%;
  max-width: none;
  margin: 0;
  padding-inline: 64px;
}
.message-text :global(p) {
  margin: 0 0 6px;
  line-height: 1.5;
}

.message-text :global(p:last-child) {
  margin-bottom: 0;
}

.message-text :global(ul),
.message-text :global(ol) {
  margin-top: 4px;
  margin-bottom: 6px;
}
/* Agent messages - left aligned, 80% width */
.message-row.agent {
  width: 80%;
  margin-right: auto;
  justify-content: flex-start;
}

/* User messages - right aligned, 40% width */
.message-row.user {
  width: 40%;
  margin-left: auto;
  justify-content: flex-end;
}

/* Message content */
.message-content {
  width: 100%;
  min-width: 0;
}

/* Prevent content overflow */
.message-text {
  overflow-wrap: anywhere;
}

.message-text :global(li) {
  margin-bottom: 2px;
}

.message-row {
  width: 100%;
}

.message-content {
  max-width: 100%;
  flex: 1;
}

.send-button:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.thinking-indicator {
  display: inline-flex;
  gap: 5px;
  padding: 8px 0;
}

.thinking-indicator span {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #d18b6e;
  animation: thinking 1s infinite ease-in-out;
}


.thinking-indicator span:nth-child(2) {
  animation-delay: 150ms;
}

.thinking-indicator span:nth-child(3) {
  animation-delay: 300ms;
}

@keyframes thinking {
  0%, 60%, 100% {
    opacity: 0.4;
    transform: translateY(0);
  }
  30% {
    opacity: 1;
    transform: translateY(-4px);
  }
}

@media (max-width: 560px) {
  .chat-home {
    padding: 70px 16px 30px;
  }


  .message-list {
    width: calc(100% - 24px);
  }

  .conversation-footer {
    padding: 10px 12px max(12px, env(safe-area-inset-bottom));
  }
}

</style>