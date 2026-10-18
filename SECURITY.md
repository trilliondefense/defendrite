# Security Policy

We take the security and privacy of our users, their local environments, and private codebases very seriously. As a desktop agent harness with the capability to execute local system tools and process workspace data, maintaining a rigorous security posture is our highest priority.

## Supported Versions

We actively provide security patches and updates to the latest release channel of the workbench.

| Version | Supported        |
| ------- | ---------------- |
| 2.x     | ✅ Yes (Current) |
| 1.x     | ❌ No            |

---

## 🔒 Security Best Practices for Users

Because this desktop application acts as an orchestrator with local file access and terminal capabilities, please follow these rules to safeguard your environment:

1. **Guard Your API Keys:** Never paste raw API tokens (e.g., Google AI Studio, Anthropic) into public text files or your application frontend layouts. The workbench compiles your secrets safely inside the Rust system layer (`runtime/`), keeping them away from public webview execution paths.
2. **Review Third-Party MCP Servers:** Only connect your desktop interface to trusted Model Context Protocol (MCP) servers. A rogue or malicious MCP server could exploit its Standard I/O channel permissions to run unauthorized commands on your machine.
3. **Verify Open Weights:** When configuring custom weights via the `/models` directory, ensure you download GGUF/Safetensors profiles from verified publishers on Hugging Face.

---

## 🐛 Reporting a Vulnerability

**Please do not open public GitHub issues for security vulnerabilities or bugs.** Instead, help us keep the community safe by following our private disclosure process.

### Our Disclosure Process

1. **Submit Your Report:** Send a detailed security report via email to **defendrite@trilliondefense.com**.
2. **What to Include:**
   - A clear description of the vulnerability or threat model.
   - Step-by-step instructions (or a proof-of-concept script) to reproduce the issue.
   - An assessment of the potential impact (e.g., local file leakage, remote code execution bypass).
3. **Acknowledgment:** A project maintainer will acknowledge your email within **48 hours** and provide a timeline for investigation and patching.
4. **Coordination:** We will work closely with you to fix the vulnerability and will credit you for your discovery upon public release, unless you prefer to remain anonymous.

---

## 🛡️ Dependency Audit Policy

Our monorepo workspace utilizes automated scanning to ensure malicious upstream dependencies do not compromise the desktop bundle layer:

- **Rust Backend:** All underlying crates inside `runtime/` and `models/` are periodically verified using `cargo-audit` to screen for vulnerabilities reported in the Rust Sec Advisory Database.
- **Frontend:** Node.js package dependency trees inside `apps/desktop/` and `mcp-servers/` are monitored using automated lockfile integrity checkers.
