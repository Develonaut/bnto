# Atomiton Package Audit for Go CLI Migration

**Date:** 2025-10-18
**Purpose:** Identify core packages from Atomiton TypeScript codebase to port to Go for Bentobox CLI
**Scope:** Focus on architecture foundation, NOT UI/TUI components

---

## Executive Summary

**Total Packages Audited:** 18
**Core Architecture Packages:** 7 ✅
**Infrastructure/Utilities:** 4 ✅
**UI/Frontend Packages:** 5 ❌ (Skip)
**Build Tooling:** 2 ❌ (Skip)

### Recommended for Go Migration (Priority Order):

1. **@atomiton/nodes** - Foundation layer (node definitions, executables, graph)
2. **@atomiton/yaml** - YAML parsing utilities
3. **@atomiton/storage** - File I/O and persistence
4. **@atomiton/conductor** - Execution orchestration engine
5. **@atomiton/logger** - Structured logging (Go: use zerolog)
6. **@atomiton/validation** - Schema validation (Go: use validator or go-playground/validator)
7. **@atomiton/utils** - Core utilities (ID generation, string helpers)

### Skip (UI/Frontend-Specific):

- @atomiton/editor (React Flow visual editor)
- @atomiton/ui (React components)
- @atomiton/hooks (React hooks)
- @atomiton/router (TanStack Router)
- @atomiton/store (Zustand state management)
- @atomiton/api (Electron IPC - not needed for standalone CLI)

---

## Package Classification Matrix

| Package | Type | Complexity | Go Priority | Dependencies | Notes |
|---------|------|------------|-------------|--------------|-------|
| **nodes** | Core | Medium | 🔥 CRITICAL | None | Foundation - port first |
| **yaml** | Infrastructure | Low | 🔥 CRITICAL | yaml lib | Trivial (gopkg.in/yaml.v3) |
| **storage** | Core | Low | 🔥 CRITICAL | nodes, yaml | File I/O (use Go os package) |
| **conductor** | Core | High | 🔥 CRITICAL | nodes, storage, logger, utils | Orchestration engine |
| **logger** | Infrastructure | Low | ✅ HIGH | electron-log | Use zerolog in Go |
| **validation** | Infrastructure | Low | ✅ HIGH | zod | Use go-playground/validator |
| **utils** | Infrastructure | Low | ✅ MEDIUM | uuid | ID generation, string utils |
| **cli** | Application | Medium | 📋 REFERENCE | conductor, nodes, storage | Reference for CLI design (Ink → Bubble Tea) |
| **api** | Transport | Medium | ⏸️ SKIP | nodes | Electron-specific IPC (not needed for standalone CLI) |
| **editor** | UI | High | ❌ SKIP | Many React deps | Visual flow editor (not CLI concern) |
| **ui** | UI | Medium | ❌ SKIP | React | Component library |
| **hooks** | UI | Low | ❌ SKIP | React | React hooks |
| **router** | UI | Medium | ❌ SKIP | TanStack Router | Web routing |
| **store** | UI | Medium | ❌ SKIP | Zustand | React state management |
| **eslint-config** | Build | - | ❌ SKIP | ESLint | Tooling |
| **typescript-config** | Build | - | ❌ SKIP | TypeScript | Tooling |
| **vite-config** | Build | - | ❌ SKIP | Vite | Tooling |
| **test** | Testing | - | 📋 REFERENCE | Testing utils | Use Go testing package |

---

## Detailed Package Analysis

### 🔥 CRITICAL: Core Architecture Packages

#### 1. **@atomiton/nodes** - Foundation Layer

**Purpose:** Node definitions, executables, graph analysis, serialization

**Key Components:**
- Type definitions (NodeDefinition, NodeExecutable, NodePort, NodeEdge)
- Node registry and factories
- Graph analysis (topological sort, critical path)
- YAML serialization (toYaml, fromYaml)
- Node library (http-request, edit-fields, image, transform, etc.)

**Dependencies:** ZERO (pure foundation)

