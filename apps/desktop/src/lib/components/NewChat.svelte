<script lang="ts">
  import { goto } from "$app/navigation";

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
                input?:string;
                isLoading:boolean;
                isSelecting:boolean;
                modelName?:string | null;
                addAttachments:() => void;
                attachments:Attachment[];
                removeAttachment:(id: string) => void;
            }

    	let { handleSendMessage,inputElement = $bindable(), modelName = $bindable(),
        input= $bindable(),isLoading,isSelecting, attachments ,removeAttachment,addAttachments }:Props = $props();
</script>
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
            disabled={isSelecting}
            onclick={addAttachments}
        >
            {isSelecting ? '...' : '+'}
        </button>
        


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


<style>


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
.message-avatar {
  width: 27px;
  height: 27px;
  object-fit: contain;
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


}

</style>