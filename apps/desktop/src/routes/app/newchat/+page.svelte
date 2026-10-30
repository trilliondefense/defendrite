
<script lang="ts">
  import { goto } from "$app/navigation";


import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import { onMount, tick } from "svelte";

  import { marked } from "marked";
  import DOMPurify from "dompurify";
  function renderMarkdown(content: string) {
    return DOMPurify.sanitize(marked.parse(content) as string);
  }

interface Message {
  id: string;
  sender: "user" | "agent";
  text: string;
}

interface Conversation {
  id: string;
  title: string;
  chatNumber: number;
  createdAt: number;
  updatedAt: number;
}

let messages = $state<Message[]>([
  
]);

let conversationId = $state<string | null>(null);
let modelName = $state<string | null>(null);

let bottomElement = $state<HTMLDivElement>();
let input = $state("");
let isLoading = $state(false);
  let viewportElement = $state<HTMLElement | null>(null);

let inputElement = $state<HTMLTextAreaElement>();
let activeAgentMsgId = $state<string | null>(null);
  

// --------------------------------------------------
// SCROLLING
// --------------------------------------------------

async function scrollToBottom() {
  await tick();

  requestAnimationFrame(() => {
    bottomElement?.scrollIntoView({
      behavior: "smooth",
      block: "end"
    });
  });
}

// --------------------------------------------------
// INPUT FOCUS
// --------------------------------------------------

function focusInput() {
  inputElement?.focus();
}

// --------------------------------------------------
// NEW CHAT
// --------------------------------------------------

function newChat() {
  if (isLoading) return;

  conversationId = null;

  messages = [
    
  ];

  activeAgentMsgId = null;
  input = "";

  focusInput();
}

// --------------------------------------------------
// GLOBAL KEYBOARD SHORTCUTS
// --------------------------------------------------

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
// EVENT LISTENER INITIALIZATION
// --------------------------------------------------

onMount(() => {
  let disposed = false;
  let unlisten: (() => void) | undefined;

  // Read model preference safely in the browser.
  modelName = localStorage.getItem("modelName");

  listen<string>("agent-token", (event) => {
    if (disposed || !activeAgentMsgId) return;

    const activeMessage = messages.find(
      (message) => message.id === activeAgentMsgId
    );

    if (activeMessage) {
      activeMessage.text += event.payload;
      void scrollToBottom();
    }
  })
    .then((unsubscribe) => {
      if (disposed) {
        unsubscribe();
      } else {
        unlisten = unsubscribe;
      }
    })
    .catch((error) => {
      console.error("Failed to register agent listener:", error);
    });

  window.addEventListener("keydown", handleGlobalKeydown);

  focusInput();

  return () => {
    disposed = true;
    unlisten?.();
    window.removeEventListener("keydown", handleGlobalKeydown);
  };
});

// --------------------------------------------------
// CREATE CONVERSATION WHEN NEEDED
// --------------------------------------------------

async function ensureConversation(): Promise<string> {
  if (conversationId) {
    return conversationId;
  }

  const chat = await invoke<Conversation>("create_chat");

  conversationId = chat.id;

  return chat.id;
}

// --------------------------------------------------
// SEND MESSAGE
// --------------------------------------------------

