# Phase 1: Proof of Concept Setup
**Bento Desktop - Wails Implementation**

**Duration:** 1-2 weeks
**Goal:** Validate Wails architecture with minimal working prototype

---

## Objectives

1. Set up Wails project with React + TypeScript
2. Establish Go ↔ React communication
3. Build minimal workflow runner UI
4. Validate architecture and developer experience
5. Confirm binary size and performance targets

---

## Prerequisites

### System Requirements

```bash
# Check Wails requirements
wails doctor

# Expected output:
# ✓ Go version (1.21+)
# ✓ Node.js version (16+)
# ✓ npm version
# ✓ Platform-specific dependencies (WebView2, etc.)
```

### Install Wails CLI

```bash
# Install Wails v2
go install github.com/wailsapp/wails/v2/cmd/wails@latest

# Verify installation
wails version
```

---

## Task Breakdown

### Task 1: Initialize Wails Project

**Objective:** Create new Wails project with React + TypeScript template

**Steps:**

1. Create desktop app directory
```bash
cd /Users/Ryan/Code/bento/cmd
wails init -n bento-desktop -t react-ts
```

2. Verify project structure
```bash
cd bento-desktop
tree -L 2
# Expected:
# ├── main.go
# ├── app.go
# ├── build/
# ├── frontend/
# │   ├── src/
# │   ├── package.json
# │   └── vite.config.ts
# └── wails.json
```

3. Test initial build
```bash
# Development mode
wails dev

# Should open window with default Wails template
```

**Acceptance Criteria:**
- [ ] Project initializes without errors
- [ ] Development mode launches successfully
- [ ] Window displays React app
- [ ] Hot reload works (edit frontend/src/App.tsx)

**Files Created:**
- `cmd/bento-desktop/main.go`
- `cmd/bento-desktop/app.go`
- `cmd/bento-desktop/frontend/src/App.tsx`
- `cmd/bento-desktop/wails.json`

---

### Task 2: Integrate Existing Bento Packages

**Objective:** Import and expose Bento packages to Wails app

**Steps:**

1. Update `go.mod` in bento-desktop
```bash
cd cmd/bento-desktop

# If go.mod doesn't exist
go mod init github.com/yourusername/bento/cmd/bento-desktop

# Add replace directive to use local Bento packages
go mod edit -replace github.com/yourusername/bento=../..
```

2. Import Bento packages in `app.go`
```go
package main

import (
    "context"
    "github.com/yourusername/bento/pkg/neta"
    "github.com/yourusername/bento/pkg/pantry"
    "github.com/yourusername/bento/pkg/hangiri"
)

type App struct {
    ctx context.Context
}

func NewApp() *App {
    return &App{}
}

func (a *App) startup(ctx context.Context) {
    a.ctx = ctx
}

// Expose Bento methods
func (a *App) GetNodeTypes() []string {
    return pantry.ListTypes()
}

func (a *App) LoadWorkflow(path string) (*neta.Definition, error) {
    return hangiri.Load(path)
}

func (a *App) ValidateWorkflow(path string) (bool, []string) {
    def, err := hangiri.Load(path)
    if err != nil {
        return false, []string{err.Error()}
    }

    errors := omakase.Validate(def)
    return len(errors) == 0, errors
}
```

3. Update `main.go` to use App
```go
package main

import (
    "embed"
    "github.com/wailsapp/wails/v2"
    "github.com/wailsapp/wails/v2/pkg/options"
    "github.com/wailsapp/wails/v2/pkg/options/assetserver"
)

//go:embed all:frontend/dist
var assets embed.FS

func main() {
    app := NewApp()

    err := wails.Run(&options.App{
        Title:  "Bento Desktop",
        Width:  1024,
        Height: 768,
        AssetServer: &assetserver.Options{
            Assets: assets,
        },
        OnStartup: app.startup,
        Bind: []interface{}{
            app,
        },
    })

    if err != nil {
        println("Error:", err.Error())
    }
}
```

4. Generate TypeScript bindings
```bash
# Wails automatically generates bindings during dev
wails dev

# Bindings will be in: frontend/wailsjs/go/main/App.ts
```

**Acceptance Criteria:**
- [ ] Bento packages import without errors
- [ ] TypeScript bindings auto-generate
- [ ] Can call `GetNodeTypes()` from React
- [ ] Can call `LoadWorkflow()` from React
- [ ] No circular dependency issues

