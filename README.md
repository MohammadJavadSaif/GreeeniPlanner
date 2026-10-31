# ◉ GreeniPlanner
### v1.5.0 — Brutalist Goal, Task, Routine, Note & Expense Manager

**Plant a goal. Watch it branch. Don't water it with vague intentions.**

GreeniPlanner is a fast, local-first, offline-capable personal planner built with **Tauri v2**. No accounts, no cloud lock-in, no subscriptions, and no telemetry — everything lives locally on your device under your complete control.

---

## ✨ What's New in v1.5.0

- 📦 **Cross-Platform Installers & Automated GitHub Releases**: Automated CI/CD workflow builds and publishes native installation packages for **Android (`.apk`)**, **Windows (`.exe` NSIS & `.msi`)**, and **macOS Universal (`.dmg` & `.app.zip` for Apple Silicon + Intel)**.
- 🔄 **Seamless In-Place Android App Updates**: Persistent release signing (`src-tauri/signing/`) combined with a monotonically increasing `versionCode` lets you install new Android APK releases directly over existing installations — **no uninstall required and zero data loss**.
- 🌐 **Full Persian (فارسی) & RTL Support**: Switch seamlessly between **English** and **Persian** in Settings (with automatic system language detection on first launch) and full Right-to-Left (RTL) layout adaptation across every screen, modal, calendar, and card.
- 🖥️ **Light, Dark & System Appearance Modes**: Choose **Light**, **Dark**, or **System** (automatically follows your OS color scheme in real time).
- ⚖️ **Debts & Credits Ledger (`D&C`)**: Create dedicated **Debts & Credits** lists alongside Weekly and Monthly Expense boards to track money owed (**Debt**) and money owed to you (**Credit**) with live net balance calculation.
- 🔔 **Native System Notification Center Integration**: Daily ToDo reminders schedule directly with Android's native `NotificationManager` (`tauri-plugin-notification`) and Service Worker notifications, complete with in-app chimes and quick **Complete** / **Snooze 10m** actions.
- 💾 **Full JSON Backup & Restore**: Export all Goals, Tasks, ToDos, Routines, Notes, Expenses, and preferences to a portable `.json` file and restore anytime from Settings.

---

## 🧭 Core Architecture

GreeniPlanner organizes your life into dedicated rooms so long-term vision connects cleanly to daily execution and financial tracking:

```text
🌳 Long-term Goal
 └── 🌿 Medium-term Goal
      └── 🌱 Short-term Goal

📋 Tasks Room (Separately managed, optionally linked to any Goal)
 └── 🔹 Sub-task

☀️ ToDos (Daily execution planner + native reminders)
↻ Routines (Recurring daily habits with per-day overrides & exceptions)
📝 Notes & Lists (Rich-text notes + interactive checklists)
💳 Expenses & D&C (Weekly/Monthly expense boards + Debts & Credits ledger)
```

---

## 🏛️ The Six Rooms

### 1. 🗂️ Goals — *The Long Game* (`⌘1` / `Alt+1`)
Plant **Long-term**, **Medium-term**, and **Short-term** goals in a collapsible hierarchy.
- **Fields**: Name (required), Description, Timeframe (**Year / Month / Day** boxes), and Target Date.
- **Live Statistics**: Click the chart icon in the header to view completion breakdowns across all goal tiers.
- **Sorting Mode (`↕`)**: Toggle sorting mode to drag and reorder goals within their parent group.

### 2. ✅ Tasks — *The Breakdown* (`⌘2` / `Alt+2`)
A dedicated workspace for actionable **Tasks** and nested **Sub-tasks**.
- **Goal Linking**: Link any Task to a Long-term, Medium-term, or Short-term Goal.
- **Smart Duration**: Pick a **Start Date** and **End Date** from the multi-calendar picker to auto-calculate duration, or enter **Year / Month / Day** manually.
- **Priority Levels**: Tag tasks as **High**, **Medium**, or **Chill** and filter by status (*All*, *Tasks*, *Sub-tasks*, *Pending*, *Done*).

### 3. ☀️ ToDos — *The Right-Now* (`⌘3` / `Alt+3`)
A focused day-by-day execution view featuring an interactive 7-day date strip and a live daily progress bar.
- **Four Daily Mood Priorities**:
  | Priority | Meaning |
  | :--- | :--- |
  | 🔴 **Kill it!** | Top priority. Must happen today. |
  | 🟡 **Mid Slash!** | Important, steady momentum. |
  | 🟢 **Final Rip!** | The final push to finish something off. |
  | 🔵 **Chill Bro!** | Low stakes. Do it when breathing room opens up. |
