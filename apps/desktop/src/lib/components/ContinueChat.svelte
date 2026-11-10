
<script lang="ts">
import { marked } from "marked";
  import DOMPurify from "dompurify";
  import type { Attachment, Message } from "#lib/utils/types";
  function renderMarkdown(content: string) {
    return DOMPurify.sanitize(marked.parse(content) as string);
  }

        interface Props {
                handleSendMessage: (event: SubmitEvent) => void;
                inputElement?: HTMLTextAreaElement; // Marked optional until element mounts
                input:string;
                isLoading:boolean;
                isSelecting:boolean;
                modelName?:string | null;
                viewportElement?:HTMLElement | null;
                messages:Message[];
                activeAgentMsgId:string | null;
                addAttachments:() => void;
                attachments:Attachment[];
                bottomElement?:HTMLDivElement;
                removeAttachment:(id: string) => void;
            }

    	let { handleSendMessage,bottomElement=$bindable(), viewportElement=$bindable(),activeAgentMsgId, inputElement = $bindable(), modelName = $bindable(),
        input= $bindable(),isLoading,isSelecting, messages,attachments ,removeAttachment,addAttachments }:Props = $props();

</script>

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
                {msg.sender === "user" ? "You" : "Agent"}
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
           
             <button  class="add-button"
              type="button"
              aria-label="Add attachment"
              title="Add attachment"
              disabled={isSelecting}
              onclick={addAttachments}>
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
            <div class="attachments-list">
              {#each attachments as attachment (attachment.id)}
                <div class="attachment-item">
                  <span class="attachment-name">
                    {attachment.name}
                  </span>

                  <button
                    type="button"
                    aria-label="Remove attachment"
                    onclick={() => removeAttachment(attachment.id)}
                  >
                    ×
                  </button>
                </div>
              {/each}
            </div>
          {/if}

        <p class="disclaimer">
          AI can make mistakes. Review important information.
        </p>
      </footer>
    </main>



<style>


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
.chat-conversation {
  width: 100%;
  flex: 1;
  min-width: 0;
}


.message-list {
  flex: 1;
  width: min(100% - 40px, 700px);
  margin: 0 auto;
  padding: 28px 0;
  overflow-y: auto;
}

.message-list {
  width: 100%;
  max-width: none;
  margin: 0;
  padding-inline: 64px;
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

.message-content {
  width: 100%;
  min-width: 0;
}

.message-content {
  max-width: 100%;
  flex: 1;
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
.message-row {
  width: 100%;
}

/* Prevent content overflow */
.message-text {
  overflow-wrap: anywhere;
}

.message-text :global(li) {
  margin-bottom: 2px;
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
 
  .message-list {
    width: calc(100% - 24px);
  }

  .conversation-footer {
    padding: 10px 12px max(12px, env(safe-area-inset-bottom));
  }
}

</style>