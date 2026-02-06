# Phase 4: Polish & Distribution
**Bento Desktop - Wails Implementation**

**Duration:** 2 weeks
**Goal:** Production-ready application with distribution packages

**Prerequisites:** Phase 3 Visual Editor completed and approved

---

## Objectives

1. Settings and preferences persistence
2. Application theming (light/dark mode)
3. System tray integration
4. Keyboard shortcuts
5. Cross-platform builds
6. Distribution packages (DMG, MSI, AppImage)
7. Auto-update mechanism (optional)

---

## Task Breakdown

### Task 1: Settings & Preferences

**Objective:** Persistent application settings

**Steps:**

1. Create settings manager
```go
// internal/desktop/settings/manager.go
package settings

import (
    "encoding/json"
    "os"
    "path/filepath"
)

type Settings struct {
    Theme             string   `json:"theme"`              // "light", "dark", "system"
    WorkflowDirs      []string `json:"workflowDirs"`
    RecentFiles       []string `json:"recentFiles"`
    EditorZoom        float64  `json:"editorZoom"`
    LogLevel          string   `json:"logLevel"`
    CheckUpdates      bool     `json:"checkUpdates"`
    StartMinimized    bool     `json:"startMinimized"`
}

type Manager struct {
    settings *Settings
    path     string
}

func NewManager() (*Manager, error) {
    configDir, err := os.UserConfigDir()
    if err != nil {
        return nil, err
    }

    bentoDir := filepath.Join(configDir, "Bento")
    os.MkdirAll(bentoDir, 0755)

    path := filepath.Join(bentoDir, "settings.json")

    mgr := &Manager{
        path: path,
        settings: &Settings{
            Theme:        "system",
            WorkflowDirs: getDefaultWorkflowDirs(),
            LogLevel:     "info",
            CheckUpdates: true,
            EditorZoom:   1.0,
        },
    }

    // Load existing settings
    if err := mgr.Load(); err != nil {
        // Use defaults
    }

    return mgr, nil
}

func (m *Manager) Load() error {
    data, err := os.ReadFile(m.path)
    if err != nil {
        return err
    }

    return json.Unmarshal(data, m.settings)
}

func (m *Manager) Save() error {
    data, err := json.MarshalIndent(m.settings, "", "  ")
    if err != nil {
        return err
    }

    return os.WriteFile(m.path, data, 0644)
}

func (m *Manager) Get() *Settings {
    return m.settings
}

func (m *Manager) Update(updates map[string]interface{}) error {
    // Apply updates to settings
    // ... update logic

    return m.Save()
}

func getDefaultWorkflowDirs() []string {
    home, _ := os.UserHomeDir()
    return []string{
        filepath.Join(home, "Bento", "Workflows"),
        "./examples",
    }
}
```

2. Integrate settings into App
```go
// cmd/bento-desktop/app.go

import "github.com/yourusername/bento/internal/desktop/settings"

type App struct {
    ctx         context.Context
    execMgr     *execution.Manager
    settingsMgr *settings.Manager
}

func NewApp() *App {
    settingsMgr, _ := settings.NewManager()

    return &App{
        execMgr:     execution.NewManager(),
        settingsMgr: settingsMgr,
    }
}

func (a *App) GetSettings() (*settings.Settings, error) {
    return a.settingsMgr.Get(), nil
}

func (a *App) UpdateSettings(updates map[string]interface{}) error {
    return a.settingsMgr.Update(updates)
}

func (a *App) AddRecentFile(path string) error {
    s := a.settingsMgr.Get()
    // Add to recent files (max 10)
    // ...
    return a.settingsMgr.Save()
}
```

