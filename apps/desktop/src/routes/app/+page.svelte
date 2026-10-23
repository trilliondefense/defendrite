

<div class="home">
  <p class="eyebrow">YOUR WORKSPACE</p>

  <h1>It works! ✳</h1>

  <p class="description">
    Your SvelteKit layout is rendering inside the Tauri desktop app.
  </p>

  <div class="status">
    <span class="dot"></span>
    Application is running
  </div>

  <a href="/app/newchat" class="button">
    Open New Chat →
  </a>
</div>

<style>
  .home {
    max-width: 600px;
    margin: 80px auto;
  }

  .eyebrow {
    color: #d17d5c;
    font-size: 11px;
    letter-spacing: 2px;
  }

  h1 {
    margin: 18px 0 12px;
    font-size: 38px;
    font-weight: 500;
  }

  .description {
    color: #777;
    line-height: 1.7;
  }

  .status {
    display: flex;
    align-items: center;
    gap: 9px;
    margin: 28px 0;
    font-size: 13px;
  }

  .dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #399764;
  }

  .button {
    display: inline-block;
    padding: 11px 16px;
    border-radius: 7px;
    background: #292824;
    color: white;
    text-decoration: none;
    font-size: 13px;
  }

  .button:hover {
    background: #45443f;
  }
</style>
