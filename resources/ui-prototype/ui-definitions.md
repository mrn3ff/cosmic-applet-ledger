# UI Definitions

These definitions describe the standalone `ledger-ui-prototype.html` exploration. Its provider list, sample usage, and simulated interactions are illustrative. Production behavior lives in `src/app/popup_view.rs` and `src/app/popup_view/`; see the root `README.md` for supported providers and account workflows.

## Overview

The top section of the UI used to select between providers. It displays an icon-only viewport of up to six providers with previous and next controls when more providers are enabled.

## Provider information

The information shown for the currently selected provider. It consists of the provider title, account details, and usage bars.

## Provider title

The name and identity of the currently selected provider.

## Account

The provider account currently being displayed, including its email or key identity and plan information.

## Usage bars

Visual indicators showing how much of a provider’s usage has been consumed within a usage window, such as a session or week.

## Settings

The separate page for configuring YapCap.

## General settings

Global settings that apply across YapCap rather than to one specific provider.

## About

Information about the YapCap application, including its version and update status.

## Account card

The account card shows the selected provider account, plan, live and active status, and last update time. The card can be switched between accounts with the arrow controls and accent-colored position dots in its final row.

## Manage accounts

The Manage accounts row is part of the account card and opens account settings for the current provider. It is visually separated from account switching and remains vertically centered with a trailing arrow.

## Manage providers

The Manage providers page lists every provider in a vertically scrollable list. Each provider has an enable toggle. The page can be opened from the main header provider icon.

## Global settings

Global settings contain refresh interval, panel icon, reset time, and usage amount controls. Each control is a softly merged segmented surface; its selected option has a slightly lighter neutral fill with accent-colored text and a checkmark.

## Updates

When a newer version is available, the About info button displays a red notification dot. The About page shows an update card linking to the YapCap release page.

## Terminology

- **Provider**: An AI coding service shown in the provider selector: Codex, Claude, Cursor, Antigravity, Gemini, Copilot, Minimax, Kimi, or OpenCode Go.
- **Account**: A signed-in identity or API key belonging to a provider.
- **Account card**: The card containing the selected account, plan, status badges, update time, Manage accounts action, and account switcher.
- **Account switcher**: The final row of the account card, containing previous/next arrows and accent-colored position dots.
- **Manage accounts**: The card action that opens account settings for the currently selected provider.
- **Manage providers**: The page listing all providers with enable or disable toggles.
- **Global settings**: Settings that apply across YapCap, including refresh interval, panel icon, reset time, and usage amount.
- **About page**: The standalone application information page, including version, links, developer, license, and update information.
- **Update notification**: The red dot on the info button indicating that a newer version is available.
