
<script lang="ts">
  import { goto } from "$app/navigation";

  let { message = "Something went wrong." }: { message?: string } = $props();

  function goHome() {
    goto("/app");
  }

  function goBack() {
    if (typeof window !== "undefined" && window.history.length > 1) {
      window.history.back();
    } else {
      goHome();
    }
  }
</script>

<svelte:head>
  <title>Something went wrong</title>
</svelte:head>

<main class="error-page">
  <div class="error-content">
    <div class="error-icon">✳</div>

    <p class="eyebrow">OOPS!</p>

    <h1>Something went wrong.</h1>

    <p class="description">
      {message}
      The page may be unavailable or may have moved.
    </p>

    <div class="actions">
      <button class="primary-button" onclick={goHome}>
        Go to Home <span>↗</span>
      </button>

      <button class="secondary-button" onclick={goBack}>
        Go Back
      </button>
    </div>

    <p class="footer-text">Golden Hour Workspace</p>
  </div>
</main>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    font-family: Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    background: #fbfaf7;
    color: #302f2a;
  }

  .error-page {
    display: grid;
    min-height: 100dvh;
    place-items: center;
    padding: 24px;
  }

  .error-content {
    width: 100%;
    max-width: 420px;
    text-align: center;
  }

  .error-icon {
    margin-bottom: 28px;
    color: #d78261;
    font-size: 42px;
  }

  .eyebrow {
    margin: 0 0 12px;
    color: #d78261;
    font-size: 10px;
    font-weight: 600;
    letter-spacing: 0.18em;
  }

  h1 {
    margin: 0;
    font-family: Georgia, "Times New Roman", serif;
    font-size: clamp(28px, 5vw, 38px);
    font-weight: 400;
    letter-spacing: -1px;
  }

  .description {
    margin: 16px 0 28px;
    color: #89867e;
    font-size: 13px;
    line-height: 1.8;
  }

  .actions {
    display: flex;
    justify-content: center;
    flex-wrap: wrap;
    gap: 10px;
  }

  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 12px 17px;
    border: 1px solid #e5e2db;
    border-radius: 8px;
    font: inherit;
    font-size: 12px;
    cursor: pointer;
    transition: background 120ms ease;
  }

  .primary-button {
    border-color: #302f2a;
    background: #302f2a;
    color: white;
  }

  .primary-button:hover {
    background: #49473f;
  }

  .secondary-button {
    background: transparent;
    color: #55534d;
  }

  .secondary-button:hover {
    background: #f0eee8;
  }

  .footer-text {
    margin-top: 55px;
    color: #aaa69d;
    font-size: 10px;
    letter-spacing: 0.04em;
  }
</style>
