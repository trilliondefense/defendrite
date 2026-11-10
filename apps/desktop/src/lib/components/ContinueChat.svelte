<script lang="ts">
  import { marked } from 'marked';
  import DOMPurify from 'dompurify';
  import type { Attachment, DraftAttachment, Message } from '#lib/utils/types.ts';
  import { convertFileSrc } from '@tauri-apps/api/core';
  import { formatFileSize } from '#lib/utils/functions.ts';
  function renderMarkdown(content: string) {
    return DOMPurify.sanitize(marked.parse(content) as string);
  }

  interface Props {
    handleSendMessage: (event: SubmitEvent) => void;
    inputElement?: HTMLTextAreaElement; // Marked optional until element mounts
    input: string;
    isLoading: boolean;
    isSelecting: boolean;
    modelName?: string | null;
    viewportElement?: HTMLElement | null;
    messages: Message[];
    activeAgentMsgId: string | null;
    addAttachments: () => void;
    attachments: DraftAttachment[];
    bottomElement?: HTMLDivElement;
    removeAttachment: (id: string) => void;
  }

  let {
    handleSendMessage,
    bottomElement = $bindable(),
    viewportElement = $bindable(),
    activeAgentMsgId,
    inputElement = $bindable(),
    modelName = $bindable(),
    input = $bindable(),
    isLoading,
    isSelecting,
    messages,
    attachments,
    removeAttachment,
    addAttachments,
  }: Props = $props();
</script>