**Go Translation:**
```go
pkg/nodes/
├── definition.go      # NodeDefinition, Port, Edge structs
├── executable.go      # Executable interface
├── registry.go        # Node type registry
├── factory.go         # Factory functions
├── graph/
│   ├── analyzer.go
│   ├── topological.go
│   └── critical_path.go
└── library/           # Node implementations
    ├── http/
    ├── editfields/
    ├── image/
    ├── transform/
    ├── group/
    ├── loop/
    └── parallel/
```

**Complexity:** Medium (transform node needs JS execution strategy)

**Migration Effort:** 2-3 weeks

---

#### 2. **@atomiton/yaml** - YAML Utilities

**Purpose:** YAML parsing and stringification

**Key Components:**
- Thin wrapper around `yaml` library
- Type-safe YAML operations

**Dependencies:** `yaml` npm package

**Go Translation:**
```go
pkg/yaml/
├── parse.go       # YAML parsing
└── stringify.go   # YAML generation

// Using gopkg.in/yaml.v3
import "gopkg.in/yaml.v3"

func ParseYAML(data []byte, v interface{}) error {
    return yaml.Unmarshal(data, v)
}

func ToYAML(v interface{}) ([]byte, error) {
    return yaml.Marshal(v)
}
```

**Complexity:** TRIVIAL (1:1 mapping to gopkg.in/yaml.v3)

**Migration Effort:** 1 day

---

#### 3. **@atomiton/storage** - Persistence Layer

**Purpose:** File I/O, path management, YAML serialization

**Key Components:**
- Storage engine abstraction (filesystem, in-memory)
- Flow file I/O (loadNodeFile, saveNodeFile)
- Path management (desktop paths, sanitization)
- Platform-specific storage (browser vs desktop)

**Dependencies:** nodes, js-yaml

**Go Translation:**
```go
pkg/storage/
├── engine.go          # IStorageEngine interface
├── filesystem.go      # Filesystem implementation
├── memory.go          # In-memory implementation
├── yaml.go            # YAML operations
└── paths/
    ├── manager.go     # Path management
    └── sanitize.go    # Path sanitization

// Example
func LoadNodeFile(path string) (*nodes.Definition, error) {
    data, err := os.ReadFile(path)
    if err != nil {
        return nil, err
    }

    var node nodes.Definition
    if err := yaml.Unmarshal(data, &node); err != nil {
        return nil, err
    }

    return &node, nil
}
```

**Complexity:** LOW (Go's `os` package is superior to Node's fs)

**Migration Effort:** 1 week

---

#### 4. **@atomiton/conductor** - Orchestration Engine

**Purpose:** Graph execution, progress tracking, event emission

**Key Components:**
- Conductor API (node.run, system.health)
- Graph-based execution (executeGraph, executeGraphNode)
- Progress tracking and state management
- Event emission (progress, completion, errors)
- Execution context and result types
- Debug controller (slowMo, error simulation)
- Execution trace and analytics

**Dependencies:** nodes, storage, logger, utils

**Go Translation:**
```go
pkg/conductor/
├── conductor.go           # Main orchestrator API
├── config.go             # Configuration types
├── execution/
│   ├── graph.go          # Graph execution engine
│   ├── executor.go       # Node executor
│   ├── context.go        # Execution context
│   ├── result.go         # Execution results
│   └── progress.go       # Progress tracking
├── events/
│   └── emitter.go        # Event emission (channels)
├── store/
│   └── graph_store.go    # Execution state store
└── debug/
    └── controller.go     # Debug utilities

// Example API
conductor := conductor.New()
result, err := conductor.Run(ctx, node, conductor.WithInput(data))

// Subscribe to progress
progressCh := make(chan *conductor.ProgressState)
conductor.Events().Subscribe(progressCh)
```

**Go Advantages:**
- Goroutines for true parallel execution
- Channels for event streaming
- Context for cancellation
- Much better concurrency primitives

**Complexity:** HIGH (complex orchestration logic, state management, events)

**Migration Effort:** 2-3 weeks

---

### ✅ HIGH PRIORITY: Infrastructure Packages

#### 5. **@atomiton/logger** - Structured Logging

**Purpose:** Cross-platform logging (browser, desktop, Node.js)

