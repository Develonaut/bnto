# Bento - Go CLI for Workflow Automation

**Date:** 2025-10-18
**Purpose:** Port Atomiton's core architecture from TypeScript to Go for a high-performance CLI tool
**TUI Framework:** Charm/Bubble Tea (Phase 2)

---

## Project Overview

Bento is a complete rewrite of Atomiton's core execution engine in Go, designed to be:

- ✅ **Fast:** 15-30x faster startup, 6x less memory
- ✅ **Small:** 20MB binary vs 202MB node_modules
- ✅ **Portable:** Single binary, cross-platform (Linux, macOS, Windows)
- ✅ **Powerful:** True parallelism with goroutines
- ✅ **Beautiful:** Charm/Bubble Tea TUI (Phase 2)

---

## Documentation Index

### Core Analysis Documents

1. **[ATOMITON_PACKAGE_AUDIT.md](./ATOMITON_PACKAGE_AUDIT.md)**
   - Complete audit of all 18 Atomiton packages
   - Package classification (Core, Infrastructure, UI, Build)
   - Dependency graph
   - Migration priorities and roadmap
   - 10-14 week timeline

2. **[COMPLETE_NODE_INVENTORY.md](./COMPLETE_NODE_INVENTORY.md)**
   - Detailed catalog of ALL 10 node types
   - Parameter specifications
   - Go implementation strategies
   - Testing requirements
   - Migration effort estimates (5-7 weeks for all nodes)

3. **[JSON_MIGRATION_NOTES.md](./JSON_MIGRATION_NOTES.md)**
   - Decision to use JSON instead of YAML
   - Rationale and benefits
   - Code examples
   - Struct tags and validation

### Agents

Located in `.claude/agents/`:

- **Karen.md** - Agent persona from Bento
- **Colossus.md** - Agent persona from Bento

---

## Executive Summary

### Packages to Port (Priority Order)

1. **@atomiton/nodes** ✅ CRITICAL - Foundation layer
2. **@atomiton/storage** ✅ CRITICAL - File I/O (JSON format)
3. **@atomiton/conductor** ✅ CRITICAL - Execution engine
4. **@atomiton/logger** ✅ HIGH - Structured logging
5. **@atomiton/validation** ✅ HIGH - Schema validation
6. **@atomiton/utils** ✅ MEDIUM - Utilities
7. **@atomiton/cli** 📋 REFERENCE - CLI design patterns

### Packages to Skip

- ❌ @atomiton/yaml (using JSON instead)
- ❌ @atomiton/api (Electron IPC - not needed for standalone CLI)
- ❌ @atomiton/editor (React Flow visual editor)
- ❌ @atomiton/ui (React components)
- ❌ @atomiton/hooks (React hooks)
- ❌ @atomiton/router (Web routing)
- ❌ @atomiton/store (React state management)
- ❌ Build tooling packages

---

## Complete Node Library (10 Nodes - ALL Required)

| # | Node Type | Complexity | Effort | Priority |
|---|-----------|------------|--------|----------|
| 1 | edit-fields | ⭐⭐ Low | 2-3 days | Phase 2 |
| 2 | http-request | ⭐⭐ Low | 2-3 days | Phase 3 |
| 3 | file-system | ⭐⭐ Low | 2-3 days | Phase 3 |
| 4 | shell-command | ⭐⭐ Low | 1-2 days | Phase 3 |
| 5 | group | ⭐⭐⭐ Medium | 3-4 days | Phase 2 |
| 6 | loop | ⭐⭐⭐ Medium | 3-4 days | Phase 2 |
| 7 | spreadsheet | ⭐⭐⭐ Medium | 3-5 days | Phase 3 |
| 8 | parallel | ⭐⭐⭐⭐ High | 4-5 days | Phase 2 |
| 9 | image | ⭐⭐⭐⭐ High | 1-2 weeks | Phase 4 |
| 10 | transform | ⭐⭐⭐⭐⭐ Very High | 1-2 weeks | Phase 4 |

**Total:** 5-7 weeks for complete 1:1 node parity

---

## Migration Phases

### Phase 1: Foundation (Weeks 1-2)
**Goal:** Core types and JSON I/O

