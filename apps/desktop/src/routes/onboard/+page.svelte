
<script lang="ts">
  import { goto } from "$app/navigation";

  let name = $state("");

  function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const userName = name.trim();
    if (!userName) return;

    localStorage.setItem("userName", userName);
    goto("/", { replaceState: true });
  }
</script>

<main class="setup-screen">
  <section class="setup-card">
    <img src="/favicon.png" alt="" class="message-avatar" />

    <p class="setup-prompt">
      Before we get started, what should I call you?
    </p>

    <form class="name-form" onsubmit={handleSubmit}>
      <input
        bind:value={name}
        type="text"
        placeholder="Enter your name"
        autocomplete="given-name"
        maxlength="80"
        aria-label="Your name"
        required
      />

      <button
        type="submit"
        disabled={!name.trim()}
        aria-label="Continue"
      >
        <svg
          viewBox="0 0 24 24"
          fill="none"
          aria-hidden="true"
        >
          <path
            d="M12 19V5m-6 6 6-6 6 6"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          />
        </svg>
      </button>
    </form>
  </section>
</main>

<style>
  :root {
  font-family: Georgia, "Times New Roman", serif;
  color: #252525;
  background: #f8f8f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

* {
  box-sizing: border-box;
}



button,
input {
  font: inherit;
}

.setup-screen {
  min-height: 100vh;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 64px 24px 40px;
  background: #f8f8f6;
}

.setup-card {
  width: 100%;
  max-width: 380px;
  margin: 0;
}


.setup-prompt {
  margin: 0 0 11px;
  font-family: Georgia, "Times New Roman", serif;
  font-size: 14px;
  font-weight: 400;
  line-height: 1.4;
  color: #292929;
}

.name-form {
  width: 100%;
  height: 40px;
  display: flex;
  align-items: center;
  gap: 0;
  padding: 3px;
  background: #ffffff;
  border: 1px solid #9aaec0;
  border-radius: 13px;
  box-shadow:
    0 0 0 2px rgba(104, 151, 191, 0.35),
    inset 0 1px 2px rgba(0, 0, 0, 0.04);
  transition:
    border-color 150ms ease,
    box-shadow 150ms ease;
}

.name-form:focus-within {
  border-color: #719ac0;
  box-shadow:
    0 0 0 2px rgba(104, 151, 191, 0.38),
    inset 0 1px 2px rgba(0, 0, 0, 0.04);
}

.name-form input {
  flex: 1;
  width: 0;
  min-width: 0;
  height: 100%;
  padding: 0 15px 0 39px;
  border: none;
  outline: none;
  background: transparent;
  color: #333333;
  font-family: Arial, Helvetica, sans-serif;
  font-size: 12px;
}

.name-form input::placeholder {
  color: #999999;
  opacity: 1;
}

.name-form button {
  width: 27px;
  height: 27px;
  flex-shrink: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  border: none;
  border-radius: 9px;
  background: #858585;
  color: #ffffff;
  cursor: pointer;
  transition:
    background 150ms ease,
    opacity 150ms ease;
}

.name-form button svg {
  width: 18px;
  height: 18px;
}


.message-avatar {
  width: 27px;
  height: 27px;
  object-fit: contain;
}

.name-form button:disabled {
  opacity: 1;
  cursor: default;
}

.name-form button:not(:disabled) {
  background: #777777;
}

.name-form button:not(:disabled):hover {
  background: #555555;
}

.name-form button:focus-visible {
  outline: 2px solid #719ac0;
  outline-offset: 2px;
}

@media (max-width: 480px) {
  .setup-screen {
    padding: 56px 24px 32px;
  }

  .setup-card {
    max-width: 380px;
  }
}

</style>