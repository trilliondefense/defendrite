<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { emit, listen } from '@tauri-apps/api/event';
  import { onMount, tick } from 'svelte';
  import { open } from '@tauri-apps/plugin-dialog';
  import { stat } from '@tauri-apps/plugin-fs';
  import { page } from '$app/state';
  import type { Message, DraftAttachment, Attachment } from '#lib/utils/types.ts';
  import ContinueChat from '#lib/components/ContinueChat.svelte';

  const conversationId = $derived(page.params.id ?? '');
  let error = $state<string | null>(null);
  $effect(() => {
    // Explicitly read conversationId here so Svelte tracks it as a dependency.
    // Every time the URL changes to a new UUID, this effect reruns.
    const activeId = conversationId;

    if (activeId) {
      loadMessages(activeId);

      // Proactively focus input when changing conversations
      void tick().then(() => {
        focusInput();
      });
    }
  });

  async function loadMessages(targetId = conversationId) {
    try {
      error = null;

      // Pass targetId explicitly to avoid state race conditions during quick clicks
      messages = await invoke<Message[]>('get_chat_messages', {
        conversationId: targetId,
      });
      consoleLogTerminal(messages.join(','));

      await scrollToBottom();
    } catch (err) {
      console.error('Failed to load conversation:', err);
      consoleLogTerminal(`Failed to load conversation: ${err}`);

      error = 'Could not load this conversation.';
    }
  }

  let attachments = $state<DraftAttachment[]>([]);
  let isSelecting = $state(false);

  async function addAttachments() {
    if (isSelecting) return;
    const now = Math.floor(Date.now() / 1000);

    try {
      isSelecting = true;

      const selected = await open({
        multiple: true,
        directory: false,
        title: 'Select attachments',
      });

      if (!selected) return;

      const paths = Array.isArray(selected) ? selected : [selected];

      const newAttachments: Attachment[] = await Promise.all(
        paths
          .filter((path) => !attachments.some((item) => item.path === path))
          .map(async (path) => {
            const metadata = await stat(path);

            return {
              id: crypto.randomUUID(),
              messageId: 0,
              name: path.split(/[\\/]/).pop() || 'Unknown file',
              path,
              createdAt: now,
              size: metadata.size,
              mimeType: null,
            };
          })
      );

      attachments = [...attachments, ...newAttachments];
    } catch (error) {
      console.error('Failed to select attachments:', error);
      consoleLogTerminal(`Message ${error}`);
    } finally {
      isSelecting = false;
    }
  }

  async function consoleLogTerminal(message: String) {
    await invoke('log_to_terminal', {
      message,
    });
  }

  function removeAttachment(id: string) {
    attachments = attachments.filter((attachment) => attachment.id !== id);
  }

  // --------------------------------------------------
  // STATE
  // --------------------------------------------------

  let messages = $state<Message[]>([]);

  let modelName = $state<string | null>(null);

  let bottomElement = $state<HTMLDivElement>();
  let viewportElement = $state<HTMLElement | null>(null);

  let input = $state('');
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
        behavior: 'smooth',
        block: 'end',
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

    if (event.key === 'Escape' && !typing) {
      focusInput();
    }

    if (event.key === '/' && !modifier && !event.altKey && !typing) {
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
    modelName = localStorage.getItem('modelName');

    // Listen for Gemini streaming tokens.
    listen<string>('agent-token', (event) => {
      if (disposed || !activeAgentMsgId) return;

      const messageId = activeAgentMsgId;

      messages = messages.map((message) =>
        message.id === messageId
          ? {
              ...message,
              content: message.content + event.payload,
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
        console.error('Failed to register agent listener:', error);
      });

    // Register keyboard shortcuts.
    window.addEventListener('keydown', handleGlobalKeydown);

    focusInput();

    // Cleanup.
    return () => {
      disposed = true;

      unlisten?.();

      window.removeEventListener('keydown', handleGlobalKeydown);
    };
  });

  // --------------------------------------------------
  // CREATE CONVERSATION WHEN NEEDED

  // --------------------------------------------------
  // SEND MESSAGE
  // --------------------------------------------------
  async function handleSendMessage(event: SubmitEvent) {
    event.preventDefault();

    if (!conversationId) return;
    const prompt = input.trim();

    if ((!prompt && attachments.length === 0) || isLoading) return;

    isLoading = true;
    input = '';

    // Snapshot attachments.
    const messageAttachments = [...attachments];

    // Clear selected attachments.
    attachments = [];

    const now = Math.floor(Date.now() / 1000);

    const userMessageId = crypto.randomUUID();
    const agentMessageId = crypto.randomUUID();

    const userMessage: Message = {
      id: userMessageId,
      conversationId,
      role: 'user',
      content: prompt,
      createdAt: now,
      attachments: messageAttachments.map((attachment) => ({
        ...attachment,
        messageId: 0,
        createdAt: now,
      })),
    };

    const agentMessage: Message = {
      id: agentMessageId,
      conversationId,
      role: 'model',
      content: '',
      createdAt: now,
      attachments: [],
    };

    messages = [...messages, userMessage, agentMessage];

    activeAgentMsgId = agentMessageId;

    void scrollToBottom();

    try {
      const chatId = conversationId;

      await invoke('ask_agent', {
        prompt,
        conversationId: chatId,
        attachments: messageAttachments,
      });

      await emit('chat-created', {});
    } catch (error) {
      const errorString = String(error);

      const errorMessage = errorString.includes('SETUP_REQUIRED')
        ? '⚠️ Workspace Configuration Required: Please navigate to Settings and specify your Google AI Studio API Key.'
        : `Runtime Execution Failure: ${errorString}`;

      messages = messages.map((message) =>
        message.id === agentMessageId ? { ...message, content: errorMessage } : message
      );

      // Restore attachments if sending fails.
      attachments = [...messageAttachments, ...attachments];
    } finally {
      activeAgentMsgId = null;
      isLoading = false;

      void scrollToBottom();
      focusInput();
    }
  }
</script>

{#key page.url.pathname}
  <div class="chat-page">
    {#if messages.length === 0}
      <main class="chat-home"></main>
    {:else}
      <ContinueChat
        {handleSendMessage}
        bind:inputElement
        bind:input
        bind:modelName
        {isLoading}
        {isSelecting}
        {addAttachments}
        {activeAgentMsgId}
        {removeAttachment}
        {attachments}
        {messages}
        {viewportElement}
        bind:bottomElement
      />
    {/if}
  </div>
{/key}

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
</style>