**Files Modified:**
- `cmd/bento-desktop/go.mod`
- `cmd/bento-desktop/app.go`
- `cmd/bento-desktop/main.go`

**Files Generated:**
- `cmd/bento-desktop/frontend/wailsjs/go/main/App.ts`
- `cmd/bento-desktop/frontend/wailsjs/go/models.ts`

---

### Task 3: Build Minimal Workflow Runner UI

**Objective:** Create simple React UI to load and display workflow

**Steps:**

1. Create WorkflowViewer component
```typescript
// frontend/src/components/WorkflowViewer.tsx
import { useState } from 'react'
import { LoadWorkflow, GetNodeTypes } from '../../wailsjs/go/main/App'
import { neta } from '../../wailsjs/go/models'

export function WorkflowViewer() {
    const [workflow, setWorkflow] = useState<neta.Definition | null>(null)
    const [error, setError] = useState<string | null>(null)
    const [nodeTypes, setNodeTypes] = useState<string[]>([])

    const loadWorkflow = async () => {
        try {
            // For POC, use a hardcoded path
            const testPath = '/Users/Ryan/Code/bento/examples/hello-world-http.bento.json'
            const def = await LoadWorkflow(testPath)
            setWorkflow(def)
            setError(null)
        } catch (err) {
            setError(err as string)
        }
    }

    const loadNodeTypes = async () => {
        const types = await GetNodeTypes()
        setNodeTypes(types)
    }

    return (
        <div style={{ padding: '20px' }}>
            <h1>Bento Desktop - POC</h1>

            <div>
                <button onClick={loadWorkflow}>Load Test Workflow</button>
                <button onClick={loadNodeTypes}>List Node Types</button>
            </div>

            {error && (
                <div style={{ color: 'red', marginTop: '20px' }}>
                    Error: {error}
                </div>
            )}

            {workflow && (
                <div style={{ marginTop: '20px' }}>
                    <h2>Workflow Loaded</h2>
                    <pre>{JSON.stringify(workflow, null, 2)}</pre>
                </div>
            )}

            {nodeTypes.length > 0 && (
                <div style={{ marginTop: '20px' }}>
                    <h2>Available Node Types</h2>
                    <ul>
                        {nodeTypes.map(type => <li key={type}>{type}</li>)}
                    </ul>
                </div>
            )}
        </div>
    )
}
```

2. Update App.tsx to use WorkflowViewer
```typescript
// frontend/src/App.tsx
import { WorkflowViewer } from './components/WorkflowViewer'
import './App.css'

function App() {
    return (
        <div className="App">
            <WorkflowViewer />
        </div>
    )
}

export default App
```

3. Test the UI
```bash
wails dev

# In the app window:
# 1. Click "Load Test Workflow"
# 2. Verify workflow JSON displays
# 3. Click "List Node Types"
# 4. Verify node types list displays
```

**Acceptance Criteria:**
- [ ] Can load workflow from file path
- [ ] Workflow JSON displays correctly
- [ ] Can fetch node types from Bento
- [ ] Error handling works (try invalid path)
- [ ] UI is responsive and functional

**Files Created:**
- `cmd/bento-desktop/frontend/src/components/WorkflowViewer.tsx`

**Files Modified:**
- `cmd/bento-desktop/frontend/src/App.tsx`

---

### Task 4: Add File Picker (Native Dialog)

**Objective:** Use Wails runtime to open native file picker

**Steps:**

1. Add file picker method to App
```go
// cmd/bento-desktop/app.go
import (
    "github.com/wailsapp/wails/v2/pkg/runtime"
)

func (a *App) OpenWorkflowDialog() (string, error) {
    filePath, err := runtime.OpenFileDialog(a.ctx, runtime.OpenDialogOptions{
        Title: "Select Bento Workflow",
        Filters: []runtime.FileFilter{
            {
                DisplayName: "Bento Workflows (*.bento.json)",
                Pattern:     "*.bento.json",
            },
            {
                DisplayName: "All Files (*.*)",
                Pattern:     "*.*",
            },
        },
    })

    return filePath, err
}
```

