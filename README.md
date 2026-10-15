# San Andreas Mod Manager (SAMM)

[![Build & Tests](https://img.shields.io/badge/tests-passing-brightgreen.svg)]()
[![Platform](https://img.shields.io/badge/platform-Windows%2010%20%2F%2011%20(64--bit)-blue.svg)]()
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)
[![Framework](https://img.shields.io/badge/Tauri-2.1-orange.svg)]()

**San Andreas Mod Manager (SAMM)** is a real, production-minded open-source desktop application that makes discovering, evaluating, installing, managing, and safely removing mods for the classic PC version of **Grand Theft Auto: San Andreas** transparent, reliable, and recoverable.

---

## 1. Core Engineering Principles

- **Automate reliable operations, explain uncertain ones**: Never silently guess where game files belong.
- **Evidence-Based Game Detection**: Distinguishes verified Classic PC 1.0 executables from patched, Steam, Rockstar Launcher, or unknown modified binaries using SHA-256 cryptographic fingerprints and file structural checks.
- **Transactional Filesystem Safety**: Overwritten files are backed up to a dedicated recovery directory before modification. An atomic SQLite recovery journal ensures that if an installation is interrupted, power fails, or an error occurs, the previous game state is restored.
- **Zero Silent Script Execution**: Automated execution of arbitrary downloaded executables (`.exe`), installers (`.msi`), or batch scripts (`.bat`) is strictly prohibited.
- **Multi-Source Discovery with Honest Capabilities**: Integrates real GTA modding repositories (GTAinside, LibertyCity, MixMods, GTAGarage) and first-class local archive imports. Sources that require browser-based downloads or web sessions are transparently declared rather than simulated with fake APIs.

---

## 2. Key Features

- **Game Detection & Version Verification**:
  - Automatically scans Steam libraries, Rockstar Games Launcher, and classic retail directories.
  - Inspects `gta_sa.exe` hashes, file sizes, and core RenderWare folders (`data/gta.dat`, `models/gta3.img`, `audio/streams/`).
  - Identifies presence of CLEO framework, ASI loaders (`vorbisFile.dll`/`d3d9.dll`), and ModLoader.
  - Rejects incompatible Definitive Edition (Unreal Engine) installs with transparent explanations.
- **Pre-Installation Safety Inspection**:
  - Displays full plan before touching the game directory: planned file operations, rules, file types, destination states (`NEW` vs `OVERWRITE`), and missing prerequisites.
- **Safe Installation Engine**:
  - Automated placement for recognized CLEO scripts (`.cs` -> `cleo/`, `.fxt` -> `cleo/cleo_text/`), ASI plugins (`.asi` -> game root), and ModLoader sandboxed folders (`modloader/<mod_name>/`).
  - Clean uninstall: restores the original backed-up file or cleanly removes newly added files without leaving orphan assets.
- **Conflict & Collision Detection**:
  - Tracks file ownership across all installed mods and alerts users when multiple mods claim the same destination path.
- **Profiles & Reproducible Manifests**:
  - Maintain isolated mod configurations (e.g. *Vanilla Baseline*, *Visual Enhancements*, *Total Conversion*).
  - Export and import reproducible mod list JSON manifests without redistributing copyrighted game assets.
- **Download Subsystem**:
  - Download progress, transfer speed, pause/resume (HTTP Range requests), and automated SHA-256 checksum verification.
- **Direct Game Launcher**:
  - Launches the game with selected profile and custom startup arguments (`-window`, `-novid`, `-width 1920 -height 1080`).

---

## 3. Technology Stack

- **Desktop Shell**: [Tauri 2](https://tauri.app/) (Windows MSVC backend)
- **Backend**: Rust 1.99+ (Tokio, Rusqlite with bundled SQLite, Reqwest, Zip, Sha2, Chrono)
- **Frontend**: React 18, TypeScript 5, Vite 5
- **UI & Styling**: Tailwind CSS (dark neutral desktop theme inspired by professional file managers and dev tools), Lucide Icons
- **Database**: SQLite 3 with WAL mode, parameterized queries, and transactional migrations

---

## 4. Architecture

```
React UI (TypeScript / Tailwind)
    │
    ▼ (Typed Tauri IPC Commands)
src-tauri/src/commands.rs
    │
    ├── game_detection.rs    -> SHA-256 verification & RenderWare directory inspector
    ├── archives.rs          -> Path traversal protection & safe zip inspector
    ├── installation.rs      -> Pre-install planner, recovery journal & atomic rollback
    ├── conflicts.rs         -> Destination path collision detection & ownership map
    ├── profiles.rs          -> Profile switching & JSON manifest export/import
    ├── sources/             -> Multi-source adapters (GTAinside, LibertyCity, MixMods, GTAGarage, Manual)
    ├── downloads.rs         -> Multi-job download manager with SHA-256 checks
    ├── database.rs          -> SQLite schema migrations & repository layer
    └── game_launch.rs       -> Direct executable spawn without shell injection
```

---

## 5. Building & Running Locally

### Prerequisites
- **Operating System**: Windows 10 or Windows 11 (64-bit)
- **Rust Toolchain**: `rustup` with `stable-x86_64-pc-windows-msvc` (`rustc` and `cargo`)
- **Visual Studio C++ Build Tools**: MSVC v142 / v143 compiler tools
- **Node.js**: v18+ (tested on Node v22)

### Steps

1. **Clone the repository**:
   ```bash
   git clone https://github.com/samm-modmanager/samm.git
   cd samm
   ```

2. **Install frontend dependencies**:
   ```bash
   npm install
   ```

3. **Run automated test suites**:
   ```bash
   # Run Rust unit & integration tests
   cd src-tauri
   cargo test --lib
   cd ..

   # Run TypeScript domain tests
   npx tsx --test src/tests/domain.test.ts
   ```

4. **Build the production application**:
   ```bash
   npm run build
   cd src-tauri
   cargo build --release
   ```
   The compiled standalone Windows desktop binary will be available at:
   `src-tauri/target/release/samm.exe`

---

## 6. Disclaimer & Legal Notice

San Andreas Mod Manager (SAMM) is an independent open-source utility developed by community members. It is not affiliated with, endorsed by, or sponsored by Rockstar Games, Take-Two Interactive, or any related entity.

SAMM does not distribute copyrighted game executables, modified game binaries, or unauthorized assets. All trademarks, game titles, and assets belong to their respective owners.
