# ◉ GreeniPlanner
### v1.2.1

**Plant a goal. Watch it branch. Don't water it with vague intentions.**

---

## What is this thing

Most goal-tracking apps want you to feel like you're filling out a tax form. GreeniPlanner wants you to feel like you're running a small, well-organized, slightly intimidating garden.

You plant a **Long-term Goal** at the top. It grows **Medium-term** and **Short-term** branches. In a separate space, you manage **Tasks** (and nested **Sub-tasks**) which can be linked to any level of your Goal tree. Daily execution happens in **ToDos**, recurring habits in **Routines**, and thoughts/lists in **Notes & Lists** — five rooms, no skipping steps — because "I'll just wing it" is how a 10-year goal quietly turns into a browser tab you never close.

```
🌳 Long-term Goal
 └── 🌿 Medium-term Goal
      └── 🌱 Short-term Goal

[ Separate Tasks Space ]
📋 Task (Optionally linked to Goal)
 └── 🔹 Sub-task

☀️ ToDos (daily planner)
↻ Routines
📝 Notes & Lists
```

No accounts. No cloud. No "upgrade to Pro to see your own data." Everything lives on your machine, under your control.

---

## The rooms in this house

### 🗂️ Goals — the long game
Click **+ New Goal** to plant a Long-term, Medium-term, or Short-term goal. Each goal card shows its description directly under the title and its actions (edit, delete, and the `+` buttons to add a child branch) always visible in the header — no expanding needed. Nested sub-goals render indented beneath.

Every goal gets:
- A **Name** (required)
- A **Description** (optional, shown directly under the title)
- A **Timeframe** — Year / Month / Day boxes, fill whichever applies
- A **Date** — picked from the in-app calendar (optional)

Drag the **≡** handle on any card to reorder it within its group. Goals, sub-goals, and everything else is reorderable.

### ✅ Tasks — the breakdown
A dedicated tab for **Tasks** and **Sub-tasks**. Tasks can be linked to a Goal (Long/Medium/Short) so you can see which goal a task belongs to. Sub-tasks nest under their parent Task. Each Task gets a Start/End Date (auto-calculates Duration), a Priority (High / Medium / Chill), and an optional Goal link. Every card uses the same always-visible header pattern as Goals: description under the title, actions in the header.

### ☀️ ToDos — the right-now
A date strip at the top lets you flip through days like a desk calendar (`⌘3` / `Alt+3`). Each day is its own list. Tasks on that day only.

Each daily task gets one of four moods:

| Priority | Vibe |
|---|---|
| 🔴 **Kill it!** | This is the thing. Today. |
| 🟡 **Mid Slash!** | Important, not on fire. |
| 🟢 **Final Rip!** | The last push to finish something. |
| 🔵 **Chill Bro!** | Low stakes. Breathe. |

You can also **Copy** or **Move** any daily task to another day using the icon buttons on each row.

### ↻ Routines — the repeat
Routines are tasks that recur on a schedule. Create one from the **Routines** page (reachable via the "≡ Routines" button on the ToDos view), give it a Start Date, Priority, and an optional Goal/Task link, and it shows up as an inline item on every applicable day.

On any given day you can:
- **Check off** a routine (done-state is tracked per day)
- **Edit it for that day only** (writes a one-day override, leaving other days untouched)
- **Remove it from that day only** (adds an exception — it still appears on other days)

### 📝 Notes & Lists — the scratchpad
The **Notes & Lists** tab (`⌘4` / `Alt+4`) holds two kinds of cards:

- **Notes** — a rich-text note (bold / italic / underline / font size / highlight color) with a title.
- **Lists** — a checklist with add/remove rows and a checkbox per item. Tick items off right on the card; done items strike through.

Notes features:
- **Pin** any note to float it to the top of the board (pinned and unpinned notes reorder independently).
- **Edit** and **Delete** from the card header.
- **Drag to reorder** with the ≡ handle (pinned stays with pinned, unpinned with unpinned).

---

## ⌨️ Keyboard Shortcuts & Guides

Click the **`⌨`** button in the topbar or press **`?`**, **`F1`**, or **`⌘/`** (**`Ctrl+/`**) anytime to open the interactive Keyboard Shortcuts Guide:

