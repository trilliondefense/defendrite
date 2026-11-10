
<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import { onMount, tick } from "svelte";
import { open } from '@tauri-apps/plugin-dialog';
import { stat } from "@tauri-apps/plugin-fs";

  
  import NewChat from "../../../lib/components/NewChat.svelte";
  import ContinueChat from "../../../lib/components/ContinueChat.svelte";

interface Message {
  id: string;
  sender: "user" | "agent";
  text: string;
}


  interface Attachment {
    id: string;
    name: string;
    path: string;
    size: number;
    mimeType: string | null;
  }

  let attachments = $state<Attachment[]>([]);
  let isSelecting = $state(false);

  async function addAttachments() {
    if (isSelecting) return;

    try {
      isSelecting = true;

      const selected = await open({
        multiple: true,
        directory: false,
        title: 'Select attachments'
      });

      if (!selected) return;

      const paths = Array.isArray(selected)
        ? selected
        : [selected];

      const newAttachments = await Promise.all(
        paths
          .filter(path =>
            !attachments.some(item => item.path === path)
          )
          .map(async (path) => {
            const metadata = await stat(path);

            return {
              id: crypto.randomUUID(),
              name: path.split(/[\\/]/).pop() || "Unknown file",
              path,
              size: metadata.size,
              mimeType: null
            };
          })
      );

      attachments = [...attachments, ...newAttachments];

    } catch (error) {
      console.error('Failed to select attachments:', error);
      consoleLogTerminal(`Message ${error}`)

    } finally {
      isSelecting = false;
    }
  }

  async function consoleLogTerminal(message:String){
     await invoke("log_to_terminal", {
        message
      });
  }

  function removeAttachment(id: string) {
    attachments = attachments.filter(
      attachment => attachment.id !== id
    );
  }

interface Conversation {
  id: string;
  title: string;
  chatNumber: number;
  createdAt: number;
  updatedAt: number;
}

// --------------------------------------------------
// STATE
// --------------------------------------------------

let messages = $state<Message[]>([]);

let conversationId = $state<string | null>(null);
let modelName = $state<string | null>(null);

let bottomElement = $state<HTMLDivElement>();
let viewportElement = $state<HTMLElement | null>(null);

let input = $state("");
let isLoading = $state(false);

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

  messages = [];

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
// GLOBAL STREAMING EVENT LISTENER
// --------------------------------------------------

onMount(() => {
  let disposed = false;
  let unlisten: (() => void) | undefined;

  // Read model preference.
  modelName = localStorage.getItem("modelName");

  // Listen for Gemini streaming tokens.
  listen<string>("agent-token", (event) => {
    if (disposed || !activeAgentMsgId) return;

    const messageId = activeAgentMsgId;

    // Append token to the active assistant message.
    messages = messages.map((message) =>
      message.id === messageId
        ? {
            ...message,
            text: message.text + event.payload
          }
        : message
    );

    void scrollToBottom();
  })
    .then((unsubscribe) => {
      if (disposed) {
        unsubscribe();
      } else {
        unlisten = unsubscribe;
      }
    })
    .catch((error) => {
      console.error(
        "Failed to register agent listener:",
        error
      );
    });

  // Register keyboard shortcuts.
  window.addEventListener(
    "keydown",
    handleGlobalKeydown
  );

  focusInput();

  // Cleanup.
  return () => {
    disposed = true;

    unlisten?.();

    window.removeEventListener(
      "keydown",
      handleGlobalKeydown
    );
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

  if ((!prompt && attachments.length === 0) || isLoading) return;

  isLoading = true;
  input = "";

  // Snapshot attachments for this message.
  const messageAttachments = [...attachments];

  // Clear selected attachments.
  attachments = [];

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
    const chatId = await ensureConversation();

    await invoke("ask_agent", {
      prompt,
      conversationId: chatId,
      attachments: messageAttachments
    });

    await emit("chat-created", {});

  } catch (error) {
    const errorString = String(error);

    const errorMessage = errorString.includes("SETUP_REQUIRED")
      ? "⚠️ Workspace Configuration Required: Please navigate to Settings and specify your Google AI Studio API Key."
      : `Runtime Execution Failure: ${errorString}`;

    messages = messages.map((message) =>
      message.id === agentMessageId
        ? { ...message, text: errorMessage }
        : message
    );

    // Restore attachments if sending fails.
    attachments = [
      ...messageAttachments,
      ...attachments
    ];

  } finally {
    activeAgentMsgId = null;
    isLoading = false;

    void scrollToBottom();
    focusInput();
  }
}


</script>

<div class="chat-page">
  {#if messages.length === 0}
    <NewChat {handleSendMessage} bind:inputElement bind:input 
      bind:modelName {isLoading} {isSelecting} {addAttachments}
      {removeAttachment} {attachments}
    />  
   
  {:else}
   <ContinueChat {handleSendMessage} bind:inputElement bind:input 
      bind:modelName {isLoading} {isSelecting} {addAttachments} {activeAgentMsgId} 
      {removeAttachment} {attachments} {messages} {viewportElement} bind:bottomElement
    />
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

</style>