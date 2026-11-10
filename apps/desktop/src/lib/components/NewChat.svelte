<script lang="ts">
  import { formatFileSize } from '#lib/utils/functions.ts';
  import { goto } from '$app/navigation';
  import { convertFileSrc } from '@tauri-apps/api/core';

  interface Attachment {
    id: string;
    name: string;
    path: string;
    size: number;
    mimeType: string | null;
  }

  interface Props {
    handleSendMessage: (event: SubmitEvent) => void;
    inputElement?: HTMLTextAreaElement; // Marked optional until element mounts
    input?: string;
    isLoading: boolean;
    isSelecting: boolean;
    modelName?: string | null;
    addAttachments: () => void;
    attachments: Attachment[];
    removeAttachment: (id: string) => void;
  }

  let {
    handleSendMessage,
    inputElement = $bindable(),
    modelName = $bindable(),
    input = $bindable(),
    isLoading,
    isSelecting,
    attachments,
    removeAttachment,
    addAttachments,
  }: Props = $props();
</script>

<main class="chat-home">
  <section class="welcome">
    <div class="welcome-heading">
      <div class="welcome-icon">
        <img src="/favicon.png" alt="" />
      </div>

      <div class="welcome-title">
        <h1>Start new Investigation</h1>
        <p>Explore ideas, investigate questions, and get things done.</p>
      </div>
    </div>

    <form class="composer" onsubmit={handleSendMessage}>
      <textarea
        bind:this={inputElement}
        bind:value={input}
        placeholder="How can I help you today?"
        rows="2"
        aria-label="Message your AI assistant"
        disabled={isLoading}
        onkeydown={(event) => {
          if (event.key === 'Enter' && !event.shiftKey && !event.isComposing) {
            event.preventDefault();
            event.currentTarget.form?.requestSubmit();
          }
        }}></textarea>

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
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.6"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  >
                    <path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" />
                    <path d="M13 2v7h7" />
                  </svg>
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
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="1.7"
                  stroke-linecap="round"
                >
                  <path d="M18 6L6 18M6 6l12 12" />
                </svg>
              </button>
            </div>
          {/each}
        </div>

        <div class="attachment-summary">
          <span>{attachments.length} files attached</span>
          <span>
            {formatFileSize(attachments.reduce((total, file) => total + file.size, 0))}
            total
          </span>
        </div>
      {/if}

      <div class="composer-bottom">
        <div class="composer-left-actions">
          <button
            class="add-button"
            type="button"
            aria-label="Add attachment"
            title="Add attachment"
            disabled={isSelecting}
            onclick={addAttachments}
          >
            {#if isSelecting}
              <span class="selecting-indicator">...</span>
            {:else}
              +
            {/if}
          </button>

          {#if modelName && modelName !== ''}
            <div class="model-selector">
              <select aria-label="Select model" value={modelName}>
                <option>{modelName}</option>
              </select>
            </div>
          {/if}
        </div>

        <div class="composer-actions">
          <button
            class="mic-button"
            type="button"
            aria-label="Voice input"
            title="Voice input"
            onclick={() => {
              // Connect your existing voice input handler here.
            }}
          >
            <svg
              viewBox="0 0 24 24"
              width="17"
              height="17"
              fill="none"
              stroke="currentColor"
              stroke-width="1.6"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <rect x="9" y="3" width="6" height="12" rx="3" />
              <path d="M5.5 11a6.5 6.5 0 0 0 13 0" />
              <path d="M12 17.5V21M9 21h6" />
            </svg>
          </button>
        </div>
      </div>
    </form>

    <button class="connect-tools" type="button" onclick={() => goto('/app/customize')}>
      <div class="connect-tools-icon">
        <svg
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.6"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path d="M12 8v8M8 12h8" />
          <circle cx="12" cy="12" r="9" />
        </svg>
      </div>

      <div class="connect-tools-content">
        <span class="connect-tools-title">Connect your tools</span>
        <span class="connect-tools-description">
          Extend your assistant with additional capabilities
        </span>
      </div>

      <span class="chevron">›</span>
    </button>
  </section>
</main>

<style>
  /* =================================
   CHAT HOME
================================= */

  .chat-home {
    display: flex;
    flex: 1;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-width: 0;
    min-height: 0;
    overflow-y: auto;
    overflow-x: hidden;
    padding: 50px 24px;
    scrollbar-width: thin;
  }

  /* =================================
   WELCOME SECTION
================================= */

  .welcome {
    width: 100%;
    max-width: 560px;
    margin: auto 0;
  }

  /* =================================
   WELCOME HEADING
================================= */

  .welcome-heading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 13px;
    margin-bottom: 30px;
  }

  .welcome-icon {
    display: grid;
    place-items: center;
    width: 43px;
    height: 43px;
    flex-shrink: 0;
    border: 1px solid #e9e5dc;
    border-radius: 13px;
    background: #fff;
    box-shadow: 0 3px 10px #00000008;
  }

  .welcome-icon img {
    width: 27px;
    height: 27px;
    object-fit: contain;
  }

  .welcome-title {
    min-width: 0;
  }

  .welcome-title h1 {
    margin: 0;
    color: #383731;
    font-family: Georgia, 'Times New Roman', serif;
    font-size: clamp(22px, 3vw, 27px);
    font-weight: 400;
    letter-spacing: -0.8px;
    line-height: 1.3;
  }

  .welcome-title p {
    margin: 5px 0 0;
    color: #89877f;
    font-size: 12px;
    line-height: 1.5;
  }

  .model-selector {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 4px 9px;
    background: #faf9f6;
    transition:
      border-color 150ms ease,
      background 150ms ease;
  }

  .model-selector:hover {
    border-color: #d9d4c9;
    background: #f5f3ed;
  }

  .model-selector select {
    max-width: 140px;
    padding: 3px 18px 3px 2px;
    border: none;
    outline: none;
    background: transparent;
    color: #55544e;
    font-family: inherit;
    font-size: 11px;
    cursor: pointer;
  }

  /* =================================
   COMPOSER
================================= */

  .composer {
    width: 100%;
    padding: 14px 14px 10px;
    border: 1px solid #e5e3dc;
    border-radius: 17px;
    background: #fff;
    box-shadow: 0 4px 14px #00000008;
    transition:
      border-color 160ms ease,
      box-shadow 160ms ease;
  }

  .composer:focus-within {
    border-color: #d9b8a7;
    box-shadow:
      0 0 0 3px #d7836210,
      0 4px 14px #00000008;
  }

  .composer textarea {
    display: block;
    width: 100%;
    min-height: 65px;
    max-height: 220px;
    resize: vertical;
    border: 0;
    outline: 0;
    background: transparent;
    color: #383731;
    font-family: inherit;
    font-size: 13px;
    line-height: 1.7;
  }

  .composer textarea::placeholder {
    color: #98958d;
    opacity: 1;
  }

  .composer textarea:disabled {
    opacity: 0.6;
  }

  /* =================================
   COMPOSER BOTTOM
================================= */

  .composer-bottom {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    min-height: 32px;
    padding-top: 5px;
  }

  .composer-left-actions,
  .composer-actions {
    display: flex;
    align-items: center;
    gap: 9px;
    min-width: 0;
  }

  .composer button {
    display: grid;
    place-items: center;
    flex-shrink: 0;
    padding: 5px;
    border: 0;
    border-radius: 7px;
    background: transparent;
    color: #65645d;
    cursor: pointer;
    transition:
      background 150ms ease,
      color 150ms ease;
  }

  .composer button:hover {
    background: #f4f2ed;
  }

  .composer button:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .add-button {
    width: 27px;
    height: 27px;
    font-size: 21px;
    line-height: 1;
  }

  .selecting-indicator {
    font-size: 13px;
  }

  .mic-button {
    width: 28px;
    height: 28px;
  }

  /* =================================
   MODEL SELECTOR
================================= */

  .model-selector {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 3px 8px;
    border-radius: 7px;
    background: #f7f6f2;
  }

  .model-selector select {
    max-width: 125px;
    min-width: 0;
    border: 0;
    outline: 0;
    background: transparent;
    color: #65645d;
    font-size: 10px;
    cursor: pointer;
  }

  .model-selector select option {
    background: #fff;
    color: #383731;
  }

  /* =================================
   ATTACHMENTS
================================= */

  .composer-attachments {
    display: flex;
    align-items: stretch;
    gap: 9px;
    width: 100%;
    padding: 12px 0;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: thin;
    scrollbar-color: #dedbd3 transparent;
  }

  .composer-attachments::-webkit-scrollbar {
    height: 4px;
  }

  .composer-attachments::-webkit-scrollbar-thumb {
    background: #dedbd3;
    border-radius: 10px;
  }

  .composer-attachment {
    position: relative;
    display: flex;
    align-items: center;
    gap: 9px;
    flex: 0 0 210px;
    min-width: 0;
    height: 64px;
    padding: 8px 32px 8px 8px;
    border: 1px solid #e5e2da;
    border-radius: 11px;
    background: #faf9f6;
    transition:
      border-color 160ms ease,
      background 160ms ease;
  }

  .composer-attachment:hover {
    border-color: #c9c2b7;
    background: #f5f3ed;
  }

  /* Image preview */

  .composer-attachment-preview {
    width: 45px;
    height: 45px;
    flex-shrink: 0;
    overflow: hidden;
    border-radius: 8px;
    background: #eeece6;
  }

  .composer-attachment-preview img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  /* File icon */

  .composer-file-icon {
    display: grid;
    place-items: center;
    width: 43px;
    height: 43px;
    flex-shrink: 0;
    border-radius: 8px;
    background: #eeece6;
    color: #77746b;
  }

  .composer-file-icon svg {
    width: 21px;
    height: 21px;
  }

  /* File details */

  .composer-attachment-info {
    display: flex;
    flex: 1;
    flex-direction: column;
    justify-content: center;
    gap: 4px;
    min-width: 0;
  }

  .composer-attachment-name {
    display: block;
    overflow: hidden;
    color: #383731;
    font-size: 11px;
    font-weight: 600;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .composer-attachment-size {
    color: #89877f;
    font-size: 10px;
  }

  /* Remove attachment */

  .composer-attachment-remove {
    position: absolute;
    top: 5px;
    right: 5px;
    display: grid;
    place-items: center;
    width: 21px;
    height: 21px;
    padding: 0;
    border: 0;
    border-radius: 50%;
    background: #eeece6;
    color: #77746b;
    cursor: pointer;
    transition:
      background 150ms ease,
      color 150ms ease;
  }

  .composer-attachment-remove:hover {
    background: #e5484d;
    color: #fff;
  }

  .composer-attachment-remove svg {
    width: 12px;
    height: 12px;
  }

  /* =================================
   ATTACHMENT SUMMARY
================================= */

  .attachment-summary {
    display: flex;
    justify-content: space-between;
    gap: 10px;
    padding: 0 2px 10px;
    color: #89877f;
    font-size: 10px;
  }

  /* =================================
   CONNECT TOOLS
================================= */

  .connect-tools {
    display: flex;
    align-items: center;
    gap: 12px;
    width: 100%;
    margin-top: 13px;
    padding: 11px 12px;
    border: 1px solid #eae8e1;
    border-radius: 12px;
    background: transparent;
    text-align: left;
    cursor: pointer;
    transition:
      border-color 160ms ease,
      background 160ms ease;
  }

  .connect-tools:hover {
    border-color: #d9d4c9;
    background: #faf9f6;
  }

  .connect-tools-icon {
    display: grid;
    place-items: center;
    width: 33px;
    height: 33px;
    flex-shrink: 0;
    border-radius: 9px;
    background: #f0eee7;
    color: #77746b;
  }

  .connect-tools-icon svg {
    width: 18px;
    height: 18px;
  }

  .connect-tools-content {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }

  .connect-tools-title {
    color: #48473f;
    font-size: 11px;
    font-weight: 600;
  }

  .connect-tools-description {
    color: #89877f;
    font-size: 10px;
  }

  .chevron {
    color: #89877f;
    font-size: 20px;
    transition: transform 150ms ease;
  }

  .connect-tools:hover .chevron {
    transform: translateX(3px);
  }

  /* =================================
   RESPONSIVE
================================= */

  @media (max-width: 700px) {
    .chat-home {
      padding: 40px 20px;
    }

    .welcome {
      max-width: 100%;
    }
  }

  @media (max-width: 560px) {
    .chat-home {
      padding: 35px 15px;
    }

    .welcome-heading {
      gap: 10px;
      margin-bottom: 23px;
    }

    .welcome-icon {
      width: 38px;
      height: 38px;
      border-radius: 11px;
    }

    .welcome-icon img {
      width: 24px;
      height: 24px;
    }

    .welcome-title h1 {
      font-size: clamp(19px, 5vw, 24px);
    }

    .welcome-title p {
      font-size: 11px;
    }

    .composer {
      padding: 11px;
      border-radius: 15px;
    }

    .composer textarea {
      min-height: 58px;
      font-size: 13px;
    }

    .composer-attachment {
      flex-basis: 185px;
      height: 60px;
    }

    .composer-attachment-preview {
      width: 41px;
      height: 41px;
    }

    .composer-file-icon {
      width: 39px;
      height: 39px;
    }

    .connect-tools {
      padding: 10px;
    }
  }

  @media (prefers-reduced-motion: reduce) {
    *,
    *::before,
    *::after {
      transition-duration: 0.01ms !important;
      animation-duration: 0.01ms !important;
      animation-iteration-count: 1 !important;
    }
  }
</style>