```
✅ Core types (NodeDefinition, Executable, Port, Edge)
✅ JSON serialization (LoadNodeFile, SaveNodeFile)
✅ ID generation utilities
```

**Deliverable:** Can load/save flow files
```bash
bentobox validate workflow.flow.json  # ✅ Parse and validate
```

---

### Phase 2: Simple Nodes + Conductor Foundation (Weeks 3-4)
**Goal:** Basic execution capability

```
✅ edit-fields node
✅ group node
✅ loop node
✅ parallel node
✅ Conductor skeleton (graph execution)
```

**Deliverable:** Can execute simple flows
```bash
bentobox run simple-flow.flow.json  # ✅ Execute
```

---

### Phase 3: I/O Nodes + Infrastructure (Weeks 5-6)
**Goal:** Real-world automation capability

```
✅ http-request node
✅ file-system node
✅ shell-command node
✅ spreadsheet node
✅ Logger (zerolog)
✅ Validation (validator.v10)
```

**Deliverable:** Production-ready for most workflows
```bash
bentobox run api-workflow.flow.json --progress
```

---

### Phase 4: Advanced Nodes (Weeks 7-8)
**Goal:** Complete node parity

```
✅ image node (govips)
✅ transform node (expr or v8go)
```

**Deliverable:** 100% node parity with TypeScript

---

### Phase 5: Full Conductor (Week 9)
**Goal:** Production orchestration engine

```
✅ Progress tracking
✅ Event emission
✅ Error handling
✅ Debug features
✅ Execution traces
```

**Deliverable:** Enterprise-grade execution engine

---

### Phase 6: CLI Commands (Week 10)
**Goal:** Complete CLI tool

```
✅ bentobox run
✅ bentobox validate
✅ bentobox list
✅ bentobox create
✅ Configuration (Viper)
```

**Deliverable:** Full CLI functionality

---

### Phase 7: TUI (Weeks 11-13) - OPTIONAL
**Goal:** Beautiful interactive interface

```
⏸️ Bubble Tea app
⏸️ Flow list screen
⏸️ Execution viewer
⏸️ Progress visualization
```

**Deliverable:** Interactive TUI (like Atomiton Ink CLI)

---

### Phase 8: Distribution (Week 14)
**Goal:** Release preparation

```
✅ Cross-compilation
✅ Homebrew formula
✅ Installation scripts
✅ Documentation
✅ Performance benchmarks
```

**Deliverable:** Distributable binary

---

## Recommended Go Libraries

### AWAITING USER INPUT

Before proceeding, you should specify preferences for:

1. **Image Processing:**
   - Option A: govips (fast, libvips wrapper, recommended)
   - Option B: imaging (pure Go, slower)
   - Option C: bimg (libvips wrapper)

2. **Transform Node (JavaScript execution):**
   - Option A: expr (expression language, 90% coverage, recommended MVP)
   - Option B: v8go (full JavaScript, requires CGO)
   - Option C: goja (pure Go JavaScript, slower)
   - Option D: Hybrid (expr + v8go)

3. **Logging:**
   - Option A: zerolog (fast, structured, recommended)
   - Option B: zap (fast, flexible)
   - Option C: logrus (popular, feature-rich)

4. **Validation:**
   - Option A: go-playground/validator (recommended, struct tags)
   - Option B: ozzo-validation (code-based)
   - Option C: govalidator (simple)

5. **HTTP Client:**
   - Option A: net/http (stdlib, recommended)
   - Option B: fasthttp (faster, less compatible)
   - Option C: resty (convenient wrapper)

6. **CLI Framework:**
   - Option A: Cobra (recommended, industry standard)
   - Option B: urfave/cli (simpler)
   - Option C: flag (stdlib only)

7. **TUI Framework (Phase 7):**
   - Bubble Tea (Charm stack, recommended)
   - tview (alternative)

---

## Performance Projections

### Node.js Atomiton CLI

```
Binary Size:      202MB (node_modules)
Startup Time:     150ms
Memory:           30MB baseline, 150MB peak
Parallel Nodes:   Event loop (single-threaded)
```

### Go Bento CLI (Projected)

```
Binary Size:      20MB (static binary, all deps included)
Startup Time:     5-10ms
Memory:           5MB baseline, 20MB peak
Parallel Nodes:   True parallelism (goroutines, multi-core)
```

