# Desktop UI Architecture Strategy
**Bnto React Frontend Evaluation**

> **2026-02-06 UPDATE:** Evaluation remains valid. Wails is confirmed as the choice.
> See [CLOUD_DESKTOP_STRATEGY.md](CLOUD_DESKTOP_STRATEGY.md) for how this fits into the full product strategy.
> Target **Wails v2** (stable). Desktop is **Phase 3**.

**Date:** 2025-10-22
**Updated:** 2026-02-06
**Purpose:** Evaluate architectural approaches for adding a React-based desktop UI to the Bnto Go CLI

---

## Executive Summary

This document evaluates multiple architectural approaches for building a desktop application that combines Bnto's existing Go CLI functionality with a React-based user interface. After analyzing five distinct approaches, **Wails** emerges as the recommended solution due to its purpose-built design for Go+React desktop apps, native webview performance, and seamless Go-JavaScript interoperability.

---

## Current State: Bnto CLI Architecture

### Core Components

```
bnto/
├── cmd/bnto/           # CLI commands (run, validate, list, new, etc.)
├── pkg/
│   ├── neta/           # Workflow node types (10 types)
│   ├── itamae/         # Orchestration engine
│   ├── pantry/         # Node registry
│   ├── hangiri/        # Storage layer
│   ├── shoyu/          # Structured logging
│   ├── omakase/        # Validation
│   └── wasabi/         # Additional utilities
```

### Key Characteristics

- **Pure Go codebase**: High performance, small binary (~20MB)
- **CLI-first design**: Commands via Cobra framework
- **Modular architecture**: Follows Bento Box Principle (single responsibility)
- **Cross-platform**: Linux, macOS, Windows support
- **Performance**: 15-30x faster startup than Node.js, 6x less memory

### Existing Roadmap

From `.claude/TODO.md`:
1. Clean up run logs
2. Make command names straightforward
3. **Charm TUI** (terminal UI)
4. **Charm TUI Bnto Editor** (terminal-based editor)

---

## Architectural Options Evaluated

### Option 1: Wails (Go + React via Native WebView)

**Architecture:**
```
┌─────────────────────────────────────┐
│   Desktop Application (Single Binary)│
├─────────────────────────────────────┤
│  React Frontend (Vite + TypeScript)  │
│  ├─ UI Components                    │
│  └─ Calls Go via window.backend      │
├─────────────────────────────────────┤
│  Wails Bridge Layer                  │
│  ├─ Auto-generated TypeScript types  │
│  ├─ Method binding (Go → JS)         │
│  └─ Event system                     │
├─────────────────────────────────────┤
│  Go Backend (Existing Bnto Pkgs)    │
│  ├─ itamae (orchestration)           │
│  ├─ neta (workflow nodes)            │
│  └─ All existing packages            │
├─────────────────────────────────────┤
│  Native WebView (OS-provided)        │
│  ├─ WebView2 (Windows)               │
│  ├─ WebKit (macOS)                   │
│  └─ WebKitGTK (Linux)                │
└─────────────────────────────────────┘
```

**Key Features:**
- Uses OS native webview (not embedded Chromium)
- Auto-generates TypeScript definitions from Go structs
- Direct Go method calls from JavaScript via `window.backend`
- Single binary output with embedded frontend assets
- Hot reload during development
- React templates with Vite pre-configured

**Performance:**
- Binary size: ~10-15MB (no Chromium embedded)
- Memory: Similar to native app (~20-30MB baseline)
- Startup: <50ms (native webview initialization)

**Development Workflow:**
```bash
# Initialize new Wails+React project
wails init -n bnto-desktop -t react-ts

# Development mode (hot reload)
wails dev

# Production build
wails build  # Single binary output
```

**Go-React Communication:**
```go
// Go Backend (app.go)
type App struct {
    ctx context.Context
}

func (a *App) RunWorkflow(path string) string {
    // Call existing Bnto packages
    result := itamae.Execute(path)
    return result
}
```

```typescript
// React Frontend (App.tsx)
import { RunWorkflow } from '../wailsjs/go/main/App'

function WorkflowRunner() {
    const run = async () => {
        const result = await RunWorkflow('/path/to/workflow.bnto.json')
        console.log(result)
    }
}
```

**Pros:**
- ✅ Purpose-built for Go + web frontend
- ✅ Smallest binary size (~10-15MB)
- ✅ Best performance (native webview)
- ✅ Type-safe Go↔JS communication
- ✅ No CGO dependencies on Windows
- ✅ Active community, well-documented
- ✅ Can reuse 100% of existing Bnto Go code
- ✅ Cross-platform (Windows/macOS/Linux)
- ✅ App Store compatible