3. Create Settings UI
```typescript
// frontend/src/components/Settings/Settings.tsx
import { useState, useEffect } from 'react'
import { GetSettings, UpdateSettings } from '../../../wailsjs/go/main/App'

interface Settings {
    theme: string
    workflowDirs: string[]
    logLevel: string
    checkUpdates: boolean
}

export function Settings({ onClose }: { onClose: () => void }) {
    const [settings, setSettings] = useState<Settings | null>(null)
    const [dirty, setDirty] = useState(false)

    useEffect(() => {
        loadSettings()
    }, [])

    const loadSettings = async () => {
        const s = await GetSettings()
        setSettings(s)
    }

    const saveSettings = async () => {
        if (settings) {
            await UpdateSettings(settings)
            setDirty(false)
        }
    }

    const handleChange = (key: string, value: any) => {
        setSettings(prev => ({ ...prev!, [key]: value }))
        setDirty(true)
    }

    if (!settings) return <div>Loading...</div>

    return (
        <div className="settings-modal">
            <div className="settings-content">
                <h2>Settings</h2>

                <div className="setting-group">
                    <h3>Appearance</h3>
                    <label>
                        Theme
                        <select
                            value={settings.theme}
                            onChange={e => handleChange('theme', e.target.value)}
                        >
                            <option value="light">Light</option>
                            <option value="dark">Dark</option>
                            <option value="system">System</option>
                        </select>
                    </label>
                </div>

                <div className="setting-group">
                    <h3>Workflow Directories</h3>
                    {settings.workflowDirs.map((dir, i) => (
                        <div key={i} className="dir-item">
                            <input type="text" value={dir} readOnly />
                            <button onClick={() => {
                                const newDirs = [...settings.workflowDirs]
                                newDirs.splice(i, 1)
                                handleChange('workflowDirs', newDirs)
                            }}>Remove</button>
                        </div>
                    ))}
                    <button onClick={() => {
                        // TODO: Open directory picker
                    }}>Add Directory</button>
                </div>

                <div className="setting-group">
                    <h3>Advanced</h3>
                    <label>
                        Log Level
                        <select
                            value={settings.logLevel}
                            onChange={e => handleChange('logLevel', e.target.value)}
                        >
                            <option value="debug">Debug</option>
                            <option value="info">Info</option>
                            <option value="warn">Warning</option>
                            <option value="error">Error</option>
                        </select>
                    </label>

                    <label>
                        <input
                            type="checkbox"
                            checked={settings.checkUpdates}
                            onChange={e => handleChange('checkUpdates', e.target.checked)}
                        />
                        Check for updates automatically
                    </label>
                </div>

                <div className="settings-actions">
                    <button onClick={saveSettings} disabled={!dirty}>
                        Save
                    </button>
                    <button onClick={onClose}>Close</button>
                </div>
            </div>
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Settings persist across app restarts
- [ ] Can change theme
- [ ] Can add/remove workflow directories
- [ ] Can change log level
- [ ] Settings saved to user config directory
- [ ] Invalid settings don't crash app

**Files Created:**
- `internal/desktop/settings/manager.go`
- `internal/desktop/settings/manager_test.go`
- `frontend/src/components/Settings/Settings.tsx`
- `frontend/src/components/Settings/Settings.css`

**Files Modified:**
- `cmd/bento-desktop/app.go`

---

### Task 2: Application Theming

**Objective:** Support light/dark mode with system preference

**Steps:**

1. Create theme system
```typescript
// frontend/src/theme/ThemeProvider.tsx
import { createContext, useContext, useEffect, useState } from 'react'

type Theme = 'light' | 'dark' | 'system'

interface ThemeContextType {
    theme: Theme
    actualTheme: 'light' | 'dark'
    setTheme: (theme: Theme) => void
}

const ThemeContext = createContext<ThemeContextType | null>(null)

export function ThemeProvider({ children }: { children: React.ReactNode }) {
    const [theme, setTheme] = useState<Theme>('system')
    const [actualTheme, setActualTheme] = useState<'light' | 'dark'>('light')

    useEffect(() => {
        // Load theme from settings
        loadThemeFromSettings()

        // Listen for system theme changes
        const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)')
        const handleChange = () => updateActualTheme(theme)
        mediaQuery.addEventListener('change', handleChange)

        return () => mediaQuery.removeEventListener('change', handleChange)
    }, [])

    useEffect(() => {
        updateActualTheme(theme)
    }, [theme])

    const loadThemeFromSettings = async () => {
        const settings = await GetSettings()
        setTheme(settings.theme)
    }

    const updateActualTheme = (t: Theme) => {
        let actual: 'light' | 'dark' = 'light'

        if (t === 'system') {
            actual = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
        } else {
            actual = t
        }

        setActualTheme(actual)
        document.documentElement.setAttribute('data-theme', actual)
    }

    return (
        <ThemeContext.Provider value={{ theme, actualTheme, setTheme }}>
            {children}
        </ThemeContext.Provider>
    )
}