**Improvements:**
- ✅ 90% smaller binary
- ✅ 15-30x faster startup
- ✅ 6x less memory
- ✅ True parallel execution
- ✅ Single binary distribution

---

## Project Structure (Actual)

```
bento/
├── cmd/
│   └── bento/
│       ├── main.go
│       ├── savor.go         # Execute command
│       ├── sample.go         # Validate command
│       ├── menu.go           # List command
│       ├── box.go            # Create command
│       ├── version.go
│       ├── output.go
│       └── main_test.go
│
├── pkg/
│   ├── neta/               # "Ingredients" - Workflow nodes
│   │   ├── definition.go
│   │   ├── executable.go
│   │   ├── ports.go
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
│   ├── hangiri/            # "Wooden Rice Tub" - Storage
│   │   ├── hangiri.go
│   │   └── hangiri_test.go
│   │
│   ├── itamae/             # "Sushi Chef" - Orchestration
│   │   ├── itamae.go
│   │   ├── parallel.go
│   │   ├── errors.go
│   │   └── *_test.go
│   │
│   ├── pantry/             # Registry of neta types
│   │   ├── pantry.go
│   │   └── pantry_test.go
│   │
│   ├── shoyu/              # "Soy Sauce" - Logging
│   │   ├── shoyu.go
│   │   └── shoyu_test.go
│   │
│   └── omakase/            # "Chef's Choice" - Validation
│       ├── omakase.go
│       └── omakase_test.go
│
├── .claude/
│   ├── README.md (this file)
│   ├── ATOMITON_PACKAGE_AUDIT.md
│   ├── BENTO_BOX_PRINCIPLE.md
│   ├── COMPLETE_NODE_INVENTORY.md
│   ├── EMOJIS.md
│   ├── PACKAGE_NAMING.md
│   ├── STATUS_WORDS.md
│   ├── agents/
│   │   ├── Karen.md
│   │   └── Colossus.md
│   └── workflow/
│       └── REVIEW_CHECKLIST.md
│
├── go.mod
├── go.sum
└── README.md
```

---

## Key Decisions

### ✅ Confirmed

1. **Use JSON for flow files** (not YAML)
   - Simpler parsing
   - Stdlib only
   - Faster

2. **Port ALL 10 nodes** (1:1 parity)
   - Complete compatibility
   - No missing features

3. **Skip Electron/IPC packages**
   - Standalone CLI only
   - No client-server architecture (for now)

4. **Focus on CLI first, TUI later**
   - Phase 1-6: CLI
   - Phase 7: Bubble Tea TUI (optional)

### 🔄 Awaiting Input

1. **Go library choices** (see "Recommended Go Libraries" above)
2. **Transform node strategy** (expr vs v8go?)
3. **Timeline approval** (10-14 weeks acceptable?)
4. **Feature priorities** (anything to add/remove?)

---

## Next Steps

1. **Review documentation** - Read the 3 analysis documents
2. **Specify library preferences** - Choose Go libraries for each component
3. **Approve roadmap** - Confirm phases and timeline
4. **Initialize Go project** - Set up `go.mod`, directory structure
5. **Start Phase 1** - Begin with foundation layer

---

## Questions for Discussion

1. **Transform node:** expr (simple) or v8go (full JS)?
2. **Image processing:** govips (fast) or imaging (pure Go)?
3. **TUI priority:** Phase 7 (optional) or earlier?
4. **Distribution:** Homebrew only or apt/yum too?
5. **Timeline:** 10-14 weeks acceptable?

---

## Success Criteria

### Project Complete When:

- ✅ All 10 nodes implemented and tested
- ✅ Conductor executes flows with progress tracking
- ✅ CLI commands fully functional
- ✅ Performance >= TypeScript version
- ✅ Cross-platform binaries built
- ✅ Documentation complete
- ✅ Ready for distribution

---

## Contact & Collaboration

**Claude Code Agents:**
- Karen (copied from Bento)
- Colossus (copied from Bento)

**Project Location:** `/Users/Ryan/Code/Bento`

---

**Status:** ✅ Analysis complete, awaiting library selection and approval to proceed
**Last Updated:** 2025-10-18
**Next Action:** Review documentation and provide library preferences