**Cons:**
- ⚠️ Adds new framework to learn
- ⚠️ Different from Charm TUI approach (potential duplication)
- ⚠️ WebView limitations (no browser DevTools in production)
- ⚠️ Less mature than Electron ecosystem

**Alignment with Bento Box Principle:**
- ✅ Single responsibility: UI layer separate from CLI
- ✅ Clear boundaries: Wails bridge defines interface
- ✅ Composable: Reuses existing Bnto packages
- ⚠️ Small concern: May need wrapper layer for Wails-specific code

---

### Option 2: Electron (Go Backend + React Frontend)

**Architecture:**
```
┌─────────────────────────────────────┐
│   Electron Application               │
├─────────────────────────────────────┤
│  Renderer Process (React)            │
│  └─ IPC calls to Main Process        │
├─────────────────────────────────────┤
│  Main Process (Node.js)              │
│  └─ child_process spawns Go binary   │
├─────────────────────────────────────┤
│  Go Backend (Bnto CLI)              │
│  ├─ HTTP/gRPC server OR              │
│  └─ stdin/stdout communication       │
├─────────────────────────────────────┤
│  Embedded Chromium + Node.js         │
└─────────────────────────────────────┘
```

**Communication Options:**

**A. HTTP/gRPC Server:**
```go
// Go Backend (starts HTTP server)
func main() {
    http.HandleFunc("/run", handleRun)
    http.ListenAndServe(":8080", nil)
}
```

```typescript
// Electron Main Process
const go = spawn('./bnto-server')

// React (via IPC)
ipcRenderer.send('run-workflow', path)
```

**B. stdin/stdout IPC:**
```go
// Go Backend (reads from stdin)
scanner := bufio.NewScanner(os.Stdin)
for scanner.Scan() {
    cmd := scanner.Text()
    // Process command
    json.NewEncoder(os.Stdout).Encode(result)
}
```

**Pros:**
- ✅ Mature ecosystem (vast npm packages)
- ✅ Full Chrome DevTools
- ✅ Well-established patterns
- ✅ Large community support

**Cons:**
- ❌ Huge binary size (~150-200MB)
- ❌ High memory usage (~150MB baseline)
- ❌ Slower startup (~500ms+)
- ❌ Complex IPC layer (Electron ↔ Go)
- ❌ Requires Node.js + Go runtime
- ❌ Goes against Bnto's "small binary" goal

**Alignment with Bento Box Principle:**
- ⚠️ Violates "small, focused" principle (200MB binary)
- ✅ Clear boundaries via IPC
- ⚠️ Not composable with existing CLI (separate processes)

---

### Option 3: Tauri (Rust-based, Go Sidecar)

**Architecture:**
```
┌─────────────────────────────────────┐
│   Tauri Application                  │
├─────────────────────────────────────┤
│  React Frontend                      │
│  └─ Calls Rust commands              │
├─────────────────────────────────────┤
│  Rust Backend (Tauri Core)           │
│  └─ Spawns Go binary as sidecar      │
├─────────────────────────────────────┤
│  Go Backend (Bnto CLI)              │
│  └─ HTTP/gRPC or stdin/stdout        │
├─────────────────────────────────────┤
│  Native WebView (OS-provided)        │
└─────────────────────────────────────┘
```

**Pros:**
- ✅ Small binary (~5-10MB Tauri + ~20MB Go)
- ✅ Native webview performance
- ✅ Excellent security model
- ✅ Fast execution

**Cons:**
- ❌ Requires Rust toolchain (additional language)
- ❌ Go runs as separate sidecar (not integrated)
- ❌ Complex build process (Rust + Go)
- ❌ Indirect communication (Rust → Go)
- ❌ Longer compile times (Rust)
- ❌ Less idiomatic for Go developers