- **Reminders & Native Notifications**: Attach a reminder time to any ToDo for native OS/phone notifications and in-app audio chimes.
- **Copy & Move**: Duplicate or move any unfinished item to another date in two clicks.

### 4. ↻ Routines — *The Repeat*
Accessible via the **`≡ Routines`** button inside ToDos.
- Define recurring habits with a **Start Date**, **Priority**, **Reminder Time**, and optional **Goal/Task link**.
- **Per-Day Flexibility**: On any date in ToDos, you can check off a routine, **edit it for that single day only** (creating a one-day override), or **remove it from that day only** (adding a single-day exception without affecting future days).

### 5. 📝 Notes & Lists — *The Scratchpad* (`⌘4` / `Alt+4`)
Keep freeform thoughts and structured checklists side by side.
- **Rich-Text Notes**: Format text with bold, italic, underline, font sizes, and highlight colors.
- **Interactive Checklists**: Tick items off directly on the board; completed items automatically sort below active items.
- **Pin & Search**: Pin important cards to the top (`📌`) and filter instantly with live search (`⌘F` or `/`).

### 6. 💳 Expenses & Debts/Credits (`D&C`) — *The Ledger* (`⌘5` / `Alt+5`)
Simple, fast personal finance tracking without bloated spreadsheets:
- **Weekly & Monthly Expense Boards**: Organize spending by calendar month/week in your chosen currency (`$`, `€`, `£`, `₺`, `IRR`, `تومان`, etc.) with automatic daily, weekly, and monthly totals.
- **Debts & Credits (`D&C`) Lists**: Create dedicated ledgers where each entry records a **Name**, **Amount**, **Type** (*Debt / Money Owed* vs. *Credit / Owed to You*), optional **Date**, and **Settled** toggle.

---

## 🌐 Language, RTL & Multi-Calendar Engine

All localization and appearance options live inside the **Settings (`⚙`)** modal in the top-right corner:

- **Bilingual UI (English & Persian)**:
  - Defaults automatically to **Persian (`فارسی`)** if your device language is Persian, or **English** otherwise.
  - Selecting **Persian** switches the entire application layout, typography (`Vazirmatn`), alignments, shadows, and popovers to **RTL (Right-to-Left)**.
- **Three Calendar Systems** (`⌘Shift+C`):
  - **Gregorian (`GR`)** — Standard international calendar
  - **Shamsi / Jalali (`SH`)** — Solar Hijri (Persian) calendar
  - **Hijri (`HI`)** — Islamic Lunar calendar
  - *Note*: Dates are stored canonically in ISO Gregorian format under the hood, so switching calendars converts every date across the entire app instantaneously.
- **Appearance Modes** (`⌘Shift+T`):
  - Switch between **Light**, **Dark**, and **System** appearance modes.

---

## 📥 Installation & Platform Packages

Pre-built native installers for **Android**, **Windows**, and **macOS** are generated automatically by GitHub Actions and published to [**GitHub Releases**](../../releases).

| Platform | Package Format | File Name Pattern | Highlights |
| :--- | :--- | :--- | :--- |
| **Android** | Universal APK | `GreeniPlanner_1.5.0_android_universal.apk` | Supports `arm64-v8a`, `armeabi-v7a`, `x86`, `x86_64`. **In-place upgrades supported.** |
| **Windows** | NSIS Setup & MSI | `GreeniPlanner_1.5.0_x64-setup.exe`<br>`GreeniPlanner_1.5.0_x64_en-US.msi` | Supports per-user or system-wide installation (`x64`) and in-place updates. |
| **macOS** | Universal DMG & App | `GreeniPlanner_1.5.0_universal.dmg`<br>`GreeniPlanner_1.5.0_macos_universal.app.zip` | Native Universal Binary for both **Apple Silicon (M1–M4)** and **Intel** Macs. |

### 🔄 Updating on Android (Preserving App Data)
GreeniPlanner's Android APK is signed with a persistent release certificate and uses a monotonically increasing `versionCode`.
- **To update**: Download the latest `.apk` and install it directly over your existing GreeniPlanner app.
- **Do NOT uninstall** the previous version — Android will upgrade the app in place and preserve all of your Goals, Tasks, ToDos, Routines, Notes, and Expenses.
- *(Tip: You can also export a `.json` backup anytime via **Settings → Data Backup & Restore**).*

