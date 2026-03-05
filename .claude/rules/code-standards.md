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

- **Files:** Target 50-100 lines. Hard cap at 250 -- no exceptions. If a file approaches 150 lines, look for extraction opportunities
- **Functions:** < 20 lines. No escape hatch
- **Components:** One exported component per file. Target 50-100 lines of JSX + logic. If a component has more than 2-3 sub-components defined in the same file, break into a folder with barrel export
- **Hooks:** One exported hook per file
- **Functions/utils:** One exported function per file. If a function is more than a few lines or reused anywhere, it gets its own file (camelCase, e.g., `formatDuration.ts`). Group related functions in a domain folder, not a grab-bag file

### One Export Per File (strict)

**Every exported component, hook, or utility function gets its own file.** No exceptions for "they're related" or "they're small."

| Type           | Rule                            | File naming            |
| -------------- | ------------------------------- | ---------------------- |
| **Components** | One exported component per file | `WorkflowCard.tsx`     |
| **Hooks**      | One exported hook per file      | `use-run-workflow.ts`  |
| **Functions**  | One exported function per file  | `validate-workflow.ts` |

**The only exception:** Compound primitives that mirror a single HTML element's anatomy (shadcn's `Card` + `CardHeader` + `CardContent`) can share a file in `primitives/`. These MUST be exported as flat named exports with prefixed names (e.g., `CardHeader`, not `Card.Header`).

### Anti-Patterns

- **The utils dumping ground** -- `utils.ts` with 20 unrelated functions
- **The hooks dumping ground** -- `hooks.ts` with 5 unrelated hooks
- **God objects** -- One class/component/struct that does everything
- **Premature abstraction** -- Interfaces with 10 methods when you use 2
- **Mega-prop components** -- 15 props and 300 lines of conditional rendering
- **Multi-export files** -- Related does not mean same file. Use a folder + barrel export

---

## Composition Is the Golden Rule

Everything else follows from this: **small, focused pieces that compose together.** This applies at every level -- components compose in JSX, hooks compose in functions, packages compose in the architecture.

**Bias toward small files early.** Directory structure is your organization tool -- a folder of 5 focused files is easier to navigate, test, and fix than one 200-line file. Each file named after what it does makes the folder a readable table of contents. Co-located tests (`formatDuration.test.ts` next to `formatDuration.ts`) keep fixes focused.

### How code naturally grows

```
1. Start inline -- everything in one component file
2. Logic grows -> extract to its own file immediately (not "when it gets complex")
3. Related files accumulate -> group in a domain folder with barrel export
4. Folder becomes a package boundary -> graduate to a package
```

The trigger is size and reuse, not complexity. A 15-line utility function that's used in two places already earns its own file.

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

### Actions Pattern: State Mutation Logic in Pure Functions

**State mutation logic belongs in pure action functions, not inside hook callbacks.** When a hook computes the next state (undo capture, validation, guards, transforms) and writes it to a store, that computation is a pure function waiting to be extracted.

```typescript
// BAD -- business logic buried inside a hook callback
function useAddNode() {
  const storeApi = useEditorStoreApi();
  return useCallback((type) => {
    const state = storeApi.getState();
    if (isIoNodeType(type) && alreadyExists(state, type)) return null; // guard
    const snapshot = captureSnapshot(state.nodes, state.configs);      // undo
    const nextNodes = [...deselected, newNode];                        // transform
    storeApi.setState({ nodes: nextNodes, isDirty: true, ... });       // write
  }, [storeApi]);
}

// GOOD -- pure action function + thin wrapper hook
// actions/addNode.ts
export function addNode(state: EditorState, type: NodeTypeName): AddNodeResult | null {
  if (isIoNodeType(type) && alreadyExists(state, type)) return null;
  const snapshot = captureSnapshot(state.nodes, state.configs);
  return { nextState: { nodes: nextNodes, isDirty: true, ... }, nodeId };
}

// hooks/useAddNode.ts (~5 lines)
function useAddNode() {
  const storeApi = useEditorStoreApi();
  return useCallback((type) => {
    const result = addNode(storeApi.getState(), type);
    if (!result) return null;
    storeApi.setState(result.nextState);
    return result.nodeId;
  }, [storeApi]);
}
```

**Why this matters:**

- Pure actions are testable with plain objects — no React, no store, no mocking
- Hooks become ~5-line wrappers with zero logic to get wrong
- The pattern is followable — every hook looks the same (get state, call action, apply result)
- Actions compose — other actions or tests can call them without a store

**When to apply:** Any hook callback that does more than "get state → call one function → apply result" should have its logic extracted into a pure action. The hook is just the bridge between the action and the store.

### Testing Strategy

| Layer                   | What to test                      | How                                      | Effort       |
| ----------------------- | --------------------------------- | ---------------------------------------- | ------------ |
| **Pure functions (TS)** | Logic, edge cases, business rules | Unit tests (Vitest) -- pure input/output | **Heavy**    |
| **Hooks (TS)**          | Non-trivial logic, derived state  | `renderHook` when worth it               | **Medium**   |
| **Components (TS)**     | Renders correctly                 | Snapshot or minimal render               | **Light**    |
| **Flows**               | Everything works together         | E2E tests (Playwright)                   | **Targeted** |

**Test pure functions heavily. Test hooks and components lightly. E2E captures the user experience.**

### File Organization

**Feature folders with `utils/` subdirectories.** One concept per file, co-located tests.

#### File Naming Conventions

| Type                 | Convention                  | Example                                                   |
| -------------------- | --------------------------- | --------------------------------------------------------- |
| Components           | PascalCase `.tsx`           | `WorkflowCard.tsx`, `Button.tsx`                          |
| Hooks                | camelCase with `use` prefix | `useRunWorkflow.ts`                                       |
| Utils/functions (TS) | camelCase `.ts`             | `formatDuration.ts`                                       |
| Component folders    | PascalCase                  | `WorkflowEditor/`                                         |
| Context files        | `context.ts`                | `context.ts` (always this name inside a component folder) |
| Barrel exports       | `index.ts`                  | `index.ts`                                                |

---

## Import Discipline

- UI components are co-located in `apps/web/components/` (future `@bnto/ui`)
- Data/actions import from `@bnto/core`
- Backend types/functions stay inside `@bnto/backend` and `@bnto/core` internals
- Each package only exports what it owns
- `apps/web` NEVER imports from `@bnto/backend` directly
- **Third-party UI libraries should be wrapped** -- e.g., import form utilities from local wrappers, not from `react-hook-form` directly
- **Named imports for React** -- use `import { useState, useRef } from "react"`, not `import * as React`. For types: `import type { ComponentProps, CSSProperties } from "react"`. Never use `React.useState`, `React.ComponentProps`, etc.

---

## Code Quality

- **No secrets** in code -- environment variables for all sensitive config
- **No magic values** -- extract to constants. Use theme tokens for colors/spacing
- **No dead code** -- remove unused imports and commented-out code blocks
- **Consistent style** -- match naming conventions and file structure of similar files
