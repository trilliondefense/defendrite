<script lang="ts">
  import { onMount } from 'svelte';

  import { Store } from '@tauri-apps/plugin-store';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  type Theme = 'light' | 'dark' | 'system';

  let theme = $state<Theme>('light');
  let accentColor = $state('#d78261');
  let compactMode = $state(false);
  let soundEnabled = $state(false);
  let enterToSend = $state(true);
  let launchAtStartup = $state(false);
  let saved = $state(false);

  let config = $state('');
  let editing = $state(false);
  let loading = $state(false);
  let error = $state('');

  let apiKey = $state('');
  let modelName = $state('gemini-1.5-flash');
  let dbHistoryKey = $state('');
  let currentPrompt = $state('');
  let streamingOutput = $state('');
  let statusLog = $state('');
  let isProcessing = $state(false);

  let store: Store | null = null;

  const accents = [
    { name: 'Terracotta', value: '#d78261' },
    { name: 'Blue', value: '#6488c7' },
    { name: 'Green', value: '#659b78' },
    { name: 'Violet', value: '#9176c5' },
    { name: 'Rose', value: '#ce7890' },
  ];

  async function saveConfig() {
    error = '';
    try {
      if (store) {
        await store.set('HARNESS_MODEL_API_KEY', { value: apiKey });
        await store.set('HARNESS_MODEL_NAME', { value: modelName });
        await store.set('HARNESS_HISTORY_DB_KEY', { value: dbHistoryKey });
        const parsed = JSON.parse(config);

        loading = true;

        await invoke('save_mcp_config', {
          config: parsed,
        });

        config = JSON.stringify(parsed, null, 2);
        editing = false;

        await store.save(); // Permanently flushes settings cache to disk
      }
      statusLog = 'Configuration saved successfully.';
    } catch (err) {
      statusLog = `Failed to save configuration: ${err}`;
      error = String(err);
    } finally {
      loading = false;
    }
  }
  async function cancelEdit() {
    editing = false;
    await loadConfig();
  }
  async function loadConfig() {
    loading = true;
    error = '';

    try {
      const result = await invoke('get_mcp_config');
      config = JSON.stringify(result, null, 2);
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }
  let isMac = $state(false);

  onMount(() => {
    isMac = /Mac|iPhone|iPad|iPod/i.test(navigator.platform);
    try {
      theme = (localStorage.getItem('workspace-theme') as Theme) || 'light';
      accentColor = localStorage.getItem('workspace-accent') || '#d78261';
      compactMode = localStorage.getItem('workspace-compact') === 'true';
      soundEnabled = localStorage.getItem('workspace-sound') === 'true';
      enterToSend = localStorage.getItem('workspace-enter-send') !== 'false';
      launchAtStartup = localStorage.getItem('workspace-startup') === 'true';
    } catch (error) {
      console.warn('Unable to load workspace preferences:', error);
    }

    applyTheme();

    let unlistenFunctions: Array<() => void> = [];

    // 1. Move all asynchronous initializations into an inner async execution block
    async function initStoreAndListeners() {
      try {
        // Load the store instance safely
        loadConfig();
        store = await Store.load('settings.json');

        // Read persisted values from disk
        const savedKey = await store.get<{ value: string }>('HARNESS_MODEL_API_KEY');
        const savedModel = await store.get<{ value: string }>('HARNESS_MODEL_NAME');
        const savedDBKey = await store.get<{ value: string }>('HARNESS_HISTORY_DB_KEY');

        if (savedKey) apiKey = savedKey.value;
        if (savedModel) modelName = savedModel.value;
        if (savedDBKey) dbHistoryKey = savedDBKey.value;

        localStorage.setItem('modelName', modelName);
        // Listen to token-by-token stream emissions from Rust runtime
        const unlistenToken = await listen<{ token: string }>('agent-token', (event) => {
          streamingOutput += event.payload.token;
        });
        unlistenFunctions.push(unlistenToken);

        // Listen to runtime generation exceptions
        const unlistenError = await listen<{ error: string }>('agent-error', (event) => {
          statusLog = `Runtime Error: ${event.payload.error}`;
          isProcessing = false;
        });
        unlistenFunctions.push(unlistenError);
      } catch (err) {
        statusLog = `Initialization error: ${err}`;
      }
    }

    // Fire off the asynchronous initializations without making onMount async
    initStoreAndListeners();

    // 2. Return a clean, synchronous function that runs perfectly on component unmount
    return () => {
      unlistenFunctions.forEach((unlisten) => unlisten());
    };
  });

  function applyTheme() {
    if (typeof document === 'undefined') return;

    const resolvedTheme =
      theme === 'system'
        ? window.matchMedia('(prefers-color-scheme: dark)').matches
          ? 'dark'
          : 'light'
        : theme;

    document.documentElement.dataset.theme = resolvedTheme;
    document.documentElement.style.setProperty('--workspace-accent', accentColor);
    document.documentElement.dataset.compact = String(compactMode);
  }

  function savePreferences() {
    try {
      localStorage.setItem('workspace-theme', theme);
      localStorage.setItem('workspace-accent', accentColor);
      localStorage.setItem('workspace-compact', String(compactMode));
      localStorage.setItem('workspace-sound', String(soundEnabled));
      localStorage.setItem('workspace-enter-send', String(enterToSend));
      localStorage.setItem('workspace-startup', String(launchAtStartup));
      saveConfig();
      applyTheme();
      saved = true;
      window.setTimeout(() => (saved = false), 1800);
    } catch (error) {
      console.error('Unable to save workspace preferences:', error);
    }
  }

  function resetPreferences() {
    theme = 'light';
    accentColor = '#d78261';
    compactMode = false;
    soundEnabled = false;
    enterToSend = true;
    launchAtStartup = false;
    savePreferences();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') {
      (event.target as HTMLElement)?.blur?.();
    }

    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 's') {
      event.preventDefault();
      savePreferences();
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="settings-page">
  <header class="page-header">
    <div>
      <h1>Customize</h1>
      <p class="subtitle">Make your workspace feel like yours.</p>

      <p class="keyboard-hint">
        <kbd>{isMac ? '⌘' : 'Ctrl'} S</kbd> Save preferences
        <span>·</span>
        <kbd>Esc</kbd> Unfocus control
      </p>
    </div>

    <div class="header-actions">
      <button class="reset-button" onclick={resetPreferences}>Reset</button>
      <button class="save-button" onclick={savePreferences}>
        {saved ? '✓ Saved' : 'Save changes'}
        <kbd>{isMac ? '⌘' : 'Ctrl'} S</kbd>
      </button>
    </div>
  </header>

  <div class="settings-layout">
    <main class="settings-content">
      <!-- <section id="appearance" class="settings-section">
        <div class="section-heading">
          <h2>Appearance</h2>
          <p>Adjust how your workspace looks.</p>
        </div>

        <div class="setting-row">
          <div class="setting-copy">
            <h3>Theme</h3>
            <p>Choose your preferred color scheme.</p>
          </div>

          <div class="theme-options" role="group" aria-label="Theme">
            {#each ['light', 'dark', 'system'] as option}
              <button
                class="theme-option"
                class:selected={theme === option}
                onclick={() => {
                  theme = option as Theme;
                  applyTheme();
                }}
                aria-pressed={theme === option}
              >
                <span
                  class="theme-preview"
                  class:dark-preview={option === 'dark'}
                  class:system-preview={option === 'system'}
                >
                  {#if option === 'light'}☼{:else if option === 'dark'}☾{:else}◐{/if}
                </span>
                <span>{option[0].toUpperCase() + option.slice(1)}</span>
              </button>
            {/each}
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-copy">
            <h3>Accent color</h3>
            <p>Choose the highlight color used across your workspace.</p>
          </div>

          <div class="accent-options" role="group" aria-label="Accent color">
            {#each accents as accent}
              <button
                class="accent-swatch"
                class:chosen={accentColor === accent.value}
                style:--swatch={accent.value}
                onclick={() => {
                  accentColor = accent.value;
                  applyTheme();
                }}
                aria-label={accent.name}
                aria-pressed={accentColor === accent.value}
                title={accent.name}
              >
                {#if accentColor === accent.value}✓{/if}
              </button>
            {/each}
          </div>
        </div>

        <div class="setting-row">
          <div class="setting-copy">
            <h3>Compact layout</h3>
            <p>Reduce spacing to fit more content on screen.</p>
          </div>

          <button
            class="toggle"
            class:enabled={compactMode}
            role="switch"
            aria-checked={compactMode}
            aria-label="Compact layout"
            onclick={() => {
              compactMode = !compactMode;
              applyTheme();
            }}
          >
            <span></span>
          </button>
        </div>
      </section> -->

      <!-- <section id="behavior" class="settings-section">
        <div class="section-heading">
          <h2>Behavior</h2>
          <p>Configure how the application responds.</p>
        </div>

        <div class="setting-row">
          <div class="setting-copy">
            <h3>Enter to send</h3>
            <p>Send messages with Enter instead of requiring a button click.</p>
          </div>

          <button
            class="toggle"
            class:enabled={enterToSend}
            role="switch"
            aria-checked={enterToSend}
            aria-label="Enter to send"
            onclick={() => (enterToSend = !enterToSend)}
          >
            <span></span>
          </button>
        </div>

        <div class="setting-row">
          <div class="setting-copy">
            <h3>Sound effects</h3>
            <p>Enable sounds for supported application events.</p>
          </div>

          <button
            class="toggle"
            class:enabled={soundEnabled}
            role="switch"
            aria-checked={soundEnabled}
            aria-label="Sound effects"
            onclick={() => (soundEnabled = !soundEnabled)}
          >
            <span></span>
          </button>
        </div>
      </section> -->

      <section id="workspace" class="settings-section">
        <div class="section-heading">
          <h2>Workspace</h2>
          <p>Manage desktop application preferences.</p>
        </div>

        <!-- API Credentials Row -->
        <div
          class="setting-row credentials-row"
          style="flex-direction: column; align-items: flex-start; gap: 0.75rem;"
        >
          <div class="setting-copy">
            <h3>Google AI Studio Key</h3>
            <p>
              Configure your personal developer key to authenticate with Gemini model runtimes
              locally.
            </p>
          </div>

          <div style="display: flex; gap: 0.5rem; width: 100%;">
            <input
              type="password"
              bind:value={apiKey}
              placeholder="AIzaSy..."
              style="flex: 1; padding: 0.5rem 0.75rem; border-radius: 4px; border: 1px solid #ccc; font-family: monospace; font-size: 0.9rem;"
            />
          </div>
        </div>
        <div
          class="setting-row credentials-row"
          style="flex-direction: column; align-items: flex-start; gap: 0.75rem;"
        >
          <div class="setting-copy">
            <h3>Model Name</h3>
            <p>
              Configure your personal developer key to authenticate with Gemini model runtimes
              locally.
            </p>
          </div>

          <div style="display: flex; gap: 0.5rem; width: 100%;">
            <input
              type="text"
              bind:value={modelName}
              placeholder="AIzaSy..."
              style="flex: 1; padding: 0.5rem 0.75rem; border-radius: 4px; border: 1px solid #ccc; font-family: monospace; font-size: 0.9rem;"
            />
          </div>
        </div>
        <div
          class="setting-row credentials-row"
          style="flex-direction: column; align-items: flex-start; gap: 0.75rem;"
        >
          <div class="setting-copy">
            <h3>DB History Key</h3>
            <p>
              Configure your personal developer key to authenticate with to encrypt database
              history.
            </p>
          </div>

          <div style="display: flex; gap: 0.5rem; width: 100%;">
            <input
              type="password"
              bind:value={dbHistoryKey}
              placeholder="AIzaSy..."
              style="flex: 1; padding: 0.5rem 0.75rem; border-radius: 4px; border: 1px solid #ccc; font-family: monospace; font-size: 0.9rem;"
            />
          </div>
        </div>
        <div class="mcp-config">
          <div class="mcp-config-header">
            <span class="mcp-config-title">MCP Configuration</span>

            {#if editing}
              <div class="mcp-config-actions">
                <button class="mcp-config-cancel" onclick={cancelEdit} disabled={loading}>
                  Cancel
                </button>

                <button class="mcp-config-save" onclick={saveConfig} disabled={loading}>
                  {loading ? 'Saving...' : '✓'}
                </button>
              </div>
            {:else}
              <button class="mcp-config-edit" onclick={() => (editing = true)} disabled={loading}>
                Edit
              </button>
            {/if}
          </div>

          {#if loading}
            <p class="mcp-config-loading">Loading configuration...</p>
          {:else if editing}
            <textarea class="mcp-config-editor" bind:value={config} spellcheck="false"></textarea>
          {:else}
            <pre class="mcp-config-preview">{config}</pre>
          {/if}

          {#if error}
            <p class="mcp-config-error">{error}</p>
          {/if}
        </div>

        <!-- About System Meta Row -->
        <div class="setting-row about-row">
          <div class="setting-copy">
            <h3>Defendrite Workspace</h3>
            <p>Desktop workspace · Local Agent</p>
          </div>
          <span class="version">Preferences v1</span>
        </div>
      </section>
    </main>
  </div>
</div>

<style>
  :global(*) {
    box-sizing: border-box;
  }

  :global(body) {
    margin: 0;
    background: #faf9f6;
    color: #302f2a;
    font-family:
      Inter,
      -apple-system,
      BlinkMacSystemFont,
      'Segoe UI',
      sans-serif;
  }

  :global(button),
  :global(input),
  :global(textarea) {
    font: inherit;
  }

  :global(button) {
    cursor: pointer;
  }

  /* Shared design tokens */

  .settings-page {
    --primary: #34332e;
    --primary-hover: #34332e;
    --primary-soft: #f2edfa;

    --line: #eae7df;
    --muted: #89867e;
    --ink: #302f2a;
    --panel: #fffefa;

    --border-color: #e5e2db;
    --accent-color: var(--primary);

    width: 100%;
    max-width: 1440px;
    min-height: 100%;
    margin: 0 auto;
    padding: 12px 16px 20px;
    color: var(--ink);
  }

  /* Header */

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--line);
  }

  h1 {
    margin: 0;
    font-family: Georgia, 'Times New Roman', serif;
    font-size: 34px;
    font-weight: 400;
    letter-spacing: -0.8px;
  }

  .subtitle {
    margin: 7px 0 0;
    color: var(--muted);
    font-size: 12px;
    line-height: 1.6;
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  /* Buttons */

  button {
    font: inherit;
    cursor: pointer;
  }

  button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .save-button,
  .reset-button {
    display: flex;
    align-items: center;
    gap: 9px;
    padding: 8px 11px;
    border: 1px solid var(--line);
    border-radius: 6px;
    background: var(--panel);
    color: var(--ink);
    font-size: 11px;
    transition: background 0.15s ease;
  }

  .save-button {
    border-color: var(--primary);
    background: var(--primary);
    color: #fff;
  }

  .save-button:hover {
    border-color: var(--primary-hover);
    background: var(--primary-hover);
  }

  .reset-button:hover {
    background: #f4f2ec;
  }

  kbd {
    padding: 3px 5px;
    border: 1px solid #e5e2db;
    border-radius: 4px;
    background: #f8f7f3;
    color: var(--muted);
    font-family: inherit;
    font-size: 9px;
  }

  .save-button kbd {
    border-color: #9479c2;
    background: #68469f;
    color: #fff;
  }

  /* Settings content */

  .settings-content {
    min-width: 0;
    padding: 12px 4px;
  }

  .settings-section {
    margin-bottom: 26px;
    scroll-margin-top: 20px;
  }

  .section-heading {
    margin-bottom: 8px;
  }

  .section-heading h2 {
    margin: 0;
    font-size: 14px;
    font-weight: 600;
  }

  .section-heading p {
    margin: 6px 0 0;
    color: var(--muted);
    font-size: 11px;
    line-height: 1.6;
  }

  /* Setting rows */

  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    min-height: 68px;
    padding: 11px 0;
    border-bottom: 1px solid var(--line);
  }

  .setting-copy {
    min-width: 0;
  }

  .setting-copy h3 {
    margin: 0;
    color: var(--ink);
    font-size: 12px;
    font-weight: 550;
  }

  .setting-copy p {
    margin: 5px 0 0;
    color: var(--muted);
    font-size: 10px;
    line-height: 1.6;
  }

  /* Theme options */
  /* 
  .theme-options {
    display: flex;
    gap: 7px;
  }

  .theme-option {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;
    min-width: 56px;
    padding: 6px;
    border: 1px solid transparent;
    border-radius: 7px;
    background: transparent;
    color: var(--muted);
    font-size: 9px;
  }

  .theme-option.selected {
    border-color: #d9c9ed;
    background: var(--primary-soft);
    color: var(--primary);
  }

  .theme-preview {
    display: grid;
    width: 34px;
    height: 25px;
    place-items: center;
    border: 1px solid var(--line);
    border-radius: 5px;
    background: #fbfaf7;
    color: #77746d;
    font-size: 16px;
  }

  .dark-preview {
    border-color: #34342f;
    background: #34342f;
    color: #f5f2eb;
  }

  .system-preview {
    background: linear-gradient(90deg, #fbfaf7 50%, #34342f 50%);
  } */

  /* Accent options */
  /* 
  .accent-options {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .accent-swatch {
    display: grid;
    width: 24px;
    height: 24px;
    place-items: center;
    border: 2px solid transparent;
    border-radius: 50%;
    background: var(--swatch);
    color: white;
    font-size: 11px;
  }

  .accent-swatch.chosen {
    outline: 2px solid var(--swatch);
    outline-offset: 3px;
  } */

  /* Toggle */
  /* 
  .toggle {
    position: relative;
    flex-shrink: 0;
    width: 34px;
    height: 20px;
    padding: 2px;
    border: 0;
    border-radius: 20px;
    background: #d9d6ce;
    transition: background 150ms;
  }

  .toggle span {
    display: block;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: white;
    box-shadow: 0 1px 3px rgb(0 0 0 / 15%);
    transition: transform 150ms;
  }

  .toggle.enabled {
    background: var(--primary);
  }

  .toggle.enabled span {
    transform: translateX(14px);
  } */

  /* Version */

  .version {
    flex-shrink: 0;
    color: var(--muted);
    font-size: 10px;
  }

  /* MCP configuration */

  .mcp-config {
    display: flex;
    flex-direction: column;
    gap: 11px;
    width: 100%;
  }

  .mcp-config-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }

  .mcp-config-title {
    font-size: 14px;
    font-weight: 600;
  }

  .mcp-config-actions {
    display: flex;
    align-items: center;
    gap: 7px;
  }

  .mcp-config-edit,
  .mcp-config-cancel,
  .mcp-config-save {
    padding: 6px 11px;
    border: 1px solid var(--border-color);
    border-radius: 6px;
    background: var(--panel);
    color: var(--ink);
    font-size: 11px;
    cursor: pointer;
  }

  .mcp-config-edit:hover,
  .mcp-config-cancel:hover {
    background: #f4f2ec;
  }

  .mcp-config-save {
    min-width: 36px;
    border-color: var(--primary);
    background: var(--primary);
    color: white;
  }

  .mcp-config-save:hover {
    border-color: var(--primary-hover);
    background: var(--primary-hover);
  }

  .mcp-config-edit:disabled,
  .mcp-config-cancel:disabled,
  .mcp-config-save:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .mcp-config-editor,
  .mcp-config-preview {
    width: 100%;
    min-height: 260px;
    max-height: 500px;
    box-sizing: border-box;
    padding: 12px;
    border: 1px solid var(--border-color);
    border-radius: 7px;
    background: #202020;
    color: #d4d4d4;
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 12px;
    line-height: 1.6;
    overflow: auto;
    white-space: pre;
    tab-size: 2;
  }

  .mcp-config-editor {
    resize: vertical;
    outline: none;
  }

  .mcp-config-editor:focus {
    border-color: var(--primary);
  }

  .mcp-config-preview {
    margin: 0;
  }

  .mcp-config-loading {
    padding: 18px;
    text-align: center;
    color: var(--muted);
    font-size: 12px;
  }

  .mcp-config-error {
    margin: 0;
    padding: 9px 11px;
    border-radius: 6px;
    background: #fbefed;
    color: #b44c42;
    font-size: 12px;
  }

  /* Keyboard hint */

  .keyboard-hint {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 18px;
    color: var(--muted);
    font-size: 10px;
  }

  /* Accessibility */

  :global(button:focus-visible),
  :global(input:focus-visible),
  :global(textarea:focus-visible) {
    outline: 2px solid var(--primary);
    outline-offset: 2px;
  }

  /* Responsive */

  @media (max-width: 760px) {
    .settings-page {
      padding: 18px 14px;
    }

    .page-header {
      align-items: flex-start;
    }

    .settings-layout {
      grid-template-columns: 1fr;
      gap: 14px;
    }

    .setting-row {
      align-items: flex-start;
      flex-wrap: wrap;
      gap: 10px;
    }
  }

  @media (max-width: 480px) {
    .page-header {
      flex-direction: column;
      gap: 12px;
    }

    .header-actions {
      width: 100%;
    }

    .header-actions button {
      flex: 1;
      justify-content: center;
    }

    .mcp-config-header {
      align-items: flex-start;
    }

    .mcp-config-editor,
    .mcp-config-preview {
      min-height: 220px;
    }
    /* 
    .theme-options {
      width: 100%;
      justify-content: space-between;
    } */
  }

  /* Reduced motion */

  @media (prefers-reduced-motion: reduce) {
    :global(*) {
      scroll-behavior: auto !important;
      transition: none !important;
      animation: none !important;
    }
  }
</style>