| Category | Shortcut | Scope & Description |
|---|---|---|
| **Navigation** | `⌘1` / `Alt+1` | **Goals Room** — Jump to long, medium, and short goal trees with live completion stats. |
| | `⌘2` / `Alt+2` | **Tasks Room** — Open dedicated task breakdown, sub-task nesting, and goal linking. |
| | `⌘3` / `Alt+3` | **ToDos Room** — Open daily planner with date navigator strip and mood priorities. |
| | `⌘4` / `Alt+4` | **Notes & Lists Room** — Open scratchpad for rich-text notes, checklists, and pinned cards. |
| | `Tab` / `Shift+Tab` | **Cycle Rooms** — Next room (`Tab`) or previous room (`Shift+Tab`) seamlessly. |
| **Creation** | `⌘N` / `Alt+N` | **Quick New Item** — Open creation form for the active room (Goal, Task, ToDo, or Note). |
| | `⌘Shift+N` | **New Checklist** — Instantly create a checkable list card in Notes & Lists. |
| | `⌘Enter` | **Save / Submit** — Submit modal form or dialog immediately without mouse click. |
| | `⌘S` | **Force Save** — Flush pending debounced state changes to local storage. |
| **Daily Planner** | `T` / `Alt+T` | **Jump to Today** — Return date navigator strip to today's date in ToDos. |
| | `[` / `Alt+←` | **Previous Week** — Shift date strip 7 days back. |
| | `]` / `Alt+→` | **Next Week** — Shift date strip 7 days forward. |
| | `D` / `J` | **Toggle Date Picker** — Open calendar picker dropdown to select any target date. |
| **App Controls** | `⌘Shift+T` | **Toggle Theme** — Switch instantly between Light and Dark brutalist themes. |
| | `⌘Shift+C` | **Calendar System** — Cycle calendar engine: Gregorian ↔ Solar Hijri (Shamsi) ↔ Lunar Hijri. |
| | `⌘F` / `/` | **Search Board** — Focus search bar in Notes & Lists room. |
| | `?` / `F1` / `⌘/` | **Shortcuts Guide** — Open full interactive keyboard shortcuts reference modal. |
| | `Esc` | **Dismiss / Cancel** — Close active modal, drop focus from inputs, or dismiss popovers. |

---

## Things worth knowing

- **🌓 Light / Dark** — top-right corner or `⌘Shift+T`. Pick your contrast.
- **📅 Calendar system** — switch between **Gregorian**, **Solar Hijri (Shamsi/Persian)**, and **Lunar Hijri** from the topbar or `⌘Shift+C`. All calendars across the app adapt instantly. Data is always stored as Gregorian internally.
- **🖼️ Crisp High-Res Icons** — includes a multi-resolution 10-layer Windows ICO (up to 256x256), macOS ICNS, and Android adaptive icon assets for pixel-perfect taskbar, launcher, and start menu display.
- **Everything autosaves.** There is no save button (writes are debounced so rapid changes stay snappy).
- **Deleting a branch deletes what's under it.** GreeniPlanner always asks first, with a proper styled dialog — not a browser popup.
- **Persian, Arabic, and Chinese** text renders correctly in all input fields. No setup needed.
- **Drag to reorder** anything — goals, nested goals, tasks, sub-tasks, daily tasks, routines, and notes — using the ≡ handle on each card.
- **Mobile-friendly** — on phone-width screens the top navigation scrolls, touch targets grow, and modal forms stack vertically.

---

## Built different, on purpose

GreeniPlanner is unapologetically brutalist: hard edges, no rounded corners, no soft shadows pretending to be friendly. JetBrains Mono and Space Grotesk do the talking. It looks like a tool because it is one.

Built with **Tauri v2** — a real native Windows/macOS/Android app, not a browser in disguise. Installer is ~5MB. Installed footprint is ~15MB. Starts fast.

---

*No ads. No tracking. No subscription. Just a tree, a calendar, and whatever you're trying to build.*

**Now go plant something.** 🌱

---

<sub>© 2026 GreeniAI. All rights reserved.</sub>

---

<sub>Building this from source or deploying a new version? See [BUILDING.md](BUILDING.md).</sub>