2. Update WorkflowViewer to use dialog
```typescript
// frontend/src/components/WorkflowViewer.tsx
import { OpenWorkflowDialog } from '../../wailsjs/go/main/App'

const loadWorkflow = async () => {
    try {
        // Open native file picker
        const filePath = await OpenWorkflowDialog()
        if (!filePath) return  // User cancelled

        const def = await LoadWorkflow(filePath)
        setWorkflow(def)
        setError(null)
    } catch (err) {
        setError(err as string)
    }
}
```

3. Test file picker
```bash
wails dev

# Click "Load Test Workflow"
# Native file picker should open
# Select a .bento.json file
# Workflow should load and display
```

**Acceptance Criteria:**
- [ ] Native file picker opens
- [ ] Can filter by .bento.json files
- [ ] Selected file loads correctly
- [ ] Cancel button works (doesn't error)

**Files Modified:**
- `cmd/bento-desktop/app.go`
- `cmd/bento-desktop/frontend/src/components/WorkflowViewer.tsx`

---

### Task 5: Production Build & Binary Size Check

**Objective:** Build production binary and validate size

**Steps:**

1. Build production binary
```bash
cd cmd/bento-desktop
wails build

# Binary location: build/bin/
```

2. Check binary size
```bash
# macOS
ls -lh build/bin/bento-desktop.app/Contents/MacOS/bento-desktop

# Linux
ls -lh build/bin/bento-desktop

# Windows
dir build\bin\bento-desktop.exe
```

3. Test production binary
```bash
# Run the binary
./build/bin/bento-desktop.app/Contents/MacOS/bento-desktop  # macOS
./build/bin/bento-desktop  # Linux
build\bin\bento-desktop.exe  # Windows
```

4. Measure startup time
```bash
# Use time command
time ./build/bin/bento-desktop.app/Contents/MacOS/bento-desktop

# Expected: < 100ms
```

**Acceptance Criteria:**
- [ ] Production build succeeds
- [ ] Binary size < 20MB (goal: 10-15MB)
- [ ] Application launches in production mode
- [ ] Startup time < 100ms
- [ ] All features work in production build

---

### Task 6: Document Architecture & Decisions

**Objective:** Capture POC findings and architecture decisions

**Steps:**

1. Create architecture document
```markdown
# cmd/bento-desktop/ARCHITECTURE.md

## Project Structure
[Document the structure]

## Go ↔ React Communication
[Document the patterns used]

## Dependencies
[List key dependencies]

## Build Process
[Document build steps]

## Known Issues
[Document any issues found]

## Performance Metrics
- Binary size: X MB
- Startup time: X ms
- Memory usage: X MB
```

2. Update main README
```markdown
# Update /Users/Ryan/Code/bento/README.md

## Desktop Application

Bento Desktop is a React-based UI for visual workflow authoring.

### Quick Start
\`\`\`bash
cd cmd/bento-desktop
wails dev
\`\`\`

See [cmd/bento-desktop/ARCHITECTURE.md](./cmd/bento-desktop/ARCHITECTURE.md) for details.
```

**Acceptance Criteria:**
- [ ] Architecture is documented
- [ ] Build process is documented
- [ ] Performance metrics are recorded
- [ ] Main README is updated

**Files Created:**
- `cmd/bento-desktop/ARCHITECTURE.md`

**Files Modified:**
- `README.md`

---

## Deliverables

### Code Deliverables

- [ ] `cmd/bento-desktop/` - Complete Wails project
- [ ] `cmd/bento-desktop/app.go` - App struct with Bento integration
- [ ] `cmd/bento-desktop/main.go` - Wails entry point
- [ ] `cmd/bento-desktop/frontend/src/components/WorkflowViewer.tsx` - Minimal UI
- [ ] `cmd/bento-desktop/ARCHITECTURE.md` - Architecture documentation

### Validation Deliverables

- [ ] Production binary builds successfully
- [ ] Binary size report (< 20MB target)
- [ ] Startup time measurement (< 100ms target)
- [ ] Screenshot of working UI
- [ ] List of discovered issues/limitations

---

## Success Criteria

### Functional Requirements
- [x] Wails project builds in dev mode
- [x] Wails project builds in production mode
- [x] Can load workflow from file picker
- [x] Can display workflow JSON
- [x] Can list available node types
- [x] TypeScript bindings auto-generate correctly

### Non-Functional Requirements
- [x] Binary size < 20MB
- [x] Startup time < 100ms
- [x] Memory usage < 50MB baseline
- [x] Hot reload works in dev mode
- [x] Cross-platform (test on 2+ platforms if possible)

### Bento Box Principle Compliance
- [x] Desktop code isolated in `cmd/bento-desktop/`
- [x] No modifications to existing `pkg/` packages
- [x] Clear separation between UI and core logic
- [x] Single responsibility per file/component
- [x] No utility grab bags

---

## Testing Checklist

### Manual Testing

- [ ] Load workflow from file picker
- [ ] Display workflow JSON
- [ ] List node types
- [ ] Try loading invalid file (error handling)
- [ ] Cancel file picker (should not error)
- [ ] Hot reload (edit App.tsx, save, verify reload)
- [ ] Production build launches
- [ ] Production build works same as dev

### Performance Testing

- [ ] Measure binary size
- [ ] Measure startup time (3 runs, average)
- [ ] Measure memory usage (Activity Monitor / Task Manager)
- [ ] Test on macOS (if available)
- [ ] Test on Windows (if available)
- [ ] Test on Linux (if available)

---

## Known Limitations (Expected in POC)

1. **Hardcoded paths**: File paths may be hardcoded for testing
2. **No workflow execution**: Only loads and displays, doesn't run
3. **Minimal UI**: Very basic styling, no polish
4. **No error recovery**: May crash on unexpected errors
5. **No state persistence**: Settings don't persist across launches

These are acceptable for Phase 1 and will be addressed in later phases.

---

## Troubleshooting

### Issue: `wails dev` fails with "command not found"

**Solution:**
```bash
# Ensure $GOPATH/bin is in PATH
export PATH=$PATH:$(go env GOPATH)/bin

# Or reinstall Wails
go install github.com/wailsapp/wails/v2/cmd/wails@latest
```

### Issue: TypeScript bindings not generating

**Solution:**
```bash
# Force regeneration
wails dev -clean

# Or manually generate
wails generate module
```

### Issue: Binary size > 20MB

**Solution:**
```bash
# Build with optimizations
wails build -ldflags="-w -s"

# Check what's contributing to size
go tool nm -size build/bin/bento-desktop | sort -n
```

### Issue: "Cannot find package" errors

**Solution:**
```bash
# Check go.mod replace directive
cat go.mod | grep replace

# Should have:
# replace github.com/yourusername/bento => ../..

# If missing, add it:
go mod edit -replace github.com/yourusername/bento=../..
```

---

## Next Steps

After completing Phase 1 POC:

1. **Evaluate Results**
   - Review performance metrics
   - Assess developer experience
   - Identify any blockers

2. **Decision Point**
   - ✅ Proceed to Phase 2 (Core Features)
   - ⚠️ Iterate on POC if issues found
   - ❌ Pivot to alternative if Wails doesn't meet needs

3. **If Proceeding**
   - Review [wails-phase2-core-features.md](./wails-phase2-core-features.md)
   - Allocate 3-4 weeks for Phase 2
   - Plan feature priorities

---

## Colossus Review Prompt

```
I've completed Phase 1 (POC Setup) for Bento Desktop using Wails.

Before marking this phase complete, please:

1. Review all files in cmd/bento-desktop/ against the Bento Box Principle (.claude/BENTO_BOX_PRINCIPLE.md)
2. Verify no modifications were made to existing pkg/ packages
3. Check that Go code follows Go Standards Review (.claude/GO_STANDARDS_REVIEW.md)
4. Verify TypeScript/React code follows best practices
5. Confirm all Task Acceptance Criteria are met (listed in wails-phase1-poc.md)
6. Run the code-review command: /code-review cmd/bento-desktop/

Key areas to scrutinize:
- Is desktop code properly isolated from core packages?
- Are there any utility grab bags (utils/ or helpers/)?
- Is each file/component < 250 lines?
- Does app.go maintain clear boundaries?
- Are Bento packages imported but not modified?

After review, provide:
- List of issues found (if any)
- Recommendations for fixes
- Approval to proceed to Phase 2 OR items to address first

Do not approve this phase until the code-review command has been run and all critical issues are resolved.
```

---

**Phase 1 Status:** Ready to implement
**Next Phase:** [Phase 2: Core Features](./wails-phase2-core-features.md)
