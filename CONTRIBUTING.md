# Contributing to Security AI Workbench

We love open-source contributions! Whether you are fixing a rendering bug in the Svelte 5 frontend, optimizing a tool loop in the Rust runtime, or building a new MCP server plugin, this guide will help you get started quickly and safely.

---

## 🗺️ Monorepo Contribution Map

Before modifying files, make sure your code goes into the correct workspace layer:

- **UI Changes:** All interface, styling, and design features belong in `apps/desktop/src/` (**Svelte 5 / Runes Mode**).
- **Orchestration / Storage Logic:** Core system features, async processing, and database work belong in `runtime/`.
- **Local Inference / Embeddings:** Weight loading or vector handling belongs in `models/`.
- **Plugins / Custom Tools:** Modifying or adding new tool features belongs in `mcp-servers/`.

---

## 🛠️ Development Setup

### 1. Prerequisites

Ensure you have the following installed locally:

- [Rust Toolchain](https://rust-lang.org) (Stable channel)
- [Node.js](https://nodejs.org) (v18+ or modern packagers like `pnpm`/`bun`)

### 2. Fork & Clone

```bash
git clone https://github.com
cd security-ai-workbench
```

### 3. Bootstrap the Workspace

Install the frontend dependencies and build your local MCP tools:

```bash
# Set up the Svelte 5 desktop app
cd apps/desktop && npm install

# Set up the core system tool server
cd ../../mcp-servers/basic-system-server && npm install
```

### 4. Run the Dev Loop

To start the compilation chain and open the application window, run this command from the `apps/desktop/` directory:

```bash
npm run tauri dev
```

---

## 📝 Code Styles & Best Practices

### Frontend (Svelte 5)

- **Use Runes Mode:** Do not use legacy Svelte 2/3/4 state mechanics. State must be declared explicitly using `$state`, `$derived`, or `$effect`.
- **Streaming Compatibility:** Never block the main thread. Push incoming chunks directly into active state arrays using `.push()` to take advantage of Svelte 5's fine-grained reactivity.
- **TypeScript:** All components must use `<script lang="ts">`. Ensure structural types match backend mappings.

### Backend (Rust Crate Ecosystem)

- **Non-Blocking Loops:** AI agent loops and network executions must be spawned off the main Tauri thread via `tokio::spawn`.
- **Cross-Platform Paths:** Never hardcode paths (`/` or `\`). Always resolve paths safely using Tauri’s `app_handle.path().app_data_dir()` to support macOS, Windows, and Linux simultaneously.
- **Run Lints:** Always verify your backend changes pass standard lints before pushing:
  ```bash
  cargo clippy --all-targets
  cargo fmt --check
  ```

---

## 🚀 Submitting Your Changes

1.  **Create a Feature Branch:** `git checkout -b feature/amazing-new-tool`
2.  **Commit with Context:** Make clear, descriptive commit messages (e.g., `feat(runtime): integrate gemini function call router`).
3.  **Test Locally:** Ensure both the SvelteKit app runs cleanly and your Rust workspaces compile without compiler diagnostics or crashes.
4.  **Open a Pull Request:** Describe your changes, identify the target workspace, and link any relevant open issues.

Thank you for helping build a lighter, faster, and completely secure open-source AI desktop client!
