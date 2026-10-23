
<div class="dashboard">
  <div class="upgrade">Free plan · <a href="/upgrade">Upgrade</a></div>

  <section class="welcome">
    <h1><span class="star">✳</span> Golden hour thinking</h1>

    <div class="composer">
      <textarea
        placeholder="How can I help you today?"
        rows="2"
      ></textarea>

      <div class="composer-bottom">
        <button class="add" aria-label="Add attachment">+</button>

        <div class="actions">
          <select aria-label="Select model">
            <option>Sonnet 4.6</option>
            <option>Opus</option>
            <option>Haiku</option>
          </select>

          <button class="mic" aria-label="Voice input">♩</button>
        </div>
      </div>
    </div>

    <button class="connect">
      Connect your tools to Claude
      <span>›</span>
    </button>
  </section>
</div>

<style>
  .dashboard {
    position: relative;
    display: flex;
    flex-direction: column;
    align-items: center;
    width: 100%;
    min-height: 100%;
    padding: 135px 24px 40px;
    background: #faf9f6;
    color: #383731;
  }

  .upgrade {
    padding: 5px 8px;
    border-radius: 5px;
    background: #f1f0eb;
    color: #77766f;
    font-size: 10px;
  }

  .upgrade a {
    color: inherit;
    text-decoration: none;
  }

  .upgrade a:hover {
    text-decoration: underline;
  }

  .welcome {
    width: 100%;
    max-width: 444px;
    margin-top: 25px;
  }

  h1 {
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 9px;
    margin: 0 0 25px;
    font-family: Georgia, "Times New Roman", serif;
    font-size: clamp(22px, 3vw, 28px);
    font-weight: 400;
    letter-spacing: -1px;
    white-space: nowrap;
  }

  .star {
    color: #d78362;
    font-family: Arial, sans-serif;
  }

  .composer {
    padding: 12px 13px 9px;
    border: 1px solid #e5e3dc;
    border-radius: 17px;
    background: #fff;
    box-shadow: 0 4px 12px #00000008;
  }

  textarea {
    display: block;
    width: 100%;
    min-height: 35px;
    resize: vertical;
    border: 0;
    outline: 0;
    background: transparent;
    color: #383731;
    font: inherit;
    font-size: 12px;
  }

  textarea::placeholder {
    color: #89877f;
  }

  .composer-bottom,
  .actions {
    display: flex;
    align-items: center;
  }

  .composer-bottom {
    justify-content: space-between;
    min-height: 25px;
  }

  .actions {
    gap: 10px;
  }

  .composer button {
    display: grid;
    place-items: center;
    padding: 3px;
    border: 0;
    background: transparent;
    color: #65645d;
    cursor: pointer;
  }

  .add {
    font-size: 19px;
  }

  .mic {
    font-size: 17px;
  }

  select {
    max-width: 110px;
    border: 0;
    outline: 0;
    background: transparent;
    color: #55544e;
    font-size: 10px;
    cursor: pointer;
  }

  .connect {
    display: flex;
    align-items: center;
    justify-content: space-between;
    align-self: stretch;
    margin-top: 3px;
    padding: 3px 14px;
    border: 0;
    background: transparent;
    color: #55544e;
    font-size: 10px;
    cursor: pointer;
  }

  .connect span {
    color: #89877f;
    font-size: 17px;
  }

  @media (max-width: 560px) {
    .dashboard {
      padding: 70px 16px 30px;
    }

    h1 {
      gap: 6px;
      font-size: clamp(19px, 5vw, 25px);
      white-space: normal;
    }
  }
</style>
