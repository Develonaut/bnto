# Go for Node/React Developers — The Bnto Edition

A reference guide for understanding Go through the lens of Node/npm/React.

---

## The Big Picture: npm world vs Go world

| Concept | Node/npm | Go |
|---|---|---|
| **Workspace** | `pnpm-workspace.yaml` | `go.work` |
| **Package config** | `package.json` | `go.mod` |
| **Lock file** | `pnpm-lock.yaml` | `go.sum` |
| **Scripts** | `"scripts"` in package.json | `Taskfile.yml` |
| **node_modules** | Downloaded deps folder | `$GOPATH/pkg/mod` (global cache, invisible) |
| **A package** | A folder with `package.json` | A folder — that's it. Every folder is a package |
| **Imports** | `import { thing } from "@bnto/core"` | `import "github.com/Develonaut/bnto/pkg/engine"` |
| **Build output** | Still JS files, needs Node to run | Single binary. No runtime needed |

---

## Your Two Go Modules

```
go.work                          <- like pnpm-workspace.yaml
├── engine/go.mod                <- "github.com/Develonaut/bnto"
└── apps/api/go.mod              <- "github.com/Develonaut/bnto-api"
```

The API server imports from the engine just like `apps/web` imports from `@bnto/core`. The `go.work` file makes it resolve locally during development (like pnpm workspace links).

---

## Reading a Go File — Annotated

```go
package engine    // Every file declares what package it's in.
                  // ALL files in the same folder MUST have the same package name.
                  // This is like: every file in packages/core/src/
                  // being part of "@bnto/core"

import (
    "context"                                          // stdlib (no URL = stdlib)
    "fmt"                                              // stdlib
    "github.com/Develonaut/bnto/pkg/node"              // your own package (URL = external/workspace)
    "github.com/Develonaut/bnto/pkg/registry"
)

// Struct = your "class" (but no inheritance, ever)
type Engine struct {
    registry  *registry.Registry       // * means pointer (reference, not copy)
    logger    *logger.Logger           // like: private registry: Registry
}

// Constructor function. Go doesn't have constructors -- just a function
// that returns your struct. By convention always called New() or NewXxx()
func New(reg *registry.Registry, log *logger.Logger) *Engine {
    return &Engine{                    // & means "give me a pointer to this"
        registry: reg,
        logger:   log,
    }
}

// Method on Engine. The (e *Engine) part is the "receiver" -- it's "this"
// Think: Engine.prototype.Serve = function(ctx, def) { ... }
func (e *Engine) Serve(ctx context.Context, def *node.Definition) (*Result, error) {
    //                                                              ^^^^^^^^^^^^^^^^
    //                         Go functions return MULTIPLE values.
    //                         The last one is almost always "error".
    //                         This replaces try/catch entirely.

    if err := e.validate(ctx, def); err != nil {
        return nil, fmt.Errorf("validation failed: %w", err)
        //                                          ^^ wraps the original error
    }

    result, err := e.execute(ctx, def)
    if err != nil {                              // You'll see this pattern EVERYWHERE
        return nil, fmt.Errorf("execution: %w", err)
    }

    return result, nil                           // nil error = success
}
```

---

## Visibility: The Simplest Rule in Go

**PascalCase = exported (public). camelCase = unexported (private).**

That's the entire visibility system. No `export` keyword, no `public/private` modifiers.

```go
type Engine struct { ... }           // Exported -- other packages can use it
type executionContext struct { ... }  // Unexported -- only this package sees it

func New() *Engine { ... }           // Exported -- it's the constructor
func (e *Engine) Serve() { ... }     // Exported -- public method
func (e *Engine) validate() { ... }  // Unexported -- internal helper
```

TS equivalent:
```typescript
export class Engine { ... }          // PascalCase struct = exported
class executionContext { ... }       // camelCase struct = not exported

export function New(): Engine { }    // PascalCase function = exported
```

---

## File Naming

Always `snake_case.go`. Never PascalCase, never camelCase for files.