**Alignment with Bento Box Principle:**
- ⚠️ Adds unnecessary layer (Rust)
- ⚠️ Not composable (separate binaries)
- ❌ Violates YAGNI (don't need Rust)

---

### Option 4: Embedded Web Server + Separate React App

**Architecture:**
```
┌─────────────────────────────────────┐
│  Option 4A: Packaged Together       │
├─────────────────────────────────────┤
│  Go Backend (embedded HTTP server)   │
│  ├─ Bnto packages                   │
│  └─ Serves embedded React SPA        │
│     (via embed.FS)                   │
├─────────────────────────────────────┤
│  System Browser or WebView Wrapper  │
└─────────────────────────────────────┘

┌─────────────────────────────────────┐
│  Option 4B: Separate Processes       │
├─────────────────────────────────────┤
│  Go Backend HTTP Server              │
│  └─ REST/gRPC API                    │
├─────────────────────────────────────┤
│  React App (separate process)        │
│  └─ Opens system browser             │
└─────────────────────────────────────┘
```

**Implementation Example (4A - Embedded):**
```go
//go:embed frontend/dist/*
var frontend embed.FS

func main() {
    // API routes
    http.HandleFunc("/api/run", handleRun)

    // Serve React app
    http.Handle("/", http.FileServer(http.FS(frontend)))
    http.ListenAndServe(":8080", nil)
}
```

**Pros:**
- ✅ Simple architecture
- ✅ Clean separation (REST API)
- ✅ Can use CLI and UI independently
- ✅ Familiar HTTP patterns
- ✅ Easy to test
- ✅ Single Go binary (Option 4A)

**Cons:**
- ❌ Not truly a "desktop app" (browser-based)
- ❌ No native window management
- ❌ No system tray integration
- ❌ Requires user to open browser (4B)
- ❌ Less polished UX

**Alignment with Bento Box Principle:**
- ✅ Excellent separation of concerns
- ✅ Clean API boundaries
- ✅ Composable
- ✅ YAGNI compliant

---

### Option 5: Pure Go UI (Fyne or Gio)

**Architecture:**
```
┌─────────────────────────────────────┐
│  Pure Go Desktop Application         │
├─────────────────────────────────────┤
│  Fyne/Gio UI Framework (Go)          │
│  └─ Declarative UI in Go             │
├─────────────────────────────────────┤
│  Bnto Packages (Go)                 │
│  └─ Direct function calls            │
└─────────────────────────────────────┘
```

**Example (Fyne):**
```go
import "fyne.io/fyne/v2/app"

func main() {
    a := app.New()
    w := a.NewWindow("Bnto")

    w.SetContent(widget.NewButton("Run Workflow", func() {
        // Direct calls to Bnto packages
        itamae.Execute(ctx, workflow)
    }))

    w.ShowAndRun()
}
```

**Pros:**
- ✅ Pure Go (no additional languages)
- ✅ Smallest binaries (~10-15MB)
- ✅ Direct integration with Bnto
- ✅ Fastest performance
- ✅ Simple deployment

**Cons:**
- ❌ **No React** (fails primary requirement)
- ❌ Limited UI components
- ❌ Less modern look/feel
- ❌ Smaller ecosystem than web tech
- ❌ Harder to hire UI developers

**Alignment with Bento Box Principle:**
- ✅ Excellent alignment (pure Go)
- ✅ Single language ecosystem
- ✅ YAGNI compliant

---

## Comparison Matrix

| Criterion | Wails | Electron | Tauri | Embedded Server | Pure Go UI |
|-----------|-------|----------|-------|-----------------|------------|
| **Binary Size** | 10-15MB | 150-200MB | 25-30MB | 25-30MB | 10-15MB |
| **Memory Usage** | ~30MB | ~150MB | ~40MB | ~30MB | ~20MB |
| **Startup Time** | <50ms | ~500ms | <100ms | <100ms | <10ms |
| **React Support** | ✅ Native | ✅ Native | ✅ Native | ✅ Via Browser | ❌ No |
| **Go Integration** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Dev Experience** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Type Safety** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Learning Curve** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Desktop Features** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **Distribution** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Bnto Alignment** | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

## Recommendation: Wails

### Primary Recommendation: **Wails v3**

**Rationale:**

1. **Purpose-Built for Go + React**: Wails is specifically designed for this exact use case
2. **Performance Aligned with Bnto Goals**: Maintains the small binary and fast startup that define Bnto
3. **Seamless Integration**: Can reuse 100% of existing Bnto packages without modification
4. **Type Safety**: Auto-generated TypeScript definitions prevent runtime errors
5. **Developer Experience**: Hot reload, familiar React tooling, Go backend development
6. **Production Ready**: App Store compliant, cross-platform, no CGO on Windows

### Architecture Design

```
bnto-desktop/
├── main.go                      # Wails app entry point
├── app.go                       # App struct with methods
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── WorkflowEditor.tsx
│   │   │   ├── WorkflowRunner.tsx
│   │   │   ├── NodeLibrary.tsx
│   │   │   └── ExecutionViewer.tsx
│   │   ├── App.tsx
│   │   └── main.tsx
│   ├── wailsjs/
│   │   └── go/                  # Auto-generated TS bindings
│   └── package.json
├── internal/
│   └── desktop/
│       ├── workflows.go         # Workflow management for desktop
│       ├── settings.go          # Desktop-specific settings
│       └── bridge.go            # Wails bridge logic
└── pkg/                         # Existing Bnto packages (unchanged)
    ├── neta/
    ├── itamae/
    └── ...
```

**Key Methods to Expose:**

```go
// app.go
type App struct {
    ctx context.Context
}

// Workflow operations
func (a *App) LoadWorkflow(path string) (*neta.Definition, error)
func (a *App) SaveWorkflow(path string, def *neta.Definition) error
func (a *App) RunWorkflow(path string, input map[string]interface{}) (string, error)
func (a *App) ValidateWorkflow(path string) ([]string, error)

// Node library
func (a *App) ListNodeTypes() []string
func (a *App) GetNodeSchema(nodeType string) (interface{}, error)

// Execution monitoring
func (a *App) GetExecutionStatus(executionID string) (*itamae.Status, error)
func (a *App) CancelExecution(executionID string) error

// Events (via Wails runtime)
func (a *App) onExecutionProgress(progress float64) {
    runtime.EventsEmit(a.ctx, "execution:progress", progress)
}
```

**React Components Structure:**

```typescript
// Frontend architecture
src/
├── components/
│   ├── WorkflowEditor/          # Visual workflow editor
│   │   ├── Canvas.tsx
│   │   ├── NodePalette.tsx
│   │   └── PropertyPanel.tsx
│   ├── WorkflowRunner/          # Execution UI
│   │   ├── ExecutionView.tsx
│   │   ├── LogViewer.tsx
│   │   └── ProgressBar.tsx
│   └── Settings/                # App settings
├── hooks/
│   ├── useWorkflow.ts           # Workflow state management
│   └── useExecution.ts          # Execution state
├── services/
│   └── api.ts                   # Wrapper around wailsjs
└── types/
    └── bnto.ts                 # TypeScript types (from Go)
```

---

## Alternative/Hybrid Approach: Wails + Charm TUI

Given that the roadmap includes **Charm TUI** and **Charm TUI Bnto Editor**, consider a hybrid approach:

### Strategy: Three Interfaces, One Codebase

```
┌─────────────────────────────────────────────────┐
│           Bnto Core Packages (pkg/)            │
│    (neta, itamae, pantry, hangiri, etc.)        │
└─────────────────────────────────────────────────┘
                      │
        ┌─────────────┼─────────────┐
        │             │             │
    ┌───▼───┐    ┌───▼───┐    ┌───▼────┐
    │  CLI  │    │  TUI  │    │Desktop │
    │(Cobra)│    │(Charm)│    │(Wails) │
    └───────┘    └───────┘    └────────┘
```

**Benefits:**
- CLI for automation/scripting
- TUI for terminal power users
- Desktop for visual workflow editing

**Implementation:**
```
bnto/
├── cmd/
│   ├── bnto/              # CLI (existing)
│   ├── bnto-tui/          # Charm TUI (planned)
│   └── bnto-desktop/      # Wails desktop app (new)
└── pkg/                    # Shared packages (unchanged)
```

Each interface would:
- Share the same workflow execution engine
- Share the same node library
- Share the same validation logic
- Provide interface-appropriate UX

---

## Implementation Phases

### Phase 1: Proof of Concept (Week 1-2)
- Set up Wails project with React + TypeScript
- Expose 3-5 core Bnto functions
- Build minimal workflow runner UI
- Validate Go ↔ React communication
- **Deliverable**: Demo running a simple workflow

### Phase 2: Core Features (Week 3-6)
- Workflow file browser
- Workflow execution with live progress
- Log viewer
- Node library browser
- **Deliverable**: Functional workflow runner

### Phase 3: Workflow Editor (Week 7-12)
- Visual workflow editor (React Flow or similar)
- Node palette (drag-and-drop)
- Property editor
- Validation feedback
- **Deliverable**: Complete workflow authoring tool

### Phase 4: Polish & Distribution (Week 13-14)
- Settings/preferences
- System tray integration
- Auto-updates (optional)
- Build scripts for all platforms
- **Deliverable**: Distributable binaries

---

## Technical Considerations

### Code Organization (Bento Box Principle)

**Maintain Clean Boundaries:**

```go
// ✅ GOOD - Desktop-specific code isolated
pkg/desktop/
├── bridge/
│   └── wails_adapter.go    # Wails-specific bridge
├── workflows/
│   └── manager.go          # Desktop workflow management
└── settings/
    └── preferences.go      # Desktop settings

// ✅ GOOD - Core packages remain UI-agnostic
pkg/itamae/
└── executor.go             # No knowledge of Wails

// ❌ BAD - Mixing concerns
pkg/itamae/
└── executor.go
    ├── Execute()           # Core logic
    └── ExecuteForWails()   # UI-specific variant
```

### Testing Strategy

```go
// Core packages: 100% unit tested (unchanged)
pkg/itamae/executor_test.go

// Desktop bridge: Integration tested
pkg/desktop/bridge/wails_adapter_test.go

// React components: Jest + React Testing Library
frontend/src/components/__tests__/
```

### Build Process

```bash
# Development
make dev-desktop     # Runs wails dev

# Production builds
make build-desktop-mac
make build-desktop-windows
make build-desktop-linux

# Combined release
make release-all     # CLI + TUI + Desktop binaries
```

---

## Risk Assessment

### Technical Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Wails learning curve | Medium | Medium | Start with POC, evaluate early |
| Go ↔ JS type mismatches | Low | Low | Use generated TypeScript types |
| WebView inconsistencies | Medium | Low | Test on all platforms early |
| Binary size creep | Low | Low | Monitor builds, profile assets |

### Strategic Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Duplicate effort (TUI + Desktop) | High | Medium | Share core packages, different UIs |
| Maintenance burden | Medium | High | Keep desktop code isolated |
| User confusion (3 interfaces) | Medium | Medium | Clear messaging about use cases |

---

## Decision Criteria

### Choose Wails If:
- ✅ You want a true desktop application
- ✅ React UI is a hard requirement
- ✅ Performance and binary size matter
- ✅ You want type-safe Go ↔ JS communication
- ✅ You plan to distribute via app stores

### Choose Embedded Server (Option 4) If:
- ✅ You prefer pure HTTP/REST architecture
- ✅ You want maximum flexibility
- ✅ Browser-based UI is acceptable
- ✅ You value simplicity over native desktop features

### Reconsider Requirements If:
- ❌ Electron is being considered (conflicts with Bnto's goals)
- ❌ Tauri is being considered (unnecessary Rust layer)
- ❌ Pure Go UI is acceptable (then skip React requirement)

---

## Next Steps

1. **Validate Assumptions**
   - Confirm React is required (vs Charm TUI only)
   - Confirm desktop app is needed (vs web-based)
   - Align with existing TUI roadmap

2. **POC Development** (if proceeding with Wails)
   - Install Wails CLI: `go install github.com/wailsapp/wails/v2/cmd/wails@latest`
   - Initialize project: `wails init -n bnto-desktop -t react-ts`
   - Expose 3-5 Bnto functions
   - Build simple workflow runner UI
   - Evaluate developer experience

3. **Document Decision**
   - Update `.claude/README.md` with desktop strategy
   - Add desktop phases to roadmap
   - Define CLI vs TUI vs Desktop use cases

4. **Communicate Strategy**
   - Share this document with stakeholders
   - Gather feedback on approach
   - Finalize technology selection

---

## References

### Wails Resources
- Official Docs: https://wails.io/docs/introduction/
- GitHub: https://github.com/wailsapp/wails
- Templates: https://wails.io/docs/guides/templates/

### Alternative Frameworks
- Electron: https://www.electronjs.org/
- Tauri: https://tauri.app/
- Fyne: https://fyne.io/

### React Desktop Patterns
- React Flow (workflow editor): https://reactflow.dev/
- React DnD (drag-drop): https://react-dnd.github.io/react-dnd/

---

## Conclusion

**Recommended Path Forward:**

1. **Short-term**: Build Wails POC (1-2 weeks)
2. **Validate**: Ensure Wails meets requirements
3. **Decide**: Commit to Wails or re-evaluate
4. **Implement**: Follow phased approach (14 weeks)

**Key Success Factors:**
- Maintain Bento Box Principle (clean separation)
- Reuse 100% of existing Bnto packages
- Keep CLI, TUI, and Desktop as separate interfaces
- Monitor binary size and performance
- Provide clear use cases for each interface

---

**Status:** Evaluation complete, awaiting decision
**Recommended:** Wails v3 with React + TypeScript
**Next Action:** Build POC to validate approach