**Key Components:**
- Logger interface (debug, info, warn, error)
- Scoped logging
- Metrics and analytics logging
- File transport configuration
- Desktop uses electron-log

**Dependencies:** electron-log

**Go Translation:**
```go
pkg/logger/
└── logger.go

// Use zerolog (structured, fast, zero-allocation)
import "github.com/rs/zerolog"

type Logger struct {
    zlog zerolog.Logger
}

func New(opts ...Option) *Logger {
    logger := zerolog.New(os.Stdout).With().Timestamp().Logger()

    return &Logger{zlog: logger}
}

func (l *Logger) Info(msg string, fields map[string]interface{}) {
    l.zlog.Info().Fields(fields).Msg(msg)
}

func (l *Logger) Scope(name string) *Logger {
    return &Logger{
        zlog: l.zlog.With().Str("scope", name).Logger(),
    }
}
```

**Complexity:** LOW (zerolog is excellent)

**Migration Effort:** 2-3 days

**Recommended Go Library:** `github.com/rs/zerolog` (fast, structured, zero-allocation)

---

#### 6. **@atomiton/validation** - Schema Validation

**Purpose:** Schema validation using Zod

**Key Components:**
- Thin wrapper around Zod
- Common validators (uuid, email, url, semver)
- JSON string validation

**Dependencies:** zod

**Go Translation:**
```go
pkg/validation/
└── validators.go

// Use go-playground/validator
import "github.com/go-playground/validator/v10"

type Validator struct {
    v *validator.Validate
}

func New() *Validator {
    return &Validator{v: validator.New()}
}

func (v *Validator) Validate(s interface{}) error {
    return v.v.Struct(s)
}

// Struct tags for validation
type NodeDefinition struct {
    ID      string `validate:"required,uuid"`
    Type    string `validate:"required"`
    Version string `validate:"required,semver"`
}
```

**Complexity:** LOW (validator.v10 is mature and feature-rich)

**Migration Effort:** 1-2 days

**Recommended Go Library:** `github.com/go-playground/validator/v10`

---

#### 7. **@atomiton/utils** - Core Utilities

**Purpose:** ID generation, string manipulation, delays

**Key Components:**
- ID generators (generateNodeId, generateEdgeId, generateExecutionId)
- String case transformations (titleCase, kebabCase)
- Delay utility

**Dependencies:** uuid

**Go Translation:**
```go
pkg/utils/
├── ids.go
├── strings.go
└── time.go

// ID generation
import "github.com/google/uuid"

func GenerateNodeID() string {
    return fmt.Sprintf("node_%s", uuid.New().String())
}

// String utilities
import "strings"

func TitleCase(s string) string {
    return strings.Title(s)
}

func KebabCase(s string) string {
    return strings.ToLower(
        strings.ReplaceAll(s, " ", "-"),
    )
}

// Delay
import "time"

func Delay(ms int) {
    time.Sleep(time.Duration(ms) * time.Millisecond)
}
```

**Complexity:** TRIVIAL (standard library + uuid)

**Migration Effort:** 1 day

**Recommended Go Libraries:**
- `github.com/google/uuid` for UUIDs
- Go standard library for string manipulation

---

### 📋 REFERENCE: Existing CLI Package

#### 8. **@atomiton/cli** - Current Node.js CLI

**Purpose:** Command-line interface using Ink (React for terminal)

**Key Components:**
- Ink-based TUI (React components)
- Commander.js for commands
- Flow execution UI
- Progress visualization
- State management with Zustand

**Dependencies:** conductor, nodes, storage, ink, commander, chalk, ora

**Go Translation:**
```go
cmd/bentobox/
├── main.go
├── commands/
│   ├── run.go         # bentobox run flow.yaml
│   ├── validate.go    # bentobox validate flow.yaml
│   ├── list.go        # bentobox list
│   └── create.go      # bentobox create
└── tui/               # Bubble Tea (later phase)
    ├── app.go
    ├── models/
    └── views/

// Using Cobra for CLI framework
import "github.com/spf13/cobra"

var rootCmd = &cobra.Command{
    Use:   "bentobox",
    Short: "Workflow automation CLI",
}

var runCmd = &cobra.Command{
    Use:   "run [flow.yaml]",
    Short: "Execute a flow file",
    Run: func(cmd *cobra.Command, args []string) {
        // Load flow
        node, err := storage.LoadNodeFile(args[0])

        // Execute
        conductor := conductor.New()
        result, err := conductor.Run(ctx, node)
    },
}
```

