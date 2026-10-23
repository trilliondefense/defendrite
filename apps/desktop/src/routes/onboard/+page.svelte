
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

<svelte:head>
  <title>Welcome</title>
</svelte:head>

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