export function useTheme() {
    const context = useContext(ThemeContext)
    if (!context) throw new Error('useTheme must be used within ThemeProvider')
    return context
}
```

2. Create theme CSS variables
```css
/* frontend/src/theme/themes.css */
:root[data-theme="light"] {
    --bg-primary: #ffffff;
    --bg-secondary: #f5f5f5;
    --bg-tertiary: #e0e0e0;

    --text-primary: #1a1a1a;
    --text-secondary: #666666;
    --text-tertiary: #999999;

    --border-color: #e0e0e0;
    --shadow: rgba(0, 0, 0, 0.1);

    --accent-primary: #2196f3;
    --accent-hover: #1976d2;

    --success: #4caf50;
    --warning: #ff9800;
    --error: #f44336;
}

:root[data-theme="dark"] {
    --bg-primary: #1a1a1a;
    --bg-secondary: #2a2a2a;
    --bg-tertiary: #3a3a3a;

    --text-primary: #ffffff;
    --text-secondary: #b3b3b3;
    --text-tertiary: #808080;

    --border-color: #3a3a3a;
    --shadow: rgba(0, 0, 0, 0.5);

    --accent-primary: #64b5f6;
    --accent-hover: #42a5f5;

    --success: #66bb6a;
    --warning: #ffa726;
    --error: #ef5350;
}

/* Apply theme colors */
body {
    background-color: var(--bg-primary);
    color: var(--text-primary);
}

button {
    background-color: var(--accent-primary);
    color: var(--text-primary);
}

button:hover {
    background-color: var(--accent-hover);
}
```

3. Wrap app with ThemeProvider
```typescript
// frontend/src/main.tsx
import { ThemeProvider } from './theme/ThemeProvider'
import './theme/themes.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <ThemeProvider>
            <App />
        </ThemeProvider>
    </React.StrictMode>
)
```

**Acceptance Criteria:**
- [ ] Light theme works
- [ ] Dark theme works
- [ ] System theme respects OS preference
- [ ] Theme changes immediately when switched
- [ ] All components support both themes
- [ ] Theme persists across restarts

**Files Created:**
- `frontend/src/theme/ThemeProvider.tsx`
- `frontend/src/theme/themes.css`
- `frontend/src/theme/useTheme.ts`

---

### Task 3: System Tray Integration

**Objective:** Minimize to system tray with context menu

**Steps:**

1. Add system tray to App
```go
// cmd/bento-desktop/app.go

import "github.com/wailsapp/wails/v2/pkg/menu"

func (a *App) createSystemTray() *menu.Menu {
    return menu.NewMenu()
}

// In main.go
func main() {
    app := NewApp()

    err := wails.Run(&options.App{
        // ... existing options
        Menu: app.createAppMenu(),
        OnStartup: app.startup,
        OnShutdown: app.shutdown,
        Bind: []interface{}{
            app,
        },
        Windows: &windows.Options{
            SystemTrayIcon: embedSystemTrayIcon(),
        },
        Mac: &mac.Options{
            SystemTrayIcon: embedSystemTrayIcon(),
        },
    })
}

func embedSystemTrayIcon() []byte {
    // Embed tray icon
    //go:embed assets/tray-icon.png
    var trayIcon []byte
    return trayIcon
}
```

2. Add tray menu items
```go
// cmd/bento-desktop/app.go

func (a *App) createSystemTray() *menu.Menu {
    return menu.NewMenu().
        AddText("Bento Desktop", nil).
        AddSeparator().
        AddText("Show", func(_ *menu.CallbackData) {
            runtime.WindowShow(a.ctx)
        }).
        AddText("Hide", func(_ *menu.CallbackData) {
            runtime.WindowHide(a.ctx)
        }).
        AddSeparator().
        AddText("Quit", func(_ *menu.CallbackData) {
            runtime.Quit(a.ctx)
        })
}
```

**Acceptance Criteria:**
- [ ] System tray icon appears
- [ ] Right-click shows menu
- [ ] "Show" brings window to front
- [ ] "Hide" minimizes to tray
- [ ] "Quit" exits application
- [ ] Works on all platforms (Windows, macOS, Linux)

**Files Modified:**
- `cmd/bento-desktop/main.go`
- `cmd/bento-desktop/app.go`

**Files Created:**
- `cmd/bento-desktop/assets/tray-icon.png`
- `cmd/bento-desktop/assets/tray-icon@2x.png`

---

### Task 4: Keyboard Shortcuts

**Objective:** Add keyboard shortcuts for common actions

**Steps:**

1. Define keyboard shortcuts
```typescript
// frontend/src/hooks/useKeyboardShortcuts.ts
import { useEffect } from 'react'

