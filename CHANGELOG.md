# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com),
and this project adheres to [Semantic Versioning](https://semver.org).

---

## [0.1.0-alpha] - 2026-10-18

### Added

- **Monorepo Workspace Infrastructure:** Configured core multi-crate file layout including structural nodes for `apps/desktop`, `runtime`, `models`, `mcp-servers`, and `packages`.
- **Svelte 5 Runes Chat View:** Implemented a modern, fine-grained reactive desktop user interface utilizing Svelte 5 `$state` runes and native lifecycle `onMount` behaviors.
- **Asynchronous Token Streaming:** Formulated real-time token stream routing using non-blocking background threads (`tokio::spawn`) linked directly to Tauri v2's event broadcast pipeline (`Emitter`).
- **Cross-Platform Storage Directory Setup:** Programmed an automatic local folder deployment routine (`runtime/src/storage/`) leveraging Tauri's `Manager` path APIs to safely point to platform-native app data hubs (`~/Library/Application Support/` on macOS).
- **Workspace System Monitor MCP Server:** Developed a decoupled, stdio-driven Model Context Protocol (MCP) TypeScript plugin baseline utilizing the official `@modelcontextprotocol/sdk`.
- **Repository Community Guidelines:** Added complete documentation maps including `README.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, and `SECURITY.md`.

### Fixed

- **Tauri Manager Scope Trait:** Added missing extension trait `use tauri::Manager;` to fix compiler lookup errors during target workspace evaluation paths.
- **Boilerplate Opener Plugin Sync:** Replaced deprecated `tauri_plugin_shell` linkages with modern `tauri_plugin_opener` references to comply with latest Tauri v2 framework builds.

---

_Initial baseline configuration and project environment wiring._