**CLI Framework:** Cobra + Viper
**TUI Framework (Phase 2):** Bubble Tea + Lip Gloss

**Migration Strategy:**
- Phase 1: Basic CLI commands (no TUI)
- Phase 2: Add Bubble Tea TUI for interactive mode

**Migration Effort:**
- CLI commands: 1 week
- TUI (Bubble Tea): 2-3 weeks (later phase)

---

### ⏸️ EVALUATE: Transport Layer

#### 9. **@atomiton/api** - Transport Abstraction

**Purpose:** Electron IPC, WebSocket, RPC transport layer

**Key Components:**
- IPC transport (Electron main ↔ renderer)
- WebSocket transport
- Channel-based RPC
- Request/response types

**Dependencies:** nodes, electron

**Go Consideration:**

**FOR STANDALONE CLI:** ❌ **SKIP** - Not needed for single-process CLI

**FOR CLIENT-SERVER ARCHITECTURE:** ✅ **INCLUDE**
```go
pkg/api/
├── transport.go      # Transport interface
├── rpc/
│   ├── request.go
│   └── response.go
└── channels/
    ├── flow.go
    ├── storage.go
    └── node.go

// gRPC or standard HTTP API
```

**Recommendation:** Start without API layer. Add later if you want:
- Daemon + CLI client architecture
- Remote execution capabilities
- Multi-user/multi-process scenarios

---

### ❌ SKIP: UI/Frontend Packages

#### @atomiton/editor
**Purpose:** Visual flow editor using React Flow
**Skip Reason:** UI component, not relevant for CLI

#### @atomiton/ui
**Purpose:** React component library
**Skip Reason:** UI components, TUI uses Bubble Tea instead

#### @atomiton/hooks
**Purpose:** React hooks
**Skip Reason:** React-specific, not applicable to Go

#### @atomiton/router
**Purpose:** TanStack Router for web navigation
**Skip Reason:** Web routing, not needed for CLI

#### @atomiton/store
**Purpose:** Zustand state management
**Skip Reason:** React state management, Go uses different patterns

---

## Dependency Graph

```
Foundation (Zero dependencies):
└── nodes
    ├── NodeDefinition types
    ├── Executable interface
    ├── Graph analysis
    └── Node library

Infrastructure:
├── yaml → (yaml library)
├── utils → (uuid library)
├── validation → (zod → Go: validator.v10)
└── logger → (electron-log → Go: zerolog)

Core Services:
├── storage → nodes, yaml
└── conductor → nodes, storage, logger, utils

Application:
└── cli → conductor, nodes, storage

Optional:
└── api → nodes (skip for standalone CLI)
```

---

## Migration Priority Roadmap

### Phase 1: Foundation (Weeks 1-2)
**Goal:** Core types and file I/O

```
✅ @atomiton/yaml        # YAML parsing (gopkg.in/yaml.v3)
✅ @atomiton/nodes       # Core types only (Definition, Executable, etc.)
✅ @atomiton/storage     # File I/O (loadNodeFile, saveNodeFile)
✅ @atomiton/utils       # ID generation, string helpers
```

**Deliverable:** Can load/save flow files in Go
```bash
go run main.go validate flow.yaml  # ✅ Parse and validate
```

---

### Phase 2: Node Library - Simple (Week 3)
**Goal:** Implement basic node types

```
✅ @atomiton/nodes/library/editfields   # Template engine
✅ @atomiton/nodes/library/group        # Grouping logic
✅ @atomiton/nodes/library/loop         # Loop execution
✅ @atomiton/nodes/library/parallel     # Goroutine-based parallelism
```

**Deliverable:** Can execute simple flows
```bash
go run main.go run simple-flow.yaml  # ✅ Execute edit-fields, group
```

---

