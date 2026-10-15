# Security Policy

## 1. Security Architecture & Threat Model

San Andreas Mod Manager (SAMM) processes untrusted third-party archives and metadata from the internet. The application enforces strict security controls at every boundary:

### Filesystem Protection
- **Path Traversal Mitigation**: All archive entry paths are strictly canonicalized and sanitized. Paths containing `..`, absolute drive letters (`C:\`), UNC prefixes (`\\`), or illegal characters are rejected before extraction.
- **Archive Bomb Mitigation**: Archives exceeding 4 GB uncompressed, containing more than 15,000 entries, or displaying suspicious compression ratios (>150:1) are blocked.
- **Staging Isolation**: Archives are extracted into isolated OS temporary staging folders, never directly into the game directory.
- **Destructive Operation Rollbacks**: All overwritten files are backed up prior to change, with an atomic SQLite recovery journal tracking state transitions.

### Network & Process Security
- **No Automatic Executable Running**: Executable files (`.exe`, `.msi`) and batch scripts (`.bat`, `.cmd`) inside mod archives are **never** executed automatically.
- **No Shell Interpolation**: Processes launched through Tauri use structured parameter arrays without passing raw strings into Windows Command Prompt or PowerShell.
- **Strict Content Security Policy**: Web views enforce CSP restricting script execution to local bundles.

---

## 2. Reporting a Vulnerability

If you discover a security vulnerability in SAMM, please report it privately by contacting the maintainers or opening a private security advisory on GitHub.

Please provide:
- A description of the vulnerability and its potential impact.
- Step-by-step reproduction instructions or a minimal proof-of-concept archive.
- Any suggested remediations.

We appreciate responsible disclosure and aim to acknowledge reports within 48 hours.