<main class="chat-conversation" bind:this={viewportElement}>
  <div class="message-list">
    {#each messages as msg (msg.id)}
      <div class="message-row" class:user={msg.role === 'user'} class:agent={msg.role === 'model'}>
        {#if msg.role === 'model'}
          <img src="/favicon.png" alt="Agent" class="message-avatar" />
        {/if}

        <div class="message-content">
          <div class="sender-label">
            {msg.role === 'user' ? 'You' : 'Agent'}
          </div>

          <!-- Attachments -->
          {#if msg.attachments?.length > 0}
            <div class="message-attachments">
              {#each msg.attachments as attachment (attachment.id)}
                <div class="attachment-card">
                  {#if attachment.mimeType?.startsWith('image/')}
                    <div class="attachment-image">
                      <img
                        src={convertFileSrc(attachment.path)}
                        alt={attachment.name}
                        loading="lazy"
                      />
                    </div>
                  {:else}
                    <div class="attachment-file-icon">
                      <svg
                        fill="#000000"
                        width="64px"
                        height="64px"
                        viewBox="0 0 32 32"
                        data-name="Layer 1"
                        id="Layer_1"
                        xmlns="http://www.w3.org/2000/svg"
                        ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                          id="SVGRepo_tracerCarrier"
                          stroke-linecap="round"
                          stroke-linejoin="round"
                        ></g><g id="SVGRepo_iconCarrier"
                          ><title></title><path
                            d="M25.69,2.9H11.93a2.7,2.7,0,0,0-2-.9A2.74,2.74,0,0,0,7.18,4.74v8.11a.94.94,0,0,0,1.87,0V4.74a.87.87,0,0,1,1.74,0v7.2a2.68,2.68,0,1,1-5.35,0V4.74a.93.93,0,1,0-1.86,0v7.2a4.52,4.52,0,0,0,4.05,4.49V27.26A2.74,2.74,0,0,0,10.37,30H25.69a2.74,2.74,0,0,0,2.73-2.74V5.64A2.74,2.74,0,0,0,25.69,2.9Zm.87,24.36a.87.87,0,0,1-.87.87H10.37a.87.87,0,0,1-.87-.87v-11a4.51,4.51,0,0,0,3.05-3.36H23.66a.94.94,0,0,0,0-1.87h-11V4.77h13a.87.87,0,0,1,.87.87Z"
                          ></path><path
                            d="M23.66,15.52H12.4a.93.93,0,1,0,0,1.86H23.66a.93.93,0,0,0,0-1.86Z"
                          ></path><path
                            d="M23.66,20H12.4a.94.94,0,1,0,0,1.87H23.66a.94.94,0,0,0,0-1.87Z"
                          ></path></g
                        ></svg
                      >
                    </div>

                    <div class="attachment-info">
                      <span class="attachment-name">
                        {attachment.name}
                      </span>
                      <span class="attachment-size">
                        {formatFileSize(attachment.size)}
                      </span>
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          {/if}

          <!-- Message text -->
          {#if msg.content}
            <div class="message-text">
              {@html renderMarkdown(msg.content)}
            </div>
          {:else if isLoading && msg.id === activeAgentMsgId}
            <div class="thinking-indicator">
              <span></span>
              <span></span>
              <span></span>
            </div>
          {/if}
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
          if (event.key === 'Enter' && !event.shiftKey && !event.isComposing) {
            event.preventDefault();

            if (!isLoading && input.trim()) {
              event.currentTarget.form?.requestSubmit();
            }
          }
        }}></textarea>

      <div class="composer-bottom">
        <button
          class="add-button"
          type="button"
          aria-label="Add attachment"
          title="Add attachment"
          disabled={isSelecting}
          onclick={addAttachments}
        >
          {isSelecting ? '...' : '+'}
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

    {#if attachments.length > 0}
      <div class="composer-attachments">
        {#each attachments as attachment (attachment.id)}
          <div class="composer-attachment">
            {#if attachment.mimeType?.startsWith('image/')}
              <div class="composer-attachment-preview">
                <img src={convertFileSrc(attachment.path)} alt={attachment.name} />
              </div>
            {:else}
              <div class="composer-file-icon">
                <svg
                  width="64px"
                  height="64px"
                  viewBox="0 0 24 24"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                  ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                    id="SVGRepo_tracerCarrier"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  ></g><g id="SVGRepo_iconCarrier"
                    ><path
                      d="M16.617 14.496a1 1 0 0 1 1.414 1.414l-3.182 3.182a7 7 0 1 1-9.9-9.9l5.658-5.656a5 5 0 1 1 7.07 7.07l-5.656 5.658a3 3 0 0 1-4.243-4.243l4.596-4.596a1 1 0 1 1 1.415 1.414l-4.597 4.596a1 1 0 1 0 1.415 1.414l5.656-5.657a3 3 0 1 0-4.242-4.242l-5.657 5.657a5 5 0 1 0 7.071 7.07l3.182-3.181z"
                      fill="#000000"
                    ></path></g
                  ></svg
                >
              </div>
            {/if}

            <div class="composer-attachment-info">
              <span class="composer-attachment-name" title={attachment.name}>
                {attachment.name}
              </span>

              <span class="composer-attachment-size">
                {formatFileSize(attachment.size)}
              </span>
            </div>

            <button
              type="button"
              class="composer-attachment-remove"
              aria-label={`Remove ${attachment.name}`}
              title="Remove attachment"
              onclick={() => removeAttachment(attachment.id)}
            >
              <svg
                width="64px"
                height="64px"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                ><g id="SVGRepo_bgCarrier" stroke-width="0"></g><g
                  id="SVGRepo_tracerCarrier"
                  stroke-linecap="round"
                  stroke-linejoin="round"
                ></g><g id="SVGRepo_iconCarrier">
                  <path
                    d="M19 5L4.99998 19M5.00001 5L19 19"
                    stroke="#000000"
                    stroke-width="1.5"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  ></path>
                </g></svg
              >
            </button>
          </div>
        {/each}
      </div>
    {/if}
    {#if attachments.length > 0}
      <div class="attachment-summary">
        <span>{attachments.length} files attached</span>
        <span>
          {formatFileSize(attachments.reduce((total, file) => total + file.size, 0))}
          total
        </span>
      </div>
    {/if}

    <p class="disclaimer">AI can make mistakes. Review important information.</p>
  </footer>
</main>

<style>
  .composer {
    padding: 12px 13px 9px;
    border: 1px solid #e5e3dc;
    border-radius: 17px;
    background: #fff;
    box-shadow: 0 4px 12px #00000008;
    transition:
      border-color 160ms,
      box-shadow 160ms;
  }

  .composer:focus-within {
    border-color: #d9b8a7;
    box-shadow:
      0 0 0 3px #d7836210,
      0 4px 12px #00000008;
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
  .chat-conversation {
    width: 100%;
    flex: 1;
    min-width: 0;
  }

  .attachment-summary {
    display: flex;
    justify-content: space-between;
    padding: 0 2px 6px;
    color: var(--text-muted, #89877f);
    font-size: 11px;
  }

  /* ================================
   COMPOSER ATTACHMENTS
================================ */

  .composer-attachments {
    display: flex;
    align-items: stretch;
    gap: 10px;
    width: 100%;
    padding: 10px 0 14px;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: thin;
    scrollbar-color: var(--border-color, #dedbd3) transparent;
  }

  .composer-attachments::-webkit-scrollbar {
    height: 4px;
  }

  .composer-attachments::-webkit-scrollbar-thumb {
    background: var(--border-color, #dedbd3);
    border-radius: 10px;
  }

  /* ================================
   ATTACHMENT CARD
================================ */

  .composer-attachment {
    position: relative;
    display: flex;
    align-items: center;
    gap: 10px;
    flex: 0 0 220px;
    min-width: 0;
    height: 68px;
    padding: 9px 34px 9px 9px;
    border: 1px solid var(--border-color, #e5e2da);
    border-radius: 12px;
    background: var(--attachment-bg, #faf9f6);
    transition:
      background 0.2s ease,
      border-color 0.2s ease,
      transform 0.2s ease;
  }

  .composer-attachment:hover {
    border-color: var(--accent-color, #aaa69a);
    background: var(--attachment-hover, #f5f3ed);
  }

  /* ================================
   IMAGE PREVIEW
================================ */

  .composer-attachment-preview {
    width: 48px;
    height: 48px;
    flex-shrink: 0;
    overflow: hidden;
    border-radius: 8px;
    background: var(--code-bg, #eeece6);
  }

  .composer-attachment-preview img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  /* ================================
   FILE ICON
================================ */

  .composer-file-icon {
    display: grid;
    place-items: center;
    width: 46px;
    height: 46px;
    flex-shrink: 0;
    border-radius: 9px;
    background: var(--code-bg, #eeece6);
    color: var(--text-secondary, #77746b);
  }

  .composer-file-icon :global(svg) {
    width: 22px;
    height: 22px;
  }

  /* ================================
   FILE INFORMATION
================================ */

  .composer-attachment-info {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 5px;
    min-width: 0;
    flex: 1;
  }

  .composer-attachment-name {
    display: block;
    overflow: hidden;
    color: var(--text-primary, #383731);
    font-size: 12px;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .composer-attachment-size {
    color: var(--text-muted, #89877f);
    font-size: 11px;
  }

  /* ================================
   REMOVE BUTTON
================================ */

  .composer-attachment-remove {
    position: absolute;
    top: 6px;
    right: 6px;
    display: grid;
    place-items: center;
    width: 23px;
    height: 23px;
    padding: 0;
    border: none;
    border-radius: 50%;
    background: var(--remove-bg, #eae8e1);
    color: var(--text-secondary, #77746b);
    cursor: pointer;
    opacity: 0.75;
    transition:
      background 0.2s ease,
      color 0.2s ease,
      opacity 0.2s ease,
      transform 0.2s ease;
  }

  .composer-attachment-remove:hover {
    background: #e5484d;
    color: white;
    opacity: 1;
    transform: scale(1.08);
  }

  .composer-attachment-remove :global(svg) {
    width: 13px;
    height: 13px;
  }

  /* ================================
   RESPONSIVE
================================ */

  @media (max-width: 600px) {
    .composer-attachments {
      gap: 8px;
      padding: 8px 0 12px;
    }

    .composer-attachment {
      flex-basis: 190px;
      height: 62px;
    }

    .composer-attachment-preview {
      width: 42px;
      height: 42px;
    }

    .composer-file-icon {
      width: 40px;
      height: 40px;
    }
  }

  /* ================================
   MESSAGE LIST
================================ */

  .message-list {
    flex: 1;
    width: 100%;
    max-width: 900px;
    min-width: 0;
    margin: 0 auto;
    padding: 32px 40px 40px;
    overflow-y: auto;
    overflow-x: hidden;
    scroll-behavior: smooth;
    scrollbar-width: thin;
    scrollbar-color: var(--border-color, #dedbd3) transparent;
  }

  .message-list::-webkit-scrollbar {
    width: 6px;
  }

  .message-list::-webkit-scrollbar-thumb {
    background: var(--border-color, #dedbd3);
    border-radius: 10px;
  }

  /* ================================
   MESSAGE ROW
================================ */

  .message-row {
    display: flex;
    align-items: flex-start;
    gap: 12px;
    width: 100%;
    margin-bottom: 30px;
    min-width: 0;
  }

  .message-row.user {
    justify-content: flex-end;
  }

  .message-row.agent {
    justify-content: flex-start;
  }

  /* ================================
   AVATAR
================================ */

  .message-avatar {
    width: 30px;
    height: 30px;
    flex-shrink: 0;
    object-fit: contain;
    border-radius: 50%;
    margin-top: 3px;
  }

  /* ================================
   MESSAGE CONTENT
================================ */

  .message-content {
    display: flex;
    flex-direction: column;
    min-width: 0;
    max-width: 85%;
    gap: 7px;
  }

  .message-row.agent .message-content {
    max-width: 100%;
    flex: 1;
  }

  .message-row.user .message-content {
    max-width: min(75%, 580px);
    align-items: flex-end;
  }

  /* ================================
   SENDER LABEL
================================ */

  .sender-label {
    font-size: 11px;
    font-weight: 600;
    color: var(--text-muted, #89877f);
    letter-spacing: 0.3px;
    padding: 0 3px;
  }

  /* ================================
   MESSAGE TEXT
================================ */

  .message-text {
    min-width: 0;
    color: var(--text-primary, #383731);
    font-size: 14px;
    line-height: 1.8;
    overflow-wrap: anywhere;
    white-space: normal;
  }

  .message-row.user .message-text {
    padding: 11px 15px;
    border-radius: 15px 15px 4px 15px;
    background: var(--user-message-bg, #f0eee7);
    line-height: 1.65;
  }

  .message-row.agent .message-text {
    width: 100%;
  }

  /* ================================
   MARKDOWN CONTENT
================================ */

  .message-text :global(p) {
    margin: 0 0 12px;
  }

  .message-text :global(p:last-child) {
    margin-bottom: 0;
  }

  .message-text :global(h1),
  .message-text :global(h2),
  .message-text :global(h3) {
    margin: 22px 0 10px;
    font-weight: 650;
    line-height: 1.4;
  }

  .message-text :global(ul),
  .message-text :global(ol) {
    padding-left: 23px;
    margin: 10px 0;
  }

  .message-text :global(li) {
    margin-bottom: 5px;
    padding-left: 3px;
  }

  .message-text :global(pre) {
    overflow-x: auto;
    padding: 15px;
    border-radius: 10px;
    background: var(--code-bg, #f5f4f0);
    margin: 12px 0;
  }

  .message-text :global(code) {
    font-size: 0.9em;
  }

  .message-text :global(blockquote) {
    margin: 12px 0;
    padding: 3px 14px;
    border-left: 3px solid var(--border-color, #dedbd3);
    color: var(--text-muted, #777);
  }

  .message-text :global(a) {
    color: var(--accent-color, #6c8de5);
    text-decoration: underline;
    text-underline-offset: 3px;
  }

  /* ================================
   ATTACHMENTS
================================ */

  .message-attachments {
    display: flex;
    flex-wrap: wrap;
    gap: 9px;
    max-width: 100%;
    margin-bottom: 3px;
  }

  .attachment-card {
    display: flex;
    align-items: center;
    gap: 10px;
    max-width: 260px;
    min-width: 0;
    padding: 9px 11px;
    border: 1px solid var(--border-color, #e3e0d8);
    border-radius: 11px;
    background: var(--attachment-bg, #faf9f6);
    overflow: hidden;
  }

  .message-row.user .attachment-card {
    background: var(--user-message-bg, #f0eee7);
  }

  /* Image preview */

  .attachment-image {
    width: 150px;
    height: 115px;
    overflow: hidden;
    border-radius: 9px;
    background: var(--code-bg, #f5f4f0);
  }

  .attachment-image img {
    width: 100%;
    height: 100%;
    display: block;
    object-fit: cover;
    cursor: pointer;
    transition: transform 0.2s ease;
  }

  .attachment-image img:hover {
    transform: scale(1.04);
  }

  /* File icon */

  .attachment-file-icon {
    width: 42px;
    height: 42px;
    flex-shrink: 0;
    display: grid;
    place-items: center;
    border-radius: 8px;
    background: var(--code-bg, #f0eee7);
    color: var(--text-secondary, #77746b);
  }

  .attachment-file-icon svg {
    width: 24px;
    height: 24px;
  }

  /* File information */

  .attachment-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 0;
  }

  .attachment-name {
    max-width: 175px;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    color: var(--text-primary, #383731);
    font-size: 12px;
    font-weight: 600;
  }

  .attachment-size {
    color: var(--text-muted, #89877f);
    font-size: 11px;
  }

  /* ================================
   THINKING INDICATOR
================================ */

  .thinking-indicator {
    display: flex;
    align-items: center;
    gap: 5px;
    padding: 10px 3px;
  }

  .thinking-indicator span {
    width: 6px;
    height: 6px;
    background: var(--text-muted, #999);
    border-radius: 50%;
    animation: thinking 1.2s infinite ease-in-out;
  }

  .thinking-indicator span:nth-child(2) {
    animation-delay: 0.15s;
  }

  .thinking-indicator span:nth-child(3) {
    animation-delay: 0.3s;
  }

  @keyframes thinking {
    0%,
    60%,
    100% {
      opacity: 0.35;
      transform: translateY(0);
    }

    30% {
      opacity: 1;
      transform: translateY(-4px);
    }
  }

  /* ================================
   SCROLL ANCHOR
================================ */

  .scroll-anchor {
    height: 1px;
    width: 100%;
  }

  /* ================================
   RESPONSIVE
================================ */

  @media (max-width: 700px) {
    .message-list {
      padding: 22px 16px 30px;
    }

    .message-row {
      gap: 9px;
      margin-bottom: 24px;
    }

    .message-content {
      max-width: 92%;
    }

    .message-row.user .message-content {
      max-width: 85%;
    }

    .message-text {
      font-size: 13px;
    }

    .message-avatar {
      width: 26px;
      height: 26px;
    }

    .attachment-image {
      width: 125px;
      height: 100px;
    }
  }

  .sender-label {
    margin-bottom: 6px;
    color: #89877f;
    font-size: 11px;
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
    0%,
    60%,
    100% {
      opacity: 0.4;
      transform: translateY(0);
    }
    30% {
      opacity: 1;
      transform: translateY(-4px);
    }
  }

  @media (max-width: 560px) {
    .message-list {
      width: calc(100% - 24px);
    }

    .conversation-footer {
      padding: 10px 12px max(12px, env(safe-area-inset-bottom));
    }
  }
</style>
