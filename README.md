# ◉ GreeniPlanner
### v1.0.8

**Plant a goal. Watch it branch. Don't water it with vague intentions.**

---

## What is this thing

Most goal-tracking apps want you to feel like you're filling out a tax form. GreeniPlanner wants you to feel like you're running a small, well-organized, slightly intimidating garden.

You plant a **Long-term Goal** at the top. It grows **Medium-term** and **Short-term** branches. Those branches grow **Tasks**. Tasks grow **Sub-tasks**. Five levels, top to bottom, no skipping steps — because "I'll just wing it" is how a 10-year goal quietly turns into a browser tab you never close.

```
🌳 Long-term Goal
 └── 🌿 Medium-term Goal
      └── 🌱 Short-term Goal
           └── ✅ Task
                └── Sub-task
```

No accounts. No cloud. No "upgrade to Pro to see your own data." Everything lives on your machine, under your control.

---

## The two rooms in this house

### 🗂️ Goals — the long game
Click **+ New Goal** to plant a Long-term, Medium-term, or Short-term goal. Tasks and Sub-tasks grow from within the tree via the small `+` buttons on each card.

Every goal gets:
- A **Name** (required)
- A **Description** (optional)
- A **Timeframe** — Year / Month / Day boxes, fill whichever applies
- A **Date** — picked from the in-app calendar (optional)

Tasks additionally get:
- A **Start Date** and **End Date** (setting both auto-calculates Duration)
- A **Priority**: High, Medium, or Chill

Drag the **≡** handle on any card to reorder it within its group. Everything in a list — goals, sub-goals, tasks, sub-tasks — is reorderable.

### ☀️ Daily Tasks — the right-now
A date strip at the top lets you flip through days like a desk calendar. Each day is its own list. Tasks on that day only.

Each daily task gets one of four moods:

| Priority | Vibe |
|---|---|
| 🔴 **Kill it!** | This is the thing. Today. |
| 🟡 **Mid Slash!** | Important, not on fire. |
| 🟢 **Final Rip!** | The last push to finish something. |
| 🔵 **Chill Bro!** | Low stakes. Breathe. |

You can also **Copy** or **Move** any daily task to another day using the icon buttons on each row.

---

## Things worth knowing

- **🌓 Light / Dark** — top-right corner. Pick your contrast.
- **📅 Calendar system** — switch between **Gregorian**, **Solar Hijri (Shamsi/Persian)**, and **Lunar Hijri** from the topbar. All calendars across the app adapt instantly. Data is always stored as Gregorian internally.
- **Everything autosaves.** There is no save button.
- **Deleting a branch deletes what's under it.** GreeniPlanner always asks first, with a proper styled dialog — not a browser popup.
- **Persian, Arabic, and Chinese** text renders correctly in all input fields. No setup needed.
- **Drag to reorder** anything — goals, nested goals, tasks, sub-tasks, daily tasks — using the ≡ handle that appears on hover.

---

## Built different, on purpose

GreeniPlanner is unapologetically brutalist: hard edges, no rounded corners, no soft shadows pretending to be friendly. JetBrains Mono and Space Grotesk do the talking. It looks like a tool because it is one.

Built with **Tauri** — a real native Windows app, not a browser in disguise. Installer is ~5MB. Installed footprint is ~15MB. Starts fast.

---

*No ads. No tracking. No subscription. Just a tree, a calendar, and whatever you're trying to build.*

**Now go plant something.** 🌱

---

<sub>© 2026 GreeniAI. All rights reserved.</sub>

---

<sub>Building this from source or deploying a new version? See [BUILDING.md](BUILDING.md).</sub>
