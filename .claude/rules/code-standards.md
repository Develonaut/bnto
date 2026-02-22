# Code Standards

## The Bento Box Principle

Like a traditional Japanese bento box where each compartment serves a specific purpose and contains carefully prepared items, our codebase exhibits the same level of organization and intention.

**Five principles:**

1. **Single Responsibility** -- Each file, function, and package does ONE thing well
2. **No Utility Grab Bags** -- No `utils.ts`, `helpers.ts`, or `utils.go` dumping grounds. Group by domain
3. **Clear Boundaries** -- Well-defined interfaces between layers. No circular dependencies
4. **Composable** -- Small pieces that work together. Compound components, not mega-props
5. **YAGNI** -- Don't add features, exports, or complexity "just in case"

### Size Limits

- **Files:** < 250 lines (max 500 with justification)
- **Functions:** < 20 lines (max 30 with justification)
- **Components:** One exported component per file
- **Hooks:** One exported hook per file
- **Functions:** One exported function per file (group by domain folder, not grab-bag files)
- **Go functions:** < 20 lines (max 30 with justification)
- **Go files:** < 250 lines (max 500 with justification)

### One Export Per File (strict)

**Every exported component, hook, or utility function gets its own file.** No exceptions for "they're related" or "they're small."

| Type | Rule | File naming |
|---|---|---|
| **Components** | One exported component per file | `WorkflowCard.tsx` |
| **Hooks** | One exported hook per file | `use-run-workflow.ts` |
| **Functions** | One exported function per file | `validate-workflow.ts` |

**The only exception:** Compound primitives that mirror a single HTML element's anatomy (shadcn's `Card` + `CardHeader` + `CardContent`) can share a file in `primitives/`. These MUST be exported as dot-notation namespaces.

### Anti-Patterns

- **The utils dumping ground** -- `utils.ts` with 20 unrelated functions (TS) or `utils.go` with mixed concerns (Go)
- **The hooks dumping ground** -- `hooks.ts` with 5 unrelated hooks
- **God objects** -- One class/component/struct that does everything
- **Premature abstraction** -- Interfaces with 10 methods when you use 2
- **Mega-prop components** -- 15 props and 300 lines of conditional rendering
- **Multi-export files** -- Related does not mean same file. Use a folder + barrel export

---

## Composition Is the Golden Rule

Everything else follows from this: **small, focused pieces that compose together.** This applies at every level -- components compose in JSX, hooks compose in functions, packages compose in the architecture, Go packages compose via interfaces.

**Don't prematurely decompose.** A single file with 80 lines of straightforward code is better than three files with 25 lines each and a barrel export. Extract when complexity demands it, not because a rule says to.

### How code naturally grows

```
1. Start inline -- everything in one component file / one Go function
2. Logic gets complex -> extract a hook / helper function in the same file
3. Hook/function outgrows the file -> move to its own file
4. Multiple related files -> graduate to a folder / package
```

Each step is triggered by real complexity, not by pattern compliance.

### The Layers: Pure Functions -> Logic Hooks -> Components

The mental model for where TypeScript code lives:

```
Pure Functions          ->  Logic Hooks              ->  Components
  no React dependency       reactive wrappers            compose & render
  testable in isolation     compose pure functions       use hooks + JSX
```

**Pure functions** are the foundation. Business rules, validation, transforms -- anything that doesn't need React. These are the easiest to test and reuse.

```typescript
// Pure function -- no React, easy to test
export function canEditWorkflow(workflow: Workflow, userId: string): boolean {
  return workflow.authorId === userId && workflow.status !== "archived";
}
```

**Logic hooks** compose pure functions with React state and context. Extract these when the logic is complex or shared -- not for every component.

**Components** use hooks (inline or extracted) and render.

**The layers are a gravity model, not a mandate.** Pure functions always make sense. Logic hooks earn their way in when complexity or reuse demands it.

### Go Code Organization

The same principles apply to Go code:

```go
// Each package has one responsibility
engine/pkg/
|-- api/          # Shared service layer (BntoService, DefaultRegistry)
|-- engine/       # Orchestration ONLY
|-- registry/     # Node type registration ONLY
|-- storage/      # Persistent storage ONLY
|-- node/         # Node type definitions and implementations
|-- validator/    # Workflow validation ONLY
|-- paths/        # Path resolution + config loading ONLY
|-- logger/       # Logging ONLY
+-- secrets/      # Secrets management ONLY
```

- Accept interfaces, return structs
- Errors wrapped with context: `fmt.Errorf("loading workflow %s: %w", path, err)`
- Context propagation: all long-running operations accept and respect `context.Context`
- No mega-interfaces with 10+ methods

### Testing Strategy

| Layer | What to test | How | Effort |
|---|---|---|---|
| **Go engine** | Node execution, validation, path resolution | `go test` with race detector | **Heavy** |
| **Go API** | HTTP endpoints, request/response contracts | `httptest` integration tests | **Heavy** |
| **Pure functions (TS)** | Logic, edge cases, business rules | Unit tests (Vitest) -- pure input/output | **Heavy** |
| **Hooks (TS)** | Non-trivial logic, derived state | `renderHook` when worth it | **Medium** |
| **Components (TS)** | Renders correctly | Snapshot or minimal render | **Light** |
| **Flows** | Everything works together | E2E tests (Playwright) | **Targeted** |

**Test the Go engine heavily. Test pure functions heavily. Test hooks and components lightly. E2E captures the user experience.**

### File Organization

**Feature folders with `utils/` subdirectories.** One concept per file, co-located tests.

#### File Naming Conventions

| Type | Convention | Example |
|---|---|---|
| Business components | PascalCase `.tsx` | `WorkflowCard.tsx` |
| Primitives (shadcn) | kebab-case `.tsx` | `button.tsx` |
| Hooks | camelCase with `use` prefix | `useRunWorkflow.ts` |
| Utils/functions (TS) | camelCase `.ts` | `formatDuration.ts` |
| Go files | snake_case `.go` | `workflow_runner.go` |
| Go test files | snake_case `_test.go` | `workflow_runner_test.go` |
| Component folders | PascalCase | `WorkflowEditor/` |
| Context files | `context.ts` | `context.ts` (always this name inside a component folder) |
| Barrel exports | `index.ts` | `index.ts` |

---

## Import Discipline

- UI components are co-located in `apps/web/components/` (future `@bnto/ui`)
- Data/actions import from `@bnto/core`
- Backend types/functions stay inside `@bnto/backend` and `@bnto/core` internals
- Each package only exports what it owns
- `apps/web` NEVER imports from `@bnto/backend` directly
- **Third-party UI libraries should be wrapped** -- e.g., import form utilities from local wrappers, not from `react-hook-form` directly

---

## Code Quality

- **No secrets** in code -- environment variables for all sensitive config
- **No magic values** -- extract to constants. Use theme tokens for colors/spacing
- **No dead code** -- remove unused imports and commented-out code blocks
- **Consistent style** -- match naming conventions and file structure of similar files
