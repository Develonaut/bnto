# 🍱 The Bento Box Principle

## Core Philosophy

Like a traditional Japanese bnto box where each compartment serves a specific purpose and contains carefully prepared items, our codebase should exhibit the same level of organization and intention.

## The Five Principles

### 1. Single Responsibility 🍙
**Each compartment has one purpose**

Every file, function, and package should do ONE thing and do it well.

```go
// ✅ GOOD - One responsibility
// pkg/node/http/client.go
package http

func Execute(ctx context.Context, req Request) (Response, error) {
    // ONLY HTTP execution logic
}

// ❌ BAD - Multiple responsibilities
// pkg/utils/helpers.go
func Execute(...) { }        // Execution
func Validate(...) { }       // Validation
func Format(...) { }         // Formatting
// This is a grab bag!
```

### 2. No Utility Grab Bags 🥢
**Utilities are logically grouped**

Don't create "utils" or "helpers" packages that become dumping grounds.

```go
// ❌ BAD
pkg/utils/
  ├── helpers.go       // Everything mixed together

// ✅ GOOD - Organized by domain
pkg/
  ├── formatting/
  │   └── date.go
  ├── validation/
  │   └── email.go
  └── serialization/
      └── json.go
```

### 3. Clear Boundaries 🍤
**Well-defined interfaces between compartments**

```go
// ✅ Node (ingredient) interface - simple, clear
type Executable interface {
    Execute(ctx context.Context, params map[string]interface{}) (interface{}, error)
}

// ✅ Engine uses Node through interface
type Engine struct {
    registry *registry.Registry
}

func (e *Engine) Prepare(ctx context.Context, def node.Definition) (Result, error) {
    executable, err := e.registry.Get(def.Type)
    // ...
}
```

### 4. Composable 🍣
**Small pieces work together**

```go
// Small, focused functions that compose
func (e *Engine) Prepare(ctx context.Context, def node.Definition) (Result, error) {
    if def.IsGroup() {
        return e.prepareGroup(ctx, def)
    }
    return e.prepareSingle(ctx, def)
}

func (e *Engine) prepareSingle(...) (Result, error) { }
func (e *Engine) prepareGroup(...) (Result, error) { }
```

### 5. YAGNI 🥗
**You Aren't Gonna Need It**

Don't add features, exports, or complexity "just in case."

```go
// ❌ BAD - Unused exports
type Definition struct {
    ID string
    Type string
    FutureField string  // "We might need this later"
}

// ✅ GOOD - Only what's needed now
type Definition struct {
    ID string
    Type string
}
```

## Go-Specific Guidelines

### Package Organization
```go
// One concept per package
pkg/node/http/          # HTTP node only
pkg/node/transform/     # Transform node only
pkg/engine/             # Orchestration only
pkg/registry/           # Node type registry only
pkg/storage/            # Persistent storage only
```

### File Size
- **Target**: < 250 lines per file
- **Maximum**: 500 lines (then refactor)
- **Reason**: Files should fit in one mental "compartment"

### Function Size
- **Target**: < 20 lines
- **Maximum**: 30 lines
- **Reason**: Functions should be immediately understandable

## Bento Box Code Review Checklist

- [ ] Each file has a single, clear responsibility
- [ ] No "utils" or "helpers" grab bags
- [ ] Clear package boundaries (no circular deps)
- [ ] Small, composable functions
- [ ] No unused code or "future-proofing"
- [ ] Files < 250 lines (preferably)
- [ ] Functions < 20 lines (preferably)

## Anti-Patterns to Avoid

### The "Utils" Dumping Ground
```go
// ❌ pkg/utils/utils.go
// Everything goes here! 500+ lines!
```

### God Objects
```go
// ❌ One struct does everything
type SuperEngine struct {
    // Does execution, validation, formatting, logging, storage...
}
```

### Premature Abstraction
```go
// ❌ Creating interfaces before you need them
type NodeFactory interface {
    Create() Node
    Validate() error
    Transform() Node
    // ... 10 more methods you don't use
}
```

## Benefits

1. **Easier Navigation** - Find things quickly
2. **Simpler Testing** - Test one thing at a time
3. **Better Maintainability** - Changes are localized
4. **Clearer Dependencies** - See what depends on what
5. **Easier Onboarding** - New developers understand quickly

## Remember

> A well-organized bnto box is a joy to eat.
> A well-organized codebase is a joy to maintain.

**Keep your code compartmentalized!** 🍱