### Phase 3: Infrastructure (Week 4)
**Goal:** Logging and validation

```
✅ @atomiton/logger        # Structured logging (zerolog)
✅ @atomiton/validation    # Schema validation (validator.v10)
```

**Deliverable:** Production-ready logging and validation

---

### Phase 4: Conductor (Weeks 5-6)
**Goal:** Full orchestration engine

```
✅ @atomiton/conductor     # Graph execution, progress, events
```

**Deliverable:** Complete execution engine with progress tracking
```bash
go run main.go run complex-flow.yaml --progress  # ✅ Shows progress
```

---

### Phase 5: Node Library - I/O (Week 7)
**Goal:** I/O nodes

```
✅ @atomiton/nodes/library/http         # HTTP client (net/http)
✅ @atomiton/nodes/library/filesystem   # File operations (os)
✅ @atomiton/nodes/library/spreadsheet  # Excel/CSV (excelize)
✅ @atomiton/nodes/library/shellcommand # Shell execution (os/exec)
```

**Deliverable:** Can perform HTTP requests, file I/O, shell commands

---

### Phase 6: Node Library - Advanced (Weeks 8-9)
**Goal:** Complex nodes

```
✅ @atomiton/nodes/library/image       # Image processing (govips)
✅ @atomiton/nodes/library/transform   # Data transformation (expr or v8go)
```

**Deliverable:** Full node library parity with TypeScript version

---

### Phase 7: CLI (Week 10)
**Goal:** Production CLI

```
✅ Basic CLI commands (Cobra)
    - bentobox run flow.yaml
    - bentobox validate flow.yaml
    - bentobox list
    - bentobox create
✅ Configuration management (Viper)
✅ Progress output (text-based)
```

**Deliverable:** Usable CLI tool
```bash
bentobox run workflow.yaml
bentobox validate workflow.yaml
bentobox list ~/flows
```

---

### Phase 8: TUI (Weeks 11-13) - OPTIONAL
**Goal:** Interactive TUI with Bubble Tea

```
⏸️ Bubble Tea setup
⏸️ Flow list screen
⏸️ Execution progress view
⏸️ Result viewer
⏸️ Settings screen
```

**Deliverable:** Beautiful interactive TUI (like existing Ink CLI)

---

### Phase 9: Distribution (Week 14)
**Goal:** Release preparation

```
✅ Cross-compilation (Linux, macOS, Windows)
✅ Installation scripts
✅ Homebrew formula
✅ apt/yum packages
✅ Documentation
✅ Performance benchmarks
```

**Deliverable:** Distributable binary for all platforms

---

## Go Package Structure Recommendation

```
bentobox/
├── cmd/
│   └── bentobox/              # CLI entry point
│       ├── main.go
│       ├── commands/
│       │   ├── run.go
│       │   ├── validate.go
│       │   ├── list.go
│       │   └── create.go
│       └── tui/               # Phase 8 (optional)
│           └── app.go
│
├── pkg/
│   ├── nodes/                 # @atomiton/nodes
│   │   ├── definition.go
│   │   ├── executable.go
│   │   ├── registry.go
│   │   ├── factory.go
│   │   ├── graph/
│   │   │   ├── analyzer.go
│   │   │   ├── topological.go
│   │   │   └── critical_path.go
│   │   └── library/
│   │       ├── editfields/
│   │       ├── http/
│   │       ├── image/
│   │       ├── transform/
│   │       ├── group/
│   │       ├── loop/
│   │       ├── parallel/
│   │       ├── filesystem/
│   │       ├── spreadsheet/
│   │       └── shellcommand/
│   │
│   ├── storage/              # @atomiton/storage
│   │   ├── engine.go
│   │   ├── filesystem.go
│   │   ├── memory.go
│   │   ├── yaml.go
│   │   └── paths/
│   │       ├── manager.go
│   │       └── sanitize.go
│   │
│   ├── conductor/            # @atomiton/conductor
│   │   ├── conductor.go
│   │   ├── config.go
│   │   ├── execution/
│   │   │   ├── graph.go
│   │   │   ├── executor.go
│   │   │   ├── context.go
│   │   │   ├── result.go
│   │   │   └── progress.go
│   │   ├── events/
│   │   │   └── emitter.go
│   │   ├── store/
│   │   │   └── graph_store.go
│   │   └── debug/
│   │       └── controller.go
│   │
│   ├── logger/               # @atomiton/logger
│   │   └── logger.go
│   │
│   ├── validation/           # @atomiton/validation
│   │   └── validators.go
│   │
│   └── utils/                # @atomiton/utils
│       ├── ids.go
│       ├── strings.go
│       └── time.go
│
├── internal/
│   └── testutil/
│
├── go.mod
├── go.sum
├── Makefile
└── README.md
```

