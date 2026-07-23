# Building GreeniPlanner — Windows, macOS & Android

This project builds real, installable apps for **Windows**, **macOS**, and **Android** — all from one GitHub repository, using GitHub's free build servers. You don't need Rust, Xcode, Android Studio, or any of the underlying tools installed on your own computer.

## What you get

| Platform | File | Approx. size | Notes |
|---|---|---|---|
| Windows | `GreeniPlanner_x64-setup.exe` | ~5 MB installer, ~15 MB installed | Adds to Start Menu + Add/Remove Programs, with uninstaller |
| macOS | `GreeniPlanner.dmg` | ~8–12 MB | Universal — runs on both Apple Silicon and Intel Macs |
| Android | `app-universal-debug.apk` | ~10–15 MB | Installs directly on any Android phone/tablet |

All three are built from the same codebase and use the operating system's own built-in web engine (WebView2 on Windows, WKWebView on macOS, Android System WebView) instead of bundling a browser — that's why they're so small compared to Electron-based apps.

---

## One-time setup

**1. Create a free GitHub account** — [github.com/signup](https://github.com/signup)

**2. Create a new repository**
- Go to [github.com/new](https://github.com/new)
- Name it `greeniplanner` (or anything you like)
- Set it to **Public** (this makes GitHub Actions build minutes free)
- Do **not** initialize with a README — we already have one

**3. Upload the project files**

⚠️ **Important:** GitHub's drag-and-drop upload hides folders that start with a dot, which means the `.github` folder (containing the build instructions) will silently not upload if you drag-and-drop everything at once. Do it in two steps:

- **Step A:** Drag and drop everything *except* the `.github` folder: `src-tauri/`, `ui/`, `icon-source.png`, `package.json`, `.gitattributes`, `.gitignore`, `BUILDING.md`, `README.md`, `README.html`. Commit.
- **Step B:** Create the workflow file manually:
  - Click **Add file → Create new file**
  - In the filename box, type exactly: `.github/workflows/build.yml`
    (typing the slashes makes GitHub create the folders automatically)
  - Paste in the full contents of `.github/workflows/build.yml` from this project
  - Commit

---

## Triggering a build

Every time you push a change to the `main` branch, all three platforms build automatically. You can also trigger a build manually without pushing anything:

- Go to the **Actions** tab on your repo
- Click **"Build GreeniPlanner (Windows + macOS + Android)"** in the left sidebar
- Click **Run workflow** → **Run workflow**

### What happens during a build

1. A fast **version-check** step runs first — it confirms `package.json`, `src-tauri/tauri.conf.json`, and `src-tauri/Cargo.toml` all agree on the same version number. If they don't match, the build stops immediately with a clear error instead of wasting time building three platforms with mismatched versions.
2. Once that passes, **Windows**, **macOS**, and **Android** build **in parallel** — you don't wait for one to finish before the next starts.
3. Total time: Windows and macOS typically finish in 8–15 minutes; Android usually takes a bit longer (15–25 minutes) the first time, since it has to download the Android SDK/NDK toolchain. Later builds are faster thanks to caching.

You'll see all three jobs listed with their own green checkmarks (or red ✗ if something failed) once you click into the workflow run.

---

## Downloading your builds

1. Click into the completed workflow run (green checkmark)
2. Scroll down to **Artifacts**
3. You'll see three separate downloads:
   - `GreeniPlanner-Windows-Installer`
   - `GreeniPlanner-macOS-Installer`
   - `GreeniPlanner-Android-APK`
4. Download whichever one(s) you need — each is a `.zip` containing the actual installer file

---

## Installing on Windows

1. Unzip and run the `.exe`
2. Windows may show **"Windows protected your PC"** (Microsoft Defender SmartScreen) — this happens to any new app without a paid code-signing certificate, it does not mean anything is wrong
   - Click **More info** → **Run anyway**
3. The install wizard runs, creates a Desktop shortcut and Start Menu entry
4. GreeniPlanner now shows up in **Add or Remove Programs** with a working uninstaller

**Permanently removing the SmartScreen warning** requires a paid code-signing certificate (~$200–700/year from providers like DigiCert or Sectigo). Not required to use the app — just a one-time click per machine.

---

## Installing on macOS

1. Unzip and open the `.dmg`
2. Drag **GreeniPlanner** into the **Applications** folder, as prompted
3. The first time you open it, macOS Gatekeeper will likely block it with **"GreeniPlanner can't be opened because it is from an unidentified developer"** — same idea as Windows SmartScreen, expected for any app without a paid Apple Developer certificate
   - **Right-click** (or Control-click) the app in Applications → **Open** → confirm **Open** in the dialog that appears
   - You only need to do this once; after that it opens normally
4. If macOS still refuses, go to **System Settings → Privacy & Security**, scroll down, and click **"Open Anyway"** next to the GreeniPlanner warning

**Permanently removing this warning** requires an Apple Developer Program membership ($99/year) to properly sign and notarize the app. Not required to use the app — just a one-time approval per machine.

---

## Installing on Android

1. Copy the `.apk` file to your Android phone or tablet (via USB cable, Google Drive, email to yourself, etc.)
2. Tap the `.apk` file on your device to begin installing
3. Android will likely block it the first time with **"To protect your device, your phone is set to block installs from unknown sources"**
   - Tap **Settings** on that prompt → enable **"Allow from this source"** for whichever app you used to open the file (Files, Chrome, Drive, etc.)
   - Go back and tap the `.apk` again to install
4. GreeniPlanner now appears in your app drawer like any other app

**Why this warning appears:** this APK is signed with a standard Android "debug" signature rather than a Google Play Store release signature. This is completely normal for apps installed outside the Play Store (called "sideloading") and doesn't affect how the app runs — it's the same standard, secure Android signing mechanism used during development, just not tied to a Play Store publisher account.

---

## Making updates to a future version

Whenever you want to ship a new version:

1. Replace `ui/index.html` with the updated app code
2. Update the version number in **all three** of these files so they match:
   - `package.json` → `"version": "1.1.4"` (for example)
   - `src-tauri/tauri.conf.json` → `"version": "1.1.4"`
   - `src-tauri/Cargo.toml` → `version = "1.1.4"`
3. Push the changes to GitHub

The version-check step will automatically catch it if you forget one of these three files — the build will fail immediately with a clear message telling you which files disagree, rather than silently shipping mismatched builds.

**Your users' data is always safe across updates** — installing a new version over an old one on any platform preserves everything already stored, since data lives in the operating system's local storage tied to the app, completely separate from the app's own code.

---

## Optional: upgrading Android to a Play Store–style release signature

The Android build in this project produces a **debug-signed** APK — the most reliable, zero-configuration path to a working, installable app (see the workflow comments for why). If you later want a proper release-signed APK (e.g. for Google Play Store submission), this requires:

1. Generating a permanent signing key with Java's `keytool` (a one-time step — **losing this key means you can never update the app on the Play Store again**, so back it up carefully)
2. Storing that key securely as GitHub Actions secrets
3. Modifying the Android build configuration to reference it

⚠️ **Honest heads-up:** this step has a real, documented history of being fragile even when following Tauri's own official guide exactly — the generated Android project file that needs editing gets rebuilt fresh on every single CI run, so the signing configuration has to be re-applied via script every time rather than just edited once and saved. If you want to pursue this, it's a good idea to test it locally first (with Android Studio installed) before relying on it in CI, and be prepared to troubleshoot the exact generated file structure, which can vary between Tauri versions.

For personal use, testing, or sharing directly with people (not through the Play Store), the debug APK this project already produces is a completely legitimate, standard way to distribute an Android app.

---

## Troubleshooting

**A red ✗ appears on one of the three jobs in the Actions tab**
Click into the failed job to see the error log. Common causes:
- `check-versions` failed → one of the three version numbers doesn't match; see "Making updates" above
- `build-android` failed → Android's toolchain setup is the most complex of the three; click into the failed step to see exactly where it broke — SDK/NDK download hiccups from Google's servers are the most common cause and usually resolve themselves on a retry
- Any job stuck on `npm install` → almost always a transient network hiccup on GitHub's runners

To retry any failed job: go to the workflow run → **Re-run failed jobs**.

**The `.github` folder didn't upload / no Actions tab visible**
See the "Upload the project files" section above — this is almost always the dot-folder drag-and-drop issue.

---

## File reference

```
ui/index.html                     — the GreeniPlanner app itself (HTML/CSS/JS)
icon-source.png                   — master icon (1254×1254) used to generate all platform icons
src-tauri/tauri.conf.json         — shared app config (name, version, window size, bundle targets)
src-tauri/tauri.macos.conf.json   — macOS-specific overrides (min OS version, hardened runtime)
src-tauri/tauri.android.conf.json — Android-specific overrides (min SDK version)
src-tauri/Cargo.toml              — Rust dependencies + release build optimizations
src-tauri/src/main.rs             — desktop entry point
src-tauri/src/lib.rs              — shared entry point (desktop + mobile)
src-tauri/icons/                  — generated icon sets for Windows, macOS, and Android
.github/workflows/build.yml       — the CI pipeline that builds all three platforms
package.json                      — Node/Tauri CLI wrapper
```
