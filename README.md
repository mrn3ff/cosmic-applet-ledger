<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="resources/ledger-icon/svg/ledger-lockup-mark-dark.svg">
  <img src="resources/ledger-icon/svg/ledger-lockup-mark-dark.svg" alt="Ledger" width="360">
</picture>

<br>
<br>

**A flat, minimalist Pop!_OS COSMIC panel applet for tracking AI usage, rate limits, and tokens across coding agents and providers.**

*Forked from YapCap by Topi Csarno, redesigned with a minimalist aesthetic inspired by Omarchy.*

</div>

---

## Features

- **Native COSMIC Integration:** Docks directly into the Pop!_OS COSMIC top panel or dock.
- **Top-Down Dashboard:**
  - **Hero:** Current provider, active plan tier, inline provider switcher (`...`), settings, and manual sync.
  - **LIMITS:** Session and Weekly usage percentages, full-width status bars, and weekly reset countdowns.
  - **TOKENS BY DAY:** 7-day token burn breakdown (`Sun` through `Today`) with stark white highlight for today.
  - **TOKENS BY MODEL:** Proportional horizontal sliders displaying model names and token volumes directly inside the bars.
- **Unified Across Providers:** Consistent layout across all providers.
- **Local & Private:** Communicates directly with local transcripts and provider APIs using secure OS keychain credentials. No third-party servers, telemetry, or external sync.
- **Minimalist Settings:** Streamlined provider enablement toggles and account configuration with zero UI fluff.

---

## Prerequisites

Ensure you have Rust (1.85+) and the required system development libraries installed:

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install COSMIC build dependencies
sudo apt update
sudo apt install -y build-essential pkg-config libxkbcommon-dev libwayland-dev libfontconfig1-dev libssl-dev just
```

---

## Build & Install

The build and installation are managed via `just`:

```bash
# Build the release binary
just build-release

# Install binary, desktop files, and dock directly into your COSMIC panel
just install
```

**Run in a standalone test window with demo data:**

```bash
just run-demo
# or
env LEDGER_DEMO=1 cargo run
```

**Uninstall and remove from the panel:**

```bash
just uninstall
```

---

## License & Attribution

This project is licensed under the Mozilla Public License 2.0 (MPL-2.0). See [LICENSE](LICENSE) for details.

*Original codebase and provider engine: YapCap © Topi Csarno.*