---

## Go Dependencies

```go
// go.mod
module github.com/bentobox/bentobox

go 1.23

require (
    // Core
    gopkg.in/yaml.v3 v3.0.1
    github.com/google/uuid v1.6.0

    // Infrastructure
    github.com/rs/zerolog v1.33.0
    github.com/go-playground/validator/v10 v10.22.0

    // CLI Framework
    github.com/spf13/cobra v1.8.1
    github.com/spf13/viper v1.19.0

    // Node Libraries
    github.com/xuri/excelize/v2 v2.8.1        // Spreadsheet
    github.com/davidbyttow/govips/v2 v2.15.0  // Image processing
    github.com/antonmedv/expr v1.16.9         // Transform (expressions)
    // github.com/rogchap/v8go v0.9.0         // Transform (full JS) - optional

    // TUI (Phase 8 - optional)
    github.com/charmbracelet/bubbletea v1.2.4
    github.com/charmbracelet/lipgloss v1.0.0
    github.com/charmbracelet/bubbles v0.20.0
)
```

---

## Migration Complexity Assessment

| Package | Lines of Code | Complexity | Go Effort | Notes |
|---------|---------------|------------|-----------|-------|
| yaml | ~100 | ⭐ Trivial | 1 day | Direct gopkg.in/yaml.v3 mapping |
| utils | ~200 | ⭐ Trivial | 1 day | stdlib + uuid |
| logger | ~300 | ⭐⭐ Low | 2-3 days | Use zerolog |
| validation | ~200 | ⭐⭐ Low | 1-2 days | Use validator.v10 |
| storage | ~800 | ⭐⭐ Low | 1 week | File I/O (Go stdlib better than Node) |
| nodes (types) | ~500 | ⭐⭐ Medium | 3-4 days | Struct definitions |
| nodes (graph) | ~600 | ⭐⭐⭐ Medium | 1 week | Topological sort, critical path |
| nodes (library - simple) | ~1000 | ⭐⭐⭐ Medium | 1 week | edit-fields, group, loop, parallel |
| nodes (library - I/O) | ~1500 | ⭐⭐⭐ Medium | 1 week | http, filesystem, spreadsheet, shell |
| nodes (library - advanced) | ~1000 | ⭐⭐⭐⭐ High | 2 weeks | image (govips), transform (expr/v8go) |
| conductor | ~3000 | ⭐⭐⭐⭐⭐ Very High | 2-3 weeks | Complex orchestration, events, state |
| CLI (basic) | ~500 | ⭐⭐ Medium | 1 week | Cobra commands |
| CLI (TUI) | ~1500 | ⭐⭐⭐⭐ High | 2-3 weeks | Bubble Tea (optional) |

**Total Estimated Effort:** 10-14 weeks (2.5-3.5 months)

---

## Key Decisions

### 1. Transform Node Strategy

**TypeScript:** Uses Function constructor with pattern filtering

**Go Options:**

**A. Expression Language (Recommended for MVP)**
```go
import "github.com/antonmedv/expr"

// Supports: map, filter, reduce with simple expressions
// Example: "item.price * 1.1" or "item.age > 18"
program, _ := expr.Compile(code)
result, _ := expr.Run(program, env)
```

**Pros:** Pure Go, fast, safe, covers 90% of use cases
**Cons:** Not full JavaScript (no complex functions)

**B. V8 Embedding (Add if needed)**
```go
import "rogchap.com/v8go"

// Full JavaScript compatibility
iso := v8go.NewIsolate()
ctx := v8go.NewContext(iso)
val, _ := ctx.RunScript(jsCode, "transform.js")
```