async function handleSendMessage(event: SubmitEvent) {
  event.preventDefault();

  const prompt = input.trim();

  if (!prompt || isLoading) return;

  isLoading = true;
  input = "";

  const userMessage: Message = {
    id: crypto.randomUUID(),
    sender: "user",
    text: prompt
  };

  const agentMessageId = crypto.randomUUID();

  messages = [
    ...messages,
    userMessage,
    {
      id: agentMessageId,
      sender: "agent",
      text: ""
    }
  ];

  activeAgentMsgId = agentMessageId;

  void scrollToBottom();

  try {
    // Create a conversation only for the first message.
    const chatId = await ensureConversation();

    // Send the prompt to the selected conversation.
    await invoke("ask_agent", {
      prompt,
      conversationId: chatId
    });
    await emit('chat-created', {}); 

  } catch (error) {
    const errorString = String(error);

    const targetIdx = messages.findIndex(
      (message) => message.id === agentMessageId
    );

    if (targetIdx !== -1) {
      messages[targetIdx].text = errorString.includes("SETUP_REQUIRED")
        ? "⚠️ Workspace Configuration Required: Please navigate to Settings and specify your Google AI Studio API Key."
        : `Runtime Execution Failure: ${errorString}`;
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
  {#if messages.length === 0}
    <main class="chat-home">

      <section class="welcome">
        <h1>
          <img src="/favicon.png" alt="" class="message-avatar" />
          Start new Investigation 
        </h1>

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
                event.currentTarget.form?.requestSubmit();
              }
            }}
          ></textarea>

          <div class="composer-bottom">
            <button
              class="add-button"
              type="button"
              aria-label="Add attachment"
              title="Add attachment"
              onclick={() => {
                // Connect your existing attachment handler here.
              }}
            >+</button>

            <div class="composer-actions">
              {#if modelName && modelName !==""}
                <select aria-label="Select model" value="{modelName}">
                  <option>{modelName}</option>
                </select>
              {/if}

              <button
                class="mic-button"
                type="button"
                aria-label="Voice input"
                title="Voice input"
                onclick={() => {
                  // Connect your existing voice input handler here.
                }}
              >
                <svg viewBox="0 0 24 24" width="17" height="17"
                  fill="none" stroke="currentColor" stroke-width="1.6"
                  stroke-linecap="round" stroke-linejoin="round">
                  <rect x="9" y="3" width="6" height="12" rx="3" />
                  <path d="M5.5 11a6.5 6.5 0 0 0 13 0" />
                  <path d="M12 17.5V21M9 21h6" />
                </svg>
              </button>
            </div>
          </div>
        </form>

        <button
          class="connect-tools"
          type="button"
          onclick={() => goto("/app/customize")}
        >
          <span>Connect your tools</span>
          <span class="tool-icons"></span>
          <span class="chevron">›</span>
        </button>
      </section>
    </main>
  {:else}
    <main class="chat-conversation" bind:this={viewportElement}>
      <div class="message-list">
        {#each messages as msg (msg.id)}
          <div
            class="message-row"
            class:user={msg.sender === "user"}
            class:agent={msg.sender === "agent"}
          >
            {#if msg.sender === "agent"}
              <img src="/favicon.png" alt="" class="message-avatar" />
            {/if}

            <div class="message-content">
              <div class="sender-label">
                {msg.sender === "user" ? "You" : "Assistant"}
              </div>

              <div class="message-text">
                {#if msg.text}
            {@html renderMarkdown(msg.text)}
                {:else if isLoading && msg.id === activeAgentMsgId}
                  <span class="thinking-indicator">
                    <span></span><span></span><span></span>
                  </span>
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
            <button class="add-button" type="button" aria-label="Add attachment">
              +
            </button>

            <button
              type="submit"
              class="send-button"
              disabled={isLoading || !input.trim()}
              aria-label="Send message"
            >
              {#if isLoading}…{:else}↑{/if}
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




.welcome {
  width: 100%;
  max-width: 444px;
  margin-top: 25px;
}

.welcome h1 {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 9px;
  margin: 0 0 25px;
  font-family: Georgia, "Times New Roman", serif;
  font-size: clamp(22px, 3vw, 28px);
  font-weight: 400;
  letter-spacing: -1px;
  white-space: nowrap;
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

.composer-actions {
  display: flex;
  align-items: center;
  gap: 10px;
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

.composer-actions select {
  max-width: 110px;
  border: 0;
  outline: 0;
  background: transparent;
  color: #55544e;
  font-size: 10px;
  cursor: pointer;
  padding: 2px 14px 2px 2px;
  appearance: auto;
}

/* Styles the actual dropdown overlay menu */
.composer-actions select option {
  background-color: #ffffff; /* Prevents transparent/see-through menus */
  color: #333332;            /* Darker, high-contrast text for small font sizes */
  font-size: 11px;           /* Slightly larger for readability inside the list */
  padding: 4px;              /* Gives options breathing room (supported in Chromium) */
}



.mic-button {
  width: 22px;
  height: 24px;
}

.connect-tools {
  display: flex;
  align-items: center;
  gap: 7px;
  width: 100%;
  margin-top: 3px;
  padding: 3px 14px;
  border: 0;
  border-radius: 5px;
  background: transparent;
  color: #55544e;
  font-size: 10px;
  text-align: left;
  cursor: pointer;
}

.connect-tools:hover {
  background: #f1f0eb;
}

.tool-icons {
  margin-left: auto;
  color: #77766f;
  font-size: 9px;
  letter-spacing: 1px;
}

.chevron {
  color: #89877f;
  font-size: 17px;
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

  .welcome {
    margin-top: 23px;
  }

  .welcome h1 {
    gap: 6px;
    font-size: clamp(19px, 5vw, 25px);
    white-space: normal;
  }

  .tool-icons {
    display: none;
  }

  .message-list {
    width: calc(100% - 24px);
  }

  .conversation-footer {
    padding: 10px 12px max(12px, env(safe-area-inset-bottom));
  }
}

</style>