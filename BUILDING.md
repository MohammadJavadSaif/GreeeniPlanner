GreeniPlanner — Building the Windows Installer (.exe)
========================================================

This folder is a ready-to-go Electron project. You don't need a Windows
PC or to install any build tools yourself — GitHub will build the
installer for you, for free, using GitHub Actions.


WHAT YOU'LL END UP WITH
-------------------------
A file called "GreeniPlanner-Setup-1.0.0.exe" — a real Windows installer
that:
  - Lets the user choose the install folder
  - Creates a Desktop shortcut and Start Menu entry
  - Adds "GreeniPlanner" to Windows' "Add or Remove Programs" list,
    with a proper uninstaller


STEP-BY-STEP
-------------

1. CREATE A FREE GITHUB ACCOUNT
   If you don't have one already: https://github.com/signup

2. CREATE A NEW REPOSITORY
   - Go to https://github.com/new
   - Name it: greeniplanner (or anything you like)
   - Set it to "Public" (Actions minutes are free for public repos)
   - Don't initialize with a README (we already have one)
   - Click "Create repository"

3. UPLOAD THIS FOLDER TO THE REPO
   Easiest way (no command line needed):
   - On your new repo's page, click "uploading an existing file"
   - Drag and drop ALL the contents of this folder
     (main.js, package.json, app/, build/, .github/, .gitignore, README.txt)
   - IMPORTANT: make sure the folder structure is preserved —
     .github/workflows/build.yml must stay at that exact nested path,
     and app/index.html must stay inside an "app" folder.
     If drag-and-drop flattens folders, use the command-line method below
     instead.
   - Scroll down, click "Commit changes"

   ALTERNATIVE — command line method (more reliable for folder structure):
     git init
     git add .
     git commit -m "Initial GreeniPlanner build"
     git branch -M main
     git remote add origin https://github.com/YOUR-USERNAME/greeniplanner.git
     git push -u origin main

4. WATCH IT BUILD
   - Go to the "Actions" tab on your GitHub repo
   - You'll see a workflow run called "Build Windows Installer" start
     automatically (takes about 3-5 minutes)
   - Wait for the green checkmark ✓

5. DOWNLOAD YOUR .EXE
   - Click on the completed workflow run
   - Scroll down to "Artifacts"
   - Click "GreeniPlanner-Windows-Installer" to download a zip
   - Inside is "GreeniPlanner-Setup-1.0.0.exe" — that's your installer!

6. INSTALL IT
   - Run the .exe on any Windows 10/11 PC
   - Follow the install wizard
   - GreeniPlanner now appears in your Start Menu and Desktop like any
     normal app, with a full uninstaller


MAKING FUTURE UPDATES
-----------------------
Whenever you want to update the app (new features, bug fixes, etc.):
  1. Replace app/index.html with the new version
  2. Push the change to GitHub (upload again, or `git push`)
  3. GitHub Actions automatically rebuilds — download the new .exe from
     the Actions tab the same way


TROUBLESHOOTING
-----------------
"Actions tab shows a red X / failed build"
  → Click into the failed run to see the error log. Most common cause is
    folder structure getting flattened during upload — re-check that
    .github/workflows/build.yml exists at that exact path.

"I don't see an Actions tab"
  → Make sure the repo is Public, or that Actions are enabled under
    Settings → Actions → General for private repos.

"npm install fails in the build log"
  → This usually resolves itself on a retry. Go to Actions →
    select the failed run → "Re-run all jobs".


FILES IN THIS PROJECT
------------------------
  main.js                       — Electron's entry point (opens the app
                                   window, no browser chrome)
  package.json                  — project config + electron-builder
                                   settings (this defines the installer)
  app/index.html                — your actual GreeniPlanner app
  build/icon.ico                — the app icon (used for the .exe,
                                   shortcuts, and Start Menu)
  .github/workflows/build.yml   — tells GitHub how to build the installer
  .gitignore                    — keeps build junk out of your repo