**Pros:** Full JS compatibility
**Cons:** Requires CGO, C++ dependency, larger binary

**Recommendation:** Start with **expr**, add v8go only if users demand full JS

---

### 2. Logging Strategy

**TypeScript:** electron-log (file + console)

**Go:** zerolog (structured, fast, zero-allocation)

```go
logger := zerolog.New(os.Stdout).With().Timestamp().Logger()

logger.Info().
    Str("nodeId", node.ID).
    Dur("duration", duration).
    Msg("Node executed successfully")
```

**Advantages:**
- Structured logging (JSON)
- Zero allocation (performance)
- Beautiful console output
- File rotation via lumberjack

---

### 3. Validation Strategy

**TypeScript:** Zod (schema validation)

**Go:** go-playground/validator (struct tag validation)

```go
type NodeDefinition struct {
    ID      string   `validate:"required,uuid"`
    Type    string   `validate:"required"`
    Version string   `validate:"required,semver"`
    Name    string   `validate:"required,min=1,max=100"`
}

validate := validator.New()
err := validate.Struct(node)
```

**Advantages:**
- Compile-time type safety
- Struct tags (declarative)
- Extensive built-in validators
- Custom validation functions

---

## Performance Projections

### Current (Node.js CLI)

```
Binary Size:      202MB (node_modules)
Startup Time:     150ms
Memory Baseline:  30MB
Peak Memory:      150MB (with Ink rendering)
Parallel Nodes:   Event loop (single-threaded)
```

### Projected (Go CLI)

```
Binary Size:      20MB (static, includes all deps)
Startup Time:     5-10ms
Memory Baseline:  5MB
Peak Memory:      20MB
Parallel Nodes:   True parallelism (goroutines)
```

**Improvements:**
- ✅ 90% smaller binary
- ✅ 15-30x faster startup
- ✅ 6x less memory
- ✅ True parallel execution
- ✅ Single binary distribution (no "install Node.js")

---

## Testing Strategy

### Unit Testing
```go
// Use Go's built-in testing package
func TestLoadNodeFile(t *testing.T) {
    node, err := storage.LoadNodeFile("testdata/flow.yaml")
    assert.NoError(t, err)
    assert.Equal(t, "test-flow", node.Name)
}
```

### Integration Testing
```go
func TestConductorExecution(t *testing.T) {
    conductor := conductor.New()
    node := createTestNode()

    result, err := conductor.Run(context.Background(), node)
    assert.NoError(t, err)
    assert.True(t, result.Success)
}
```

### Recommended Libraries:
- `github.com/stretchr/testify` - Assertions
- `github.com/golang/mock` - Mocking
- Built-in `testing` package for test framework

---

## Success Criteria

### Phase 1 Complete When:
- ✅ Can load YAML flow files
- ✅ Can parse NodeDefinition structs
- ✅ Can save modified flows

### Phase 7 Complete When:
- ✅ CLI can execute all node types
- ✅ Progress tracking works
- ✅ Error handling is robust
- ✅ Performance >= TypeScript version

### Project Complete When:
- ✅ Feature parity with TypeScript CLI
- ✅ All tests passing
- ✅ Cross-platform binaries built
- ✅ Documentation complete
- ✅ Homebrew formula published

---

## Next Steps

1. **Review this audit** - Validate package selection
2. **Create Go project structure** - Set up bentobox repository
3. **Start Phase 1** - Port yaml, utils, storage
4. **Proof of concept** - Validate architecture with simple flow execution
5. **Iterate** - Build incrementally following roadmap

---

## Questions for Discussion

1. **Transform node:** expr or v8go? Or both?
2. **TUI priority:** Essential or optional (Phase 8)?
3. **API layer:** Skip entirely or plan for future daemon architecture?
4. **Testing rigor:** Unit tests only or full integration test suite?
5. **Timeline:** Is 10-14 weeks acceptable for full migration?

---

**Document Version:** 1.0
**Last Updated:** 2025-10-18
**Next Review:** After Phase 1 completion
