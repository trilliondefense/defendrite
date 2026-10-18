# Security AI Workbench: Desktop Agent Harness

An enterprise-grade, open-source desktop agent harness built with **Tauri v2**, **Svelte 5 (Runes Mode)**, and **Rust**. The architecture is structured as a modular monorepo designed to support local open-weights inference, remote model orchestration, and extensible tool executions via the **Model Context Protocol (MCP)**.

---

## 🏗️ Repository Architecture

This project is organized as a Cargo and NPM monorepo workspace to keep core orchestration logic strictly isolated from the UI runtime layer:

```text
├── apps/
│   └── desktop/               # Tauri v2 wrapper + Svelte 5 (Runes) frontend chat UI
├── runtime/                   # Core agent loop controller room (Rust crate)
│   └── src/storage/           # Cross-platform secure app data storage module
├── mcp-servers/               # Modular tool execution plugins (Node.js/TypeScript)
│   └── basic-system-server/   # Stdio-based system diagnostic tool server
├── models/                    # Built-in local fallback ML engine / HF downloader (Rust crate)
├── packages/                  # Shared data schemas, types, and manifest configurations
├── training/                  # Workspace area for fine-tuning scripts and datasets
├── tests/                     # Integration tests across workspaces
└── Cargo.toml                 # Root Cargo Workspace configuration
```

---

## 🚀 Key Features

- **Svelte 5 Runes Streaming:** Modern fine-grained reactivity driving a real-time token-streaming interface modeled after Claude Desktop.
- **Zero-Bloat Tauri Runtime:** Operates natively on macOS (WebKit) and Windows/Linux (Webview2), idling at a minimal ~30MB-50MB RAM footprint.
- **Dual-Model Execution Paths:**
  - _Cloud Orchestration:_ Drives complex reasoning loops via high-throughput external APIs (e.g., Google AI Studio Gemini).
  - _Local Micro-Models:_ Uses the local `/models` Rust library to download and run embedding models (like Nomic/BERT) for secure, offline code vectorization.
- **Native MCP Server Integration:** Seamless process-level communication over Standard I/O channel transports to execute sandboxed system tools securely.

---

## 🛠️ The 3-Step Agent Tool Loop

When this system integrates with a tool-enabled API like Gemini, execution flows across the folder workspaces as follows:

1.  **Declaration:** The `runtime` crate queries active Node.js processes in `mcp-servers/` for tool manifests and registers them alongside your chat prompt to the LLM API.
2.  **Interception:** The model determines if it needs local environment data. If it does, it halts text output and returns a structured `functionCall` payload.
3.  **Local Execution:** The Rust `runtime` catches the payload, pipes it directly into the local `mcp-servers/` runner over `StdioTransport`, and safely updates your secure `runtime/src/storage/` index before routing the final response back to the Svelte frontend layout via asynchronous Tauri `Emitter` channels.

---

## ⚡ Quick Start

### Prerequisites

- [Rust Engine Toolchain](https://rust-lang.org) (Latest stable version)
- [Node.js](https://nodejs.org) (v18+ or modern alternative packagers like `pnpm`/`bun`)

### Setup Commands

1. **Install Frontend Dependencies:**

   ```bash
   cd apps/desktop
   npm install
   ```

2. **Setup MCP Server Plugins:**

   ```bash
   cd ../../mcp-servers/basic-system-server
   npm install
   ```

3. **Launch the Desktop Environment Dev Loop:**
   ```bash
   cd ../../apps/desktop
   npm run tauri dev
   ```

---

## 🔒 Security & Privacy Defaults

- **Isolated Environment Variables:** Desktop builds on macOS/Linux use custom patches to cleanly bridge user environment contexts without leaking raw shell data into frontend scripts.
- **Secured Secrets Storage:** API tokens are stored off the webview rendering thread, remaining compiled within the Rust system backend to prevent Cross-Site Scripting (XSS) vulnerability exposures.
