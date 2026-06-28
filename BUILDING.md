GreeniPlanner v1.0.8 — Building the Windows Installer
========================================================

This is a Tauri project. GitHub Actions builds the Windows installer
automatically — you don't need Rust, Node, or a Windows machine locally.

The output is a real, tiny Windows installer:
  - GreeniPlanner-1.0.8-setup.exe  (~4–6MB)
  - Installed footprint: ~12–18MB
  - Adds to "Add or Remove Programs" with a proper uninstaller
  - Desktop + Start Menu shortcuts created on install


WHY SO SMALL? (vs. the old Electron build)
--------------------------------------------
Tauri uses Windows' built-in WebView2 engine (already on every
Windows 10/11 PC since 2021) instead of bundling a full copy of Chrome.
The Rust backend is compiled to a tiny native binary. Result: 10-15x
smaller than Electron for the same app.


STEP-BY-STEP BUILD (via GitHub Actions — no local tools needed)
----------------------------------------------------------------

1. CREATE A FREE GITHUB ACCOUNT
   https://github.com/signup

2. CREATE A NEW REPOSITORY
   - https://github.com/new
   - Name: greeniplanner
   - Set to Public (free Actions build minutes)
   - Do NOT initialize with a README

3. UPLOAD ALL FILES TO THE REPO
   IMPORTANT: The .github folder must be uploaded correctly.
   GitHub's drag-and-drop hides dot-folders. Instead:

   A) Upload everything EXCEPT .github via drag-and-drop
   B) Then create the workflow file manually:
      - Click "Add file" -> "Create new file"
      - Name it exactly: .github/workflows/build.yml
        (type the slashes -- GitHub creates the folders automatically)
      - Paste the contents of .github/workflows/build.yml from this folder
      - Commit

4. WATCH THE BUILD
   - Go to the "Actions" tab on your repo
   - "Build GreeniPlanner Windows Installer" starts automatically
   - Takes 8-15 minutes (Rust compilation is slower than JS)
   - Green checkmark = done

5. DOWNLOAD YOUR INSTALLER
   - Click the completed workflow run
   - Scroll to "Artifacts"
   - Download "GreeniPlanner-Windows-Installer"
   - Inside: GreeniPlanner-1.0.8-setup.exe

6. INSTALL & RUN
   - Run the .exe
   - If Windows SmartScreen appears: click "More info" -> "Run anyway"
   - Install wizard runs, creates shortcuts
   - Done


SMARTSCREEN WARNING
--------------------
Happens to all new unsigned apps. To permanently fix it you need a
code-signing certificate (~$200-700/year from DigiCert, Sectigo, etc).
Without one, "More info" -> "Run anyway" works fine.


DATA MIGRATION
---------------
Tauri stores localStorage in %APPDATA%\com.greeniai.greeniplanner.
Installing over a previous version preserves all user data automatically.
