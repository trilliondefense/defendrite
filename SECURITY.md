# Security

## Reporting a Vulnerability

**Do not open a GitHub issue.** Issues are public, and a report filed there is an immediate public disclosure.

Email **defendrite@trilliondefense.com** with enough detail to reproduce the issue. We will acknowledge receipt within three working days and keep you updated until it is resolved. If you would like credit in the fix, please let us know in your email.

## Pre-Release Status & Scope

This software is currently in a **pre-release phase (pre-v1)**. While it is under active development and not yet an official v1.0.0 release, we take security vulnerabilities seriously.

### In Scope

The core Workbench platform, local ingestion architecture, and custom tool runtime. We prioritize vulnerabilities that compromise the local runtime sandbox, leak sensitive case data across judicial boundaries, compromise remote API credential storage, or allow execution escape during automated classification and report generation.

## DefendRite Boundaries & Threat Model

As a specialized investigation application running open-weights models, local tools, and remote APIs, the platform operates under distinct security assumptions:

- **Ingestion and Classifier Integrity:** Because the workbench performs local data ingestion to instantly classify illicit material, parsing engines must be entirely resilient. Memory corruption, buffer overflows, or arbitrary code execution triggered by analyzing hostile media or maliciously crafted metadata are critical vulnerabilities.
- **API Credential & Remote Tool Isolation:** The workbench coordinates local orchestration with remote APIs. Any flaw that exposes configured API tokens, leaks telemetry to unauthorized external endpoints, or permits cross-case data contamination during correlation is strictly in scope.
- **Courtroom Report & Artifact Tampering:** Generated outputs must be deterministic and structurally secure to maintain evidentiary weight for global jurisdictions. Flaws allowing local privilege escalation, unauthorized modification of cross-case logs, or silent manipulation of generated report payloads represent high-severity boundary breaches.

## For Implementers and Tool Developers

If you are developing custom tools, model connectors, or extensions for the workbench, you must adhere to these compliance obligations:

- **Data Containment:** Never cache, mirror, or leak ingested media, hashes, or investigator metadata outside the explicitly defined local sandbox or intended remote API payloads.
- **Strict Input Validation:** Treat all ingested directory paths, foreign forensic packages, and external model responses as completely untrusted inputs. They must be validated, allowlisted, and sandboxed before execution.
