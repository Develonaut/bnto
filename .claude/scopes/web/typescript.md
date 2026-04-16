# TypeScript Standards

**Inference is king.** Let the compiler do the work. Code that "reads well" to the compiler also reads well to developers.

## Core Principles

- **Let TypeScript infer.** No redundant annotations. `const x = getValue()` -- never `const x: Type = getValue<Type>()`
- **Design APIs so callers never annotate.** Use constrained generics with defaults, `satisfies`, and factory patterns
- **Return types: infer, don't annotate.** Only annotate at package boundaries or when inference leaks internal types
- **No `any`.** Use `unknown` with type guards. Each `any` must have eslint-disable + justification
- **No `Record<string, unknown>` for domain data.** Use typed interfaces
- **No gratuitous `as` assertions.** Only at trust boundaries (JSON.parse, external APIs, Convex `Id<T>` -> `string`)
- **Types flow down.** Core defines types, UI and web consume them. UI never defines its own data types

## Inference Patterns

**`as const` for literal preservation:**

```typescript
export const NODE_TYPES = {
  image: "image",
  file: "file",
  http: "http",
  transform: "transform",
} as const;
export type NodeType = (typeof NODE_TYPES)[keyof typeof NODE_TYPES];
```

**`satisfies` for validation without widening:**

```typescript
const routes = {
  home: "/",
  workflows: "/workflows",
  executions: "/executions",
} as const satisfies Record<string, string>;
```

**Factory functions over manual construction:**

```typescript
// GOOD -- inference flows through the factory
const mutation = createMutation(api.workflows.save);

// BAD -- manually constructing, requires explicit types
const mutation = (args: SaveWorkflowArgs): Promise<void> => { ... };
```

## Anti-Patterns

| Anti-Pattern                              | Fix                                                 |
| ----------------------------------------- | --------------------------------------------------- |
| `const x: Foo = getFoo()`                 | `const x = getFoo()` -- inference handles it        |
| `function foo(): ReturnType` on internals | Drop annotation -- let TS infer                     |
| `<Foo>` turbofish where TS can infer      | Remove type parameter -- fix API if inference fails |
| `as Type` to "help" the compiler          | Fix the source type -- `as` hides real errors       |
| `Record<string, X>` for known keys        | Mapped type or `as const satisfies`                 |
| Separate type and value declarations      | Derive types from values with `typeof` / `as const` |
