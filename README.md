<div align="center">

# Ledger
(cosmic-applet-ledger)

**A clean, minimalist COSMIC panel applet tracking AI usage, quota, and credits across coding agents and providers.**

*Forked from YapCap by Topi Csarno, redesigned with a flat, minimalist asesthetic inspired by Omarchy.*

</div>

---

# Features

- Native COSMIC Integration: Docks directly into the Pop!_OS COSMIC top panel or dock.
- Local & Private: Communicates directly with provider APIs and local CLI session transcripts. No third-party servers, telemetry, or sync required.
- Multi-Service Tracking:
  - Claude Code — 5h session windows, 7-day weekly caps, and extra usage
  - Codex CLI — 5h and weekly quotas plus credits
  - OpenRouter — Prepaid credits and usage monitoring (in development)
  - OpenCode Go — API key usage tracking
  - Cursor, Copilot, Gemini, Grok, and more
- Clean Display: Minimalist, high-contrast monochrome layout showing token burn, reset countdowns, and balance meters.

# Prerequisites

Make sure you have Rust (1.85+) and the required system development libraries installed:

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install COSMIC build dependencies
sudo apt update
sudo apt install -y build-essential pkg-config libxkbcommon-dev libwayland-dev libfontconfig1-dev libssl-dev just

```
# Build & Install

The build and installation are managed via just:

```bash
# Build the release binary
just build-release

# Install binary, desktop files, and dock directly into your COSMIC panel
just install
```

**To run it in a standalone test window without installing to the panel:**

```bash
cargo run
```

**To uninstall and remove from the panel:**

```bash
just uninstall
```



# License & Attribution

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0). See LICENSE for details.

*Original codebase and provider engine: YapCap © Topi Csarno.*