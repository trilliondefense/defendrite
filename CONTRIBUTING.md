# Contributing to the DefendRite

Thank you for your interest in contributing. This platform is built to provide investigators with open-weights models, local tools, and remote APIs to instantly classify illicit material and generate courtroom-ready reports. Because this tool handles highly sensitive child safety datasets (CSAM/CP) and generates legal evidence, all contributions must adhere to strict security, isolation, and verification guidelines.

---

## Code of Conduct & Development Safety

1. **Zero Asset Retention:** Never commit test media, live hashes, or real case metadata to this repository. All testing must be performed using synthetic, structurally valid placeholder data.
2. **Local Privacy First:** The workbench is designed to default to strict local data containment. Features or custom tools that introduce unauthorized external network egress, background telemetry, or unencrypted caching will be rejected.
3. **Responsible Disclosure:** If you discover a security vulnerability during development, do not open a public GitHub issue. Refer strictly to our [Security Policy](Security.md) and report it privately to `defendrite@trilliondefense.com`.

---

## Getting Started (Pre-Release Desktop Workspace)

The workbench is currently in a **pre-v1 pre-release phase**.

### 1. Prerequisites

- **Rust Engine Toolchain:** Latest stable version installed locally.
- **Node.js:** Version 18+ (modern alternatives like `pnpm` or `bun` are fully supported).

### 2. Setup & Local Dev Loop

Fork the repository, clone it locally, and run the following commands to spin up your environment:

```bash
# 1. Install dependencies
cd apps/desktop && npm install

# 2. Launch the desktop development environment
npm run tauri dev
```

---

## Development Guidelines

### Local Data Ingestion & Tool Sandboxing

- **Defensive Parsing:** All classification tools, cross-case engines, and ingestion workflows must treat incoming directories and metadata arrays as completely untrusted input. Use strict validation schemas before running extraction workflows.
- **Tauri Core Isolation:** Avoid exposing unconstrained IPC commands from the frontend to the Rust backend. Keep system interactions scoped down to the minimum necessary file handles.

### Developing Custom Tools & Model Connectors

- If you are building custom modules to generate courtroom reports, ensure all string interpolations and document renders are deterministic and fully guarded against cross-case leakage.
- Remote API integrations must utilize secure local environment variables or standard, encrypted operating system keystores for token management.

---

## 📬 Pull Request (PR) Checklist

Before submitting a PR for review, ensure your branch satisfies the following criteria:

- [ ] All code conforms to standard formatting rules (`cargo fmt` for Rust, `npm run format`/`lint` for frontend assets).
- [ ] No local configuration paths, debug tokens, or hardcoded execution flags are committed.
- [ ] Your changes do not introduce unintended tracking, logging, or third-party analytical phone-homes.
- [ ] All automated tests pass locally, and any additions to the reporting engine include valid mock data tests.

---

## ⚖️ Legal Note

By contributing to this repository, you agree that your contributions will be licensed under the project's open-source license terms. All submitted code must be original work or explicitly cleared for distribution under these boundaries.