interface Shortcuts {
    [key: string]: () => void
}

export function useKeyboardShortcuts(shortcuts: Shortcuts) {
    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            const key = getShortcutKey(e)

            if (shortcuts[key]) {
                e.preventDefault()
                shortcuts[key]()
            }
        }

        window.addEventListener('keydown', handleKeyDown)
        return () => window.removeEventListener('keydown', handleKeyDown)
    }, [shortcuts])
}

function getShortcutKey(e: KeyboardEvent): string {
    const parts: string[] = []

    if (e.ctrlKey || e.metaKey) parts.push('Cmd')
    if (e.shiftKey) parts.push('Shift')
    if (e.altKey) parts.push('Alt')

    parts.push(e.key.toUpperCase())

    return parts.join('+')
}
```

2. Add shortcuts to WorkflowEditor
```typescript
// frontend/src/components/WorkflowEditor/WorkflowEditor.tsx
import { useKeyboardShortcuts } from '../../hooks/useKeyboardShortcuts'

export function WorkflowEditor() {
    const [nodes, setNodes] = useState<Node[]>([])
    const [selectedNodes, setSelectedNodes] = useState<Node[]>([])

    useKeyboardShortcuts({
        'Cmd+S': () => saveWorkflow(),
        'Cmd+O': () => openWorkflow(),
        'Cmd+N': () => newWorkflow(),
        'Delete': () => deleteSelectedNodes(),
        'Cmd+Z': () => undo(),
        'Cmd+Shift+Z': () => redo(),
        'Cmd+D': () => duplicateSelectedNodes(),
        'Cmd+A': () => selectAllNodes(),
    })

    // ... rest of component
}
```

3. Create keyboard shortcuts help modal
```typescript
// frontend/src/components/KeyboardShortcuts/ShortcutsModal.tsx
export function ShortcutsModal({ onClose }: { onClose: () => void }) {
    return (
        <div className="shortcuts-modal">
            <h2>Keyboard Shortcuts</h2>

            <div className="shortcuts-section">
                <h3>File</h3>
                <div className="shortcut">
                    <kbd>Cmd+S</kbd>
                    <span>Save workflow</span>
                </div>
                <div className="shortcut">
                    <kbd>Cmd+O</kbd>
                    <span>Open workflow</span>
                </div>
                <div className="shortcut">
                    <kbd>Cmd+N</kbd>
                    <span>New workflow</span>
                </div>
            </div>

            <div className="shortcuts-section">
                <h3>Edit</h3>
                <div className="shortcut">
                    <kbd>Cmd+Z</kbd>
                    <span>Undo</span>
                </div>
                <div className="shortcut">
                    <kbd>Cmd+Shift+Z</kbd>
                    <span>Redo</span>
                </div>
                <div className="shortcut">
                    <kbd>Delete</kbd>
                    <span>Delete selected</span>
                </div>
            </div>

            <button onClick={onClose}>Close</button>
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Cmd+S saves workflow
- [ ] Cmd+O opens file picker
- [ ] Cmd+N creates new workflow
- [ ] Delete removes selected nodes
- [ ] Cmd+Z/Cmd+Shift+Z undo/redo
- [ ] Shortcuts work on all platforms (Cmd on Mac, Ctrl on Windows/Linux)
- [ ] Help modal shows all shortcuts

**Files Created:**
- `frontend/src/hooks/useKeyboardShortcuts.ts`
- `frontend/src/components/KeyboardShortcuts/ShortcutsModal.tsx`

---

### Task 5: Cross-Platform Builds

**Objective:** Build binaries for all platforms

**Steps:**

1. Create build scripts
```bash
# scripts/build-all.sh
#!/bin/bash

echo "Building Bento Desktop for all platforms..."

# macOS (Universal)
echo "Building for macOS..."
cd cmd/bento-desktop
wails build -platform darwin/universal -clean

# Windows (AMD64)
echo "Building for Windows..."
wails build -platform windows/amd64

# Linux (AMD64)
echo "Building for Linux..."
wails build -platform linux/amd64

echo "Builds complete!"
echo "Binaries located in cmd/bento-desktop/build/bin/"
```

2. Add Makefile targets
```makefile
# Makefile
.PHONY: build-desktop-mac build-desktop-windows build-desktop-linux build-desktop-all

build-desktop-mac:
	cd cmd/bento-desktop && wails build -platform darwin/universal

build-desktop-windows:
	cd cmd/bento-desktop && wails build -platform windows/amd64

build-desktop-linux:
	cd cmd/bento-desktop && wails build -platform linux/amd64

build-desktop-all: build-desktop-mac build-desktop-windows build-desktop-linux
	@echo "All desktop builds complete"
```

3. Configure wails.json for each platform
```json
// cmd/bento-desktop/wails.json
{
  "name": "bento-desktop",
  "outputfilename": "Bento Desktop",
  "frontend:install": "npm install",
  "frontend:build": "npm run build",
  "author": {
    "name": "Your Name",
    "email": "you@example.com"
  },
  "info": {
    "companyName": "Bento",
    "productName": "Bento Desktop",
    "productVersion": "1.0.0",
    "copyright": "Copyright 2025",
    "comments": "Visual workflow editor for Bento"
  },
  "nsisType": "multiple",
  "obfuscated": false,
  "wailsjsdir": "./frontend"
}
```

**Acceptance Criteria:**
- [ ] macOS build works (Universal binary)
- [ ] Windows build works (AMD64)
- [ ] Linux build works (AMD64)
- [ ] All binaries under 30MB
- [ ] Build script completes without errors
- [ ] Makefile targets work

**Files Created:**
- `scripts/build-all.sh`
- `scripts/build-mac.sh`
- `scripts/build-windows.sh`
- `scripts/build-linux.sh`

**Files Modified:**
- `Makefile`
- `cmd/bento-desktop/wails.json`

---

### Task 6: Distribution Packages

**Objective:** Create installer packages for each platform

**Steps:**

1. macOS: Create DMG
```bash
# scripts/package-mac.sh
#!/bin/bash

APP_NAME="Bento Desktop"
VERSION="1.0.0"
DMG_NAME="BentoDesktop-${VERSION}.dmg"

cd cmd/bento-desktop/build/bin

# Create temporary DMG directory
mkdir -p dmg
cp -r "${APP_NAME}.app" dmg/

# Create Applications symlink
ln -s /Applications dmg/Applications

# Create DMG
hdiutil create -volname "${APP_NAME}" -srcfolder dmg -ov -format UDZO "${DMG_NAME}"

# Cleanup
rm -rf dmg

echo "DMG created: ${DMG_NAME}"
```

2. Windows: Configure NSIS installer (via Wails)
```json
// cmd/bento-desktop/wails.json
{
  "info": {
    "productName": "Bento Desktop",
    "productVersion": "1.0.0",
    "copyright": "Copyright 2025",
    "comments": "Visual workflow editor for Bento"
  },
  "nsisType": "multiple"
}
```

3. Linux: Create AppImage
```bash
# scripts/package-linux.sh
#!/bin/bash

APP_NAME="BentoDesktop"
VERSION="1.0.0"

cd cmd/bento-desktop/build/bin

# Create AppDir structure
mkdir -p AppDir/usr/bin
mkdir -p AppDir/usr/share/applications
mkdir -p AppDir/usr/share/icons/hicolor/256x256/apps

# Copy binary
cp bento-desktop AppDir/usr/bin/

# Create .desktop file
cat > AppDir/usr/share/applications/bento-desktop.desktop << EOF
[Desktop Entry]
Name=Bento Desktop
Exec=bento-desktop
Icon=bento-desktop
Type=Application
Categories=Development;
EOF

# Copy icon
cp ../../assets/icon.png AppDir/usr/share/icons/hicolor/256x256/apps/bento-desktop.png

# Create AppImage (requires appimagetool)
appimagetool AppDir "${APP_NAME}-${VERSION}-x86_64.AppImage"

echo "AppImage created: ${APP_NAME}-${VERSION}-x86_64.AppImage"
```

**Acceptance Criteria:**
- [ ] macOS DMG installs correctly
- [ ] Windows MSI/EXE installs correctly
- [ ] Linux AppImage runs correctly
- [ ] All installers include app icon
- [ ] All installers create desktop shortcuts
- [ ] Uninstallers work correctly

**Files Created:**
- `scripts/package-mac.sh`
- `scripts/package-windows.sh`
- `scripts/package-linux.sh`
- `scripts/package-all.sh`

---

### Task 7: Auto-Update (Optional)

**Objective:** Automatic update checking and installation

**Steps:**

1. Add update checking
```go
// internal/desktop/updates/checker.go
package updates

import (
    "encoding/json"
    "net/http"
    "time"
)

type Release struct {
    Version string `json:"version"`
    URL     string `json:"url"`
    Notes   string `json:"notes"`
}

type Checker struct {
    currentVersion string
    checkURL       string
}

func NewChecker(version string) *Checker {
    return &Checker{
        currentVersion: version,
        checkURL:       "https://releases.bento.example.com/latest.json",
    }
}

func (c *Checker) CheckForUpdates() (*Release, error) {
    resp, err := http.Get(c.checkURL)
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()

    var release Release
    if err := json.NewDecoder(resp.Body).Decode(&release); err != nil {
        return nil, err
    }

    if release.Version > c.currentVersion {
        return &release, nil
    }

    return nil, nil // No update available
}
```

2. Add update notification
```go
// cmd/bento-desktop/app.go

func (a *App) CheckForUpdates() (*updates.Release, error) {
    checker := updates.NewChecker("1.0.0") // From build
    return checker.CheckForUpdates()
}
```

3. Show update notification in UI
```typescript
// frontend/src/components/UpdateNotification/UpdateNotification.tsx
export function UpdateNotification() {
    const [update, setUpdate] = useState<any>(null)

    useEffect(() => {
        checkForUpdates()
    }, [])

    const checkForUpdates = async () => {
        const release = await CheckForUpdates()
        if (release) {
            setUpdate(release)
        }
    }

    if (!update) return null

    return (
        <div className="update-notification">
            <div className="update-content">
                <h3>Update Available</h3>
                <p>Version {update.version} is available</p>
                <p>{update.notes}</p>
                <div className="update-actions">
                    <button onClick={() => window.open(update.url)}>
                        Download
                    </button>
                    <button onClick={() => setUpdate(null)}>
                        Later
                    </button>
                </div>
            </div>
        </div>
    )
}
```

**Acceptance Criteria:**
- [ ] Checks for updates on startup (if enabled)
- [ ] Shows notification when update available
- [ ] Opens download page in browser
- [ ] Can dismiss notification
- [ ] Respects "Don't check for updates" setting

**Files Created:**
- `internal/desktop/updates/checker.go`
- `internal/desktop/updates/checker_test.go`
- `frontend/src/components/UpdateNotification/UpdateNotification.tsx`

---

## Deliverables

### Code Deliverables

- [ ] `internal/desktop/settings/` - Settings management
- [ ] `internal/desktop/updates/` - Update checking (optional)
- [ ] `frontend/src/theme/` - Theme system
- [ ] `frontend/src/components/Settings/` - Settings UI
- [ ] `frontend/src/hooks/useKeyboardShortcuts.ts` - Keyboard shortcuts
- [ ] Updated `cmd/bento-desktop/main.go` - System tray integration

### Build & Distribution

- [ ] `scripts/build-all.sh` - Cross-platform build script
- [ ] `scripts/package-all.sh` - Packaging script
- [ ] `Makefile` - Build targets
- [ ] `cmd/bento-desktop/wails.json` - Build configuration

### Documentation

- [ ] `cmd/bento-desktop/INSTALL.md` - Installation instructions
- [ ] `cmd/bento-desktop/BUILDING.md` - Build instructions
- [ ] Updated `README.md` - Download links and getting started

---

## Success Criteria

### Functional Requirements
- [ ] Settings persist correctly
- [ ] Theme switching works
- [ ] System tray integration works
- [ ] Keyboard shortcuts work
- [ ] Cross-platform builds succeed
- [ ] Distribution packages install correctly

### Non-Functional Requirements
- [ ] macOS binary < 20MB
- [ ] Windows binary < 25MB
- [ ] Linux binary < 20MB
- [ ] Startup time < 100ms on all platforms
- [ ] Settings load in < 50ms

### Bento Box Principle Compliance
- [ ] Settings code isolated in internal/desktop/settings
- [ ] No modifications to pkg/ packages
- [ ] Clear separation of concerns

---

## Testing Checklist

### Manual Testing

**Settings:**
- [ ] Change theme (light/dark/system)
- [ ] Add workflow directory
- [ ] Remove workflow directory
- [ ] Change log level
- [ ] Toggle auto-update
- [ ] Restart app, verify settings persist

**Theming:**
- [ ] Switch to light theme
- [ ] Switch to dark theme
- [ ] Switch to system theme
- [ ] Change OS theme, verify app updates

**System Tray:**
- [ ] Minimize to tray
- [ ] Restore from tray
- [ ] Right-click tray icon
- [ ] Quit from tray

**Keyboard Shortcuts:**
- [ ] Test all shortcuts
- [ ] Verify shortcuts don't conflict
- [ ] Open shortcuts help modal

**Cross-Platform:**
- [ ] Test on macOS (Intel and Apple Silicon)
- [ ] Test on Windows 10/11
- [ ] Test on Ubuntu/Debian Linux

### Distribution Testing

**macOS:**
- [ ] Download DMG
- [ ] Open DMG
- [ ] Drag to Applications
- [ ] Launch from Applications
- [ ] Verify code signing (if signed)

**Windows:**
- [ ] Run installer
- [ ] Install to Program Files
- [ ] Launch from Start Menu
- [ ] Verify desktop shortcut
- [ ] Uninstall correctly

**Linux:**
- [ ] Download AppImage
- [ ] Make executable
- [ ] Run AppImage
- [ ] Verify it works

---

## Performance Targets

- [ ] Settings load in < 50ms
- [ ] Theme switch completes in < 100ms
- [ ] Update check completes in < 2 seconds
- [ ] Binary size < 25MB on all platforms

---

## Release Checklist

Before releasing v1.0.0:

- [ ] All Phase 1-4 features complete
- [ ] All tests passing
- [ ] All platforms tested
- [ ] Documentation complete
- [ ] Code reviewed (via /code-review)
- [ ] Version bumped to 1.0.0
- [ ] Changelog updated
- [ ] Release notes written
- [ ] Binaries built for all platforms
- [ ] Installers tested on all platforms
- [ ] GitHub release created
- [ ] Download links work
- [ ] Website updated (if applicable)

---

## Post-Release Tasks

1. **Monitor for Issues**
   - Watch GitHub issues
   - Monitor error reports
   - Gather user feedback

2. **Plan v1.1.0**
   - Prioritize feature requests
   - Address common issues
   - Plan improvements

3. **Documentation**
   - Create video tutorials
   - Write blog posts
   - Update wiki

---

## Colossus Review Prompt

```
I've completed Phase 4 (Polish & Distribution) for Bento Desktop.

This is the FINAL phase before release. Please conduct a thorough review:

1. Review all Phase 4 code against Bento Box Principle (.claude/BENTO_BOX_PRINCIPLE.md):
   - Check internal/desktop/settings/ structure
   - Verify theme system is clean
   - Confirm no utility grab bags

2. Verify Go Standards (.claude/GO_STANDARDS_REVIEW.md):
   - Check settings persistence
   - Verify error handling
   - Check file I/O operations

3. Review build & distribution setup:
   - Verify wails.json configuration
   - Check build scripts
   - Verify package scripts

4. Comprehensive testing:
   - Test on macOS (if available)
   - Test on Windows (if available)
   - Test on Linux (if available)
   - Test all installers
   - Verify all features work

5. Documentation review:
   - Check INSTALL.md completeness
   - Check BUILDING.md accuracy
   - Verify README.md is up to date

6. Run the code-review command: /code-review cmd/bento-desktop/ internal/desktop/

Key areas to scrutinize:
- Is settings persistence robust?
- Does theme system handle edge cases?
- Are build scripts correct?
- Do installers work correctly?
- Is documentation complete?

Release Readiness Checklist:
- [ ] All features from Phases 1-4 complete
- [ ] All platforms build successfully
- [ ] All installers tested
- [ ] Documentation complete
- [ ] No critical bugs
- [ ] Performance targets met

After review, provide:
- List of blocking issues (must fix before release)
- List of non-blocking issues (can defer to v1.1)
- Final approval for release OR items to address

This is the go/no-go decision for v1.0.0 release.

Do not approve until:
1. Code-review command has been run
2. All blocking issues are resolved
3. At least 2 platforms have been tested end-to-end
4. All Phase 4 acceptance criteria are met
```

---

**Phase 4 Status:** Ready to implement
**Previous Phase:** [Phase 3: Workflow Editor](./wails-phase3-workflow-editor.md)
**Next Step:** Release v1.0.0!