### 🍎 First Launch on macOS (Gatekeeper)
Because the macOS build uses ad-hoc signing (`-`) rather than a paid Apple Developer ID certificate:
1. Drag **GreeniPlanner.app** to your `/Applications` folder.
2. On first launch, **Right-click (or Ctrl-click) `GreeniPlanner.app` → click `Open`**, then confirm **Open** in the dialog.
3. Alternatively, clear the quarantine attribute in Terminal:
   ```bash
   xattr -cr /Applications/GreeniPlanner.app
   ```

---

## ⚙️ Automated GitHub Actions CI/CD

The repository includes two GitHub Actions workflows in `.github/workflows/`:

1. **`build-all-platforms.yml` (Build & Release — Android, Windows, macOS)**:
   - Triggers automatically on pushes to `main`/`master`, version tags (`v*`), or manual dispatch (`workflow_dispatch`).
   - **`verify-metadata`**: Validates version alignment across `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml`.
   - **`build-android`**: Compiles the universal Android `.apk`, calculates a strictly increasing `versionCode` (`base_version_code + github.run_number`), and signs the APK using the persistent credentials in `src-tauri/signing/`.
   - **`build-windows`**: Builds both the NSIS `.exe` installer and WiX `.msi` package on `windows-latest`.
   - **`build-macos`**: Compiles a Universal macOS binary (`aarch64-apple-darwin` + `x86_64-apple-darwin`) and packages `.dmg` and `.app.zip` bundles on `macos-latest`.
   - **`publish-release`**: Consolidates all platform installers and publishes/updates the **GitHub Release** automatically.

2. **`build-android.yml` (Android APK Quick Build)**:
   - Manual `workflow_dispatch` workflow for quickly testing Android-only APK builds without running desktop jobs.

---

## 🛠️ Building From Source

### Prerequisites
- **Node.js** `>= 20` & **npm**
- **Rust** stable toolchain (`rustup`)
- **Platform SDKs**:
  - *Windows*: Microsoft Visual Studio C++ Build Tools & WebView2
  - *macOS*: Xcode Command Line Tools (`xcode-select --install`)
  - *Android*: JDK 17, Android SDK (Platform 35, Build-Tools 35.0.0), and NDK 27

### Commands

```bash
# Install dependencies
npm install

# Run web preview in development mode (port 3000)
npm run dev

# Validate frontend production build
npm run build

# Build desktop installer for current OS
npm run tauri:build

# Build Windows NSIS (.exe) & WiX (.msi) installers
npm run tauri:windows:build

# Build macOS Universal (.dmg & .app) bundles
npm run tauri:macos:build

# Initialize & build Android APK locally
npm run tauri:android:init
npm run tauri:android:build
```

---

## ⌨️ Keyboard Shortcuts Reference

Press **`?`**, **`F1`**, or **`⌘/`** (**`Ctrl+/`**) anywhere in the app to open the interactive Keyboard Shortcuts modal:

| Category | Shortcut | Action |
| :--- | :--- | :--- |
| **Navigation** | `⌘1`–`⌘5` / `Alt+1`–`Alt+5` | Switch directly to **Goals (1)**, **Tasks (2)**, **ToDos (3)**, **Notes (4)**, or **Expenses (5)** |
| | `Tab` / `Shift+Tab` | Cycle forward or backward through the topbar rooms |
| **Creation & Saving** | `⌘N` / `Ctrl+N` / `Alt+N` | Open the **New Item** modal for the active room |
| | `⌘Shift+N` / `Ctrl+Shift+N` | Create a new **Checklist** immediately when in Notes & Lists |
| | `⌘Enter` / `Ctrl+Enter` | Submit the active modal form or confirm dialog |
| | `⌘S` / `Ctrl+S` | Force-flush pending state changes to local storage |
| **Daily Planner** | `T` / `Alt+T` | Jump the date strip back to **Today** |
| | `[` or `←` / `]` or `→` | Shift the date navigator strip **7 days backward / forward** |
| | `D` / `J` | Open or close the **Jump to Date** calendar popover |
| **App Controls** | `⌘Shift+T` / `Ctrl+Shift+T` | Cycle appearance theme (**Light → Dark → System**) |
| | `⌘Shift+C` / `Ctrl+Shift+C` | Cycle calendar system (**Gregorian → Shamsi → Hijri**) |
| | `⌘F` / `Ctrl+F` / `/` | Focus the search bar in **Notes & Lists** or **Expenses** |
| | `?` / `F1` / `⌘/` | Open the **Keyboard Shortcuts Guide** |
| | `Esc` | Close active modal, dialog, calendar popover, or detail view |

---

*No ads. No tracking. No subscription. Just a tree, a calendar, a ledger, and whatever you're trying to build.*

**Now go plant something.** 🌱

---

<sub>© 2026 GreeniAI. All rights reserved.</sub>