```
engine.go              <- main type + constructor
executor_single.go     <- single node execution
executor_group.go      <- group node execution
loop_foreach.go        <- forEach loop logic
engine_test.go         <- tests (always _test.go suffix)
```

---

## Interfaces Work Backwards from TypeScript

In TS, you explicitly say `implements`:
```typescript
class Image implements Executable {
  execute(ctx: Context, params: Params): Result { ... }
}
```

In Go, you just... have the right methods. No keyword needed:
```go
// This interface exists in node/executable.go
type Executable interface {
    Execute(ctx context.Context, params map[string]interface{}) (interface{}, error)
}

// This struct in image/image.go satisfies it automatically
// because it has an Execute method with the right signature.
// Nothing declares "Image implements Executable" anywhere.
type Image struct{}

func (i *Image) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
    // ...
}
```

If `Image` has `Execute` with the right signature, it *is* an `Executable`. The compiler checks at usage time, not at definition time. This is called "structural typing" -- TypeScript actually does this too at the type level, Go just does it at the interface level.

---

## Package Structure — Mapped to Your TS Packages

```
engine/pkg/                        packages/
├── api/          <- service.go     ├── core/          <- the public API
├── engine/       <- orchestrator   |   (engine is internal to Go,
├── registry/     <- node registry  |    core is the TS equivalent
├── node/         <- types/ifaces   |    of the public surface)
|   └── library/
|       ├── image/     <- each node type = its own package
|       ├── http/      <- like each node being its own npm package
|       ├── loop/
|       └── ...
├── validator/
├── storage/      <- ~/.bnto persistence
├── paths/        <- config/path resolution
└── logger/       <- structured logging
```

**Key rule:** packages only depend downward, never sideways or up. `engine` imports `node` and `registry`, but `node` never imports `engine`. Same principle as your TS architecture (Apps -> Core -> Engine).

---

## The Five Patterns You'll See Everywhere

### 1. Constructor + Methods (the "class" pattern)

```go
// Constructor -- always called New() by convention
func New(deps ...) *Thing { return &Thing{...} }

// Methods -- (t *Thing) is "this"
func (t *Thing) DoStuff(ctx context.Context) error { ... }
```

TS equivalent:
```typescript
class Thing {
  constructor(deps) { ... }
  doStuff(): Promise<void> { ... }
}
```

### 2. The Error Check (replaces try/catch)

```go
result, err := doSomething()
if err != nil {
    return fmt.Errorf("context about what failed: %w", err)
}
// use result safely here
```

TS equivalent:
```typescript
try {
  const result = await doSomething();
} catch (err) {
  throw new Error(`context about what failed: ${err.message}`);
}
```

The `%w` verb wraps the original error so callers can unwrap it later (like `cause` in JS Error). You always add context -- "what were you trying to do when this failed?"

### 3. defer (replaces try/finally)

```go
file, err := os.Open("data.json")
if err != nil { return err }
defer file.Close()    // runs when this function exits, no matter what
// use file...
```

TS equivalent:
```typescript
const file = await fs.open("data.json");
try {
  // use file...
} finally {
  file.close();
}
```

Multiple defers run in reverse order (LIFO), like a stack of finally blocks.

### 4. Goroutines (replaces async/background work)

```go
// In your API server -- fire and forget
go runAsync(svc, mgr, id, def, timeout)    // starts in background
writeJSON(w, 202, response)                 // responds immediately
```

TS equivalent:
```typescript
// Don't await -- fire and forget
runAsync(svc, mgr, id, def, timeout);
res.status(202).json(response);
```

The `go` keyword spawns a lightweight thread (goroutine). Thousands of them cost almost nothing -- unlike Node worker threads.

### 5. context.Context (replaces AbortController)

```go
ctx, cancel := context.WithTimeout(ctx, 30*time.Second)
defer cancel()
result, err := engine.Serve(ctx, def)  // will be cancelled after 30s
```

