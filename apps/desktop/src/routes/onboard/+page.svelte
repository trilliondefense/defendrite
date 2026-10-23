
<script lang="ts">
  import { goto } from "$app/navigation";
  import "./local.css";

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
    <div class="brand-mark" aria-label="Welcome">
      <svg
        viewBox="0 0 32 32"
        fill="none"
        aria-hidden="true"
      >
        <g
          stroke="currentColor"
          stroke-width="2.4"
          stroke-linecap="round"
        >
          <path d="M16 3v8" />
          <path d="M16 21v8" />
          <path d="M3 16h8" />
          <path d="M21 16h8" />
          <path d="M6.8 6.8l5.7 5.7" />
          <path d="M19.5 19.5l5.7 5.7" />
          <path d="M25.2 6.8l-5.7 5.7" />
          <path d="M12.5 19.5l-5.7 5.7" />
          <path d="M10.2 4.7l2.5 6.8" />
          <path d="M19.3 20.5l2.5 6.8" />
          <path d="M4.7 10.2l6.8 2.5" />
          <path d="M20.5 19.3l6.8 2.5" />
        </g>
      </svg>
    </div>

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

html,
body {
  width: 100%;
  min-width: 320px;
  min-height: 100%;
  margin: 0;
}

body {
  min-height: 100vh;
  background: #f8f8f6;
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

.brand-mark {
  width: 30px;
  height: 30px;
  color: #d88770;
  margin-bottom: 19px;
}

.brand-mark svg {
  display: block;
  width: 100%;
  height: 100%;
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