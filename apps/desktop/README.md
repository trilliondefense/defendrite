# Workbench: Desktop Agent Harness

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