TS equivalent:
```typescript
const controller = new AbortController();
setTimeout(() => controller.abort(), 30_000);
const result = await engine.serve(def, { signal: controller.signal });
```

Context flows through every function as the first parameter. It carries cancellation signals, deadlines, and request-scoped values.

---

## The Concurrency Mental Shift

In Node: everything is async by default and single-threaded.
In Go: everything is synchronous by default and multi-threaded.

That's why you see things that don't exist in Node:

| Go concept | Why it exists | Node equivalent |
|---|---|---|
| `sync.RWMutex` | Thread safety -- multiple goroutines can access shared data | Not needed (single-threaded) |
| `context.Context` everywhere | Cancellation propagation across goroutines | AbortSignal (but used way less) |
| `go func()` to make things async | Opt-in concurrency | Forgetting to `await` (but intentional) |
| `chan` (channels) | Communication between goroutines | EventEmitter or message passing |

Your Go code reads top-to-bottom, line by line, no callbacks, no `.then()`. When you need concurrency, you explicitly opt in with the `go` keyword.

---

## Symbols Cheat Sheet

These symbols confused me coming from JS:

| Symbol | Meaning | Example |
|---|---|---|
| `*` before a type | Pointer type (reference) | `*Engine` = "reference to an Engine" |
| `&` before a value | "Get the address of" (create pointer) | `&Engine{...}` = "create and return reference" |
| `*` before a variable | Dereference (follow the pointer) | `*ptr` = "get the value this points to" |
| `:=` | Declare + assign (type inferred) | `x := 5` is like `const x = 5` |
| `=` | Assign to existing variable | `x = 10` (already declared) |
| `...` before a type | Variadic (rest params) | `func Log(args ...string)` = `function log(...args: string[])` |
| `...` after a slice | Spread | `append(a, b...)` = `[...a, ...b]` |
| `_` | Discard/ignore a value | `_, err := doThing()` (ignore first return) |

The pointer stuff (`*` and `&`) is the most foreign part. The simple mental model:

- **`&thing`** = "I want to share this, not copy it" (pass by reference)
- **`*Thing`** = "This parameter expects a shared reference" (pointer type)
- Most of the time you use `&` when creating and `*` in type signatures, and don't think about it beyond that

---

## Quick Reference: Where Things Live

### Engine (the stable core)

```
engine/
├── cmd/bnto/          <- CLI entry point (like bin/ in npm)
├── pkg/
|   ├── api/           <- public service layer (like @bnto/core's public API)
|   ├── engine/        <- orchestration (runs workflows step by step)
|   ├── registry/      <- node type registry (maps "image" -> ImageNode factory)
|   ├── node/          <- shared types (Definition, Executable interface)
|   |   └── library/   <- one sub-package per node type
|   |       ├── image/
|   |       ├── http/
|   |       └── ...
|   ├── validator/     <- workflow validation before execution
|   ├── storage/       <- ~/.bnto directory management
|   ├── paths/         <- config file resolution
|   └── logger/        <- structured logging
└── tests/
    ├── fixtures/      <- .bnto.json test workflows
    └── integration/   <- end-to-end tests using fixtures
```

### API Server (HTTP wrapper)

```
apps/api/
├── cmd/server/        <- server entry point
└── internal/          <- "internal" means ONLY this module can import it
    ├── handler/       <- HTTP route handlers (like Next.js API routes)
    ├── server/        <- router + middleware assembly (like Express app setup)
    ├── execution/     <- in-memory execution state tracking
    └── r2/            <- Cloudflare R2 file transit client
```

Note: `internal/` is a special Go convention. Packages under `internal/` can only be imported by the parent module. It's like having truly private packages -- the compiler enforces it. The engine uses `pkg/` (importable by anyone) because the API server needs to use it.

---

## Further Reading

- [Go Tour](https://go.dev/tour/) -- interactive, covers the basics in ~1 hour
- [Effective Go](https://go.dev/doc/effective_go) -- the style guide everyone follows
- [Go by Example](https://gobyexample.com/) -- practical patterns with runnable code
