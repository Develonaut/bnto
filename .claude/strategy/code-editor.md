# Code Editor Strategy

**Last Updated:** February 2026
**Status:** Research Complete — Informing Sprint 4+ Planning

---

## What This Is

A `.bnto.json` code editor — a coding-oriented experience for power users who want to create and edit recipes as structured JSON. This is a **separate feature** from the visual canvas editor (Sprint 4). Both share the same headless primitives (Sprint 4 Wave 1-2), but the code editor is its own UX with its own persona.

Think: Notion's slash command UX meets a schema-aware JSON editor. Users who prefer code get the same power as the visual canvas, with the speed and precision of text editing.

---

## Why This Matters

1. **Power users think in code.** Developers want to type, not drag. The code editor is how they build recipes fastest.
2. **Two entry points, same outcome.** Visual canvas for visual thinkers, code editor for keyboard-first users. Both produce valid `.bnto.json`. Both use the same headless state.
3. **CLI/TUI bridge.** The code editor's architecture directly informs a future `bnto edit` CLI command. Operations are portable across platforms.
4. **Community recipes.** Power users who export and share `.bnto.json` files are the seed of a recipe ecosystem. The code editor is their authoring tool.
5. **Free, highest-intent Pro signal.** Users who create custom recipes via code editing are the strongest Pro conversion candidates.

---

## Tech Choice: CodeMirror 6

**Decision: CodeMirror 6, not Monaco.**

### Why CM6 Wins

| Dimension | CodeMirror 6 | Monaco |
|-----------|-------------|--------|
| **Bundle size** | ~93 KB gzipped (basic) | ~2.4 MB gzipped |
| **JSON Schema** | `codemirror-json-schema` (community, solid) | Built-in (first-class) |
| **Mobile** | Native support | Officially unsupported |
| **Theming** | CSS variables (direct OKLCH token integration) | Hard-coded hex in JS (sync burden) |
| **Instance isolation** | Fully independent per-instance | Global shared state |
| **Tree-shaking** | Excellent (ES6 modules) | Weak (~2 MB floor) |
| **Headless state** | `EditorState` works without DOM | Requires DOM always |
| **Documentation** | Excellent (single author) | Poor (Sourcegraph, Replit flagged this) |

### Decisive Factors for Bnto

- **Bundle size is a dealbreaker.** Our WASM engine is 606 KB gzipped. Monaco would be 4x the editor. CM6 is 1/7th.
- **CSS variable theming.** Bnto uses OKLCH tokens in `globals.css`. CM6 themes use CSS directly — no hex duplication. Monaco requires hard-coded colors in JS.
- **Mobile matters.** "Small teams getting things done" includes people on tablets. Monaco is officially unsupported on mobile.
- **Headless state.** CM6's `EditorState` runs without a DOM — enables server-side validation, Web Worker processing, and future CLI reuse.
- **Cost-first architecture.** Smaller bundle = faster loads = better Core Web Vitals = SEO advantage.

### Monaco's Only Advantage

Monaco's built-in JSON Schema support is more mature. But `codemirror-json-schema` covers the same ground (validation, autocompletion, hover tooltips, dynamic schemas) and is actively maintained. The tradeoff is clear: accept a community JSON Schema package to save 2.3 MB of bundle.

### Evidence

- **Sourcegraph** migrated Monaco → CM6, reduced page JS by 43% (6 MB → 3.4 MB).
- **Replit** chose CM6 for mobile support and modular architecture.
- **Chrome DevTools** uses CM6 for their built-in code editing.

---

## Architecture: Headless-First

The code editor follows the same headless-first principle as the visual canvas editor. Logic lives in pure functions and an immutable state model. The visual rendering is a thin shell.

```
@bnto/nodes (types, schemas, validation)      ← already built
         ↓
Sprint 4 Wave 1 pure functions (CRUD)          ← shared with visual editor
         ↓
Sprint 4 Wave 2 editor store (Zustand)         ← shared with visual editor
         ↓
CodeMirror 6 extensions (code-editor-specific) ← this feature
         ↓
React wrapper + UI chrome                      ← this feature
```

### The Shared Layer (Sprint 4 Waves 1-2)

Both the visual canvas and code editor consume:
- **Pure functions** (`addNode`, `removeNode`, `updateParams`, `addEdge`, etc.) from `@bnto/nodes`
- **Editor store** (`useEditorStore`) with `definition`, `selectedNodeId`, `isDirty`, `validationErrors`
- **Definition ↔ JSON serialization** (already native — Definition IS JSON)

The code editor adds its own layer on top: CodeMirror extensions that provide JSON-specific intelligence.

### State Flow

```
User types in CodeMirror → CM6 transaction
  → JSON text changes → JSON.parse → validate → update Definition
  → Definition changes → update editor store
  → Editor store notifies both editors (if split view)
```

```
Visual editor changes a node → editor store updates Definition
  → Definition serialized to JSON → update CodeMirror document
  → CM6 renders new text (no user-visible transaction)
```

Both editors are views of the same `Definition`. The editor store is the single source of truth.

---

## Feature Set

### Tier 1: Foundation (Must-Have)

| Feature | Implementation |
|---------|---------------|
| **JSON syntax highlighting** | `@codemirror/lang-json` (Lezer parser) |
| **Schema validation** | `codemirror-json-schema` against `.bnto.json` JSON Schema |
| **Schema autocompletion** | Property names, enum values, defaults from schema |
| **Hover tooltips** | Schema descriptions on hover (markdown rendered) |
| **Code folding** | Collapse objects/arrays (built-in with `@codemirror/lang-json`) |
| **Bracket matching** | Built-in |
| **Undo/redo** | CM6 history extension |
| **Format on save** | Pretty-print JSON |
| **Error diagnostics** | Inline squiggly underlines with actionable messages |
| **Warm theme** | OKLCH token integration via CM6 CSS theming |

### Tier 2: Intelligence (Bnto-Specific)

| Feature | Implementation |
|---------|---------------|
| **Slash commands** | `/` trigger opens command palette inline — insert node templates, change type |
| **Command palette** | Cmd-K opens cmdk overlay — app-level commands (run, validate, export, navigate) |
| **JSON path breadcrumbs** | Panel above editor showing current position: `root > nodes > [0] > parameters > quality` |
| **Node template insertion** | Slash command inserts well-formed node JSON blocks with correct ports and default params |
| **Cross-node validation** | Custom linter validates: node type exists in registry, edges reference valid nodes, no cycles |
| **Template expression hints** | Autocomplete `{{.INPUT_DIR}}`, `{{.item}}`, `{{index . "node-id" "port"}}` |

### Tier 3: Polish (Nice-to-Have)

| Feature | Implementation |
|---------|---------------|
| **Quick fixes** | Code actions on diagnostics: "Missing required field 'type'" → insert with cursor placement |
| **Outline panel** | Tree view showing node hierarchy — click to navigate |
| **Inline node previews** | Widget decorations rendering node config as visual cards |
| **Multi-cursor** | Built-in CM6 feature |
| **Search & replace** | Built-in CM6 feature |
| **Collaborative editing** | `@codemirror/collab` for future shared editing |

---

## Slash Commands: The Notion Pattern

The slash command is the bridge between "code editor" and "visual editor" ergonomics. When you type `/` in the code editor, an inline command menu appears with context-aware options.

### How It Works

1. User types `/` at a position where a node can be inserted (inside a `"nodes": [...]` array)
2. CM6 custom `CompletionSource` activates, anchored at the `/` position
3. Menu shows available commands, filtered as the user types:
   - `/image` → Insert image compression node template
   - `/csv` → Insert CSV operations node template
   - `/loop` → Insert loop container template
   - `/file` → Insert file rename node template
   - etc.
4. Arrow keys navigate, Enter selects, Escape dismisses
5. On selection, the `apply` function inserts a complete, valid node JSON block with:
   - Generated unique ID
   - Correct node type
   - Default parameters from schema
   - Default input/output ports from `NODE_TYPE_INFO`
   - Cursor positioned at the first editable parameter

### Implementation Approach

Two layers, matching the Notion pattern:

**Layer 1: Inline slash menu (CM6 tooltip + custom StateField)**
- `StateField` tracks: menu active, anchor position, filter query, selected index
- `showTooltip` facet renders the menu at the anchor position
- Items come from the same command registry that feeds cmdk
- Context-aware: only shows node types valid for the current position

**Layer 2: Global command palette (cmdk via shadcn/ui Command)**
- Cmd-K opens the app-level command palette
- Commands: Run Recipe, Validate, Format JSON, Export, Insert Node, Navigate to Node
- Shares the same command registry as the slash menu
- Not CM6-specific — works across the entire app

### Command Registry (Single Source of Truth)

```typescript
interface EditorCommand {
  id: string;                    // "insert-image-node"
  label: string;                 // "Image Compression"
  description: string;           // "Compress, resize, or convert images"
  category: "insert" | "edit" | "navigate" | "run";
  icon?: string;                 // Lucide icon name
  shortcut?: string;             // "Cmd+Shift+I"
  slashTrigger?: string;         // "/image"
  available: (state: EditorState) => boolean;  // Context-aware visibility
  execute: (state: EditorState, dispatch: Dispatch) => void;
}
```

Both the slash menu and Cmd-K palette query the same registry. A command like "Insert Image Node" appears in both — `/image` in the editor, Cmd-K → "image" in the palette.

---

## JSON Schema Strategy

### The Schema

Define a JSON Schema for `.bnto.json` in `@bnto/nodes`. This schema drives:
- CM6 validation (via `codemirror-json-schema`)
- CM6 autocompletion (property names, enum values)
- CM6 hover tooltips (schema descriptions)
- CLI validation (`bnto validate`)
- Visual editor validation (same schema, different consumer)

### Schema Composition

The schema is composed from existing `@bnto/nodes` types:

```
@bnto/nodes/src/schemas/types.ts     → ParameterSchema (per-parameter rules)
@bnto/nodes/src/schemas/image.ts     → imageSchema (image node params)
@bnto/nodes/src/schemas/*.ts         → all 10 node type schemas
@bnto/nodes/src/definition.ts        → Definition type (the document structure)
     ↓
Generated JSON Schema (build step)
     ↓
Fed to codemirror-json-schema at runtime
```

**The schema is generated, not hand-written.** A build script derives it from the TypeScript types and `ParameterSchema` objects that already exist. This ensures the schema stays in sync with the node type definitions — single source of truth.

### Dynamic Schema (Per-Node Context)

The `.bnto.json` schema changes based on context:
- When the cursor is in a node with `"type": "image"`, the `parameters` object should validate against `imageSchema`
- When in `"type": "csv"`, validate against `csvSchema`

`codemirror-json-schema` supports dynamic schemas via `updateSchema(editor, schema)`. A CM6 `ViewPlugin` watches cursor position, determines the current node type, and swaps the active schema to include the relevant parameter constraints.

---

## CLI/TUI Parallels

### Shared Operations (The Real Bridge)

The code editor and a future CLI/TUI editor share the same **operation vocabulary**:

| Operation | Code Editor (CM6) | CLI/TUI (Ratatui) |
|-----------|-------------------|-------------------|
| Add node | Slash command inserts JSON block | `:add image "Compress" quality=80` |
| Remove node | Delete JSON block + cascade edges | `:remove node-1` or `d` in normal mode |
| Update param | Edit value in JSON text | `:param quality 75` or `i` to edit |
| Validate | Inline diagnostics from schema | `:validate` prints errors |
| Navigate | Click breadcrumb or Cmd-G | `j`/`k` to move, `h`/`l` for depth |
| Undo/redo | CM6 history or Cmd-Z | `u`/`Ctrl-R` (vim-style) |
| Save | Export to `.bnto.json` | `:w compress-images.bnto.json` |

### What CAN Be Shared (TypeScript → Both)

- **`RecipeStep` type** — the discriminated union of all operations (`add-node`, `remove-node`, `set-param`, etc.)
- **`invertStep()` function** — each step is invertible for undo
- **Validation logic** — `@bnto/nodes/validate.ts` already exists
- **JSON Schema** — same schema validates in CM6 and CLI
- **Node type registry** — `NODE_TYPE_INFO` drives both slash commands and CLI `:add`
- **Command definitions** — same command IDs and descriptions, different keybindings

### What CANNOT Be Shared Directly

- **Rendering** — CM6 uses DOM, Ratatui uses terminal cells
- **Parser** — CM6 uses Lezer, CLI would use tree-sitter (different grammars, same semantic model)
- **State container** — CM6 uses `EditorState`, Rust TUI uses a struct

### The Xi-Editor Lesson

Xi-editor tried to separate core from frontend via a JSON-RPC protocol. The async boundary caused cascading complexity. **The shared layer should be a library (types + pure functions), not a running service.** Both platforms import the operations directly — no protocol, no IPC.

---

## React Integration

### Approach: Custom Hook (Not @uiw/react-codemirror)

Following CM6 author Marijn Haverbeke's recommendation: CM6 is imperative. React wrappers add overhead. A custom hook with `useRef`/`useEffect` gives maximum control:

```typescript
function useCodeEditor(options: CodeEditorOptions) {
  const containerRef = useRef<HTMLDivElement>(null);
  const viewRef = useRef<EditorView | null>(null);

  useEffect(() => {
    const view = new EditorView({
      state: EditorState.create({
        doc: options.initialValue,
        extensions: [
          json(),
          jsonSchema(bntoJsonSchema),
          bntoSlashCommands(),
          bntoBreadcrumbs(),
          bntoTheme(),
          // ... other extensions
        ],
      }),
      parent: containerRef.current!,
    });
    viewRef.current = view;
    return () => view.destroy();
  }, []);

  // Sync with editor store via compartment reconfiguration
  // ...

  return { containerRef, viewRef };
}
```

### Sync with Editor Store

The code editor and visual canvas share the same `useEditorStore`. When the code editor's JSON changes:

1. CM6 `updateListener` fires on every transaction
2. Parse the new JSON text
3. If valid, update `useEditorStore.definition`
4. Editor store notifies the visual canvas (if mounted)

When the visual canvas changes:

1. `useEditorStore.definition` changes
2. Code editor subscribes to store changes
3. Serialize the new Definition to formatted JSON
4. Dispatch a CM6 transaction to replace the document
5. Mark this transaction as "external" (don't re-sync back)

### Split View

The code editor and visual canvas can coexist in a split view:

```
┌──────────────────────────────────┐
│ EditorToolbar (shared)           │
├────────────────┬─────────────────┤
│ ConveyorCanvas │ CodeEditor      │
│ (visual)       │ (JSON)          │
│                │                 │
│ ┌──┐  ┌──┐    │ {               │
│ │IN│──│CS│    │   "type":"group" │
│ └──┘  └──┘    │   "nodes": [    │
│                │     ...         │
└────────────────┴─────────────────┘
```

Both are views of the same Definition. Changes in either sync through the editor store.

---

## Packages

| Package | Purpose | Bundle Impact |
|---------|---------|--------------|
| `@codemirror/state` | Core state model | ~10 KB |
| `@codemirror/view` | DOM rendering | ~40 KB |
| `@codemirror/lang-json` | JSON language (Lezer) | ~10 KB |
| `@codemirror/autocomplete` | Completion system | ~15 KB |
| `@codemirror/lint` | Diagnostic system | ~5 KB |
| `@codemirror/commands` | Standard commands | ~8 KB |
| `@codemirror/search` | Find/replace | ~5 KB |
| `codemirror-json-schema` | JSON Schema intelligence | ~15 KB |
| **Total** | | **~108 KB** (gzipped ~40 KB) |

Compare: Monaco is ~2.4 MB gzipped. CM6 with full JSON Schema support is ~40 KB gzipped. That's a **60x difference.**

---

## Theming

CM6 themes use CSS, which means direct integration with bnto's OKLCH token system:

```javascript
EditorView.theme({
  "&": {
    backgroundColor: "var(--background)",
    color: "var(--foreground)",
    fontFamily: "var(--font-mono)",
  },
  ".cm-content": {
    caretColor: "var(--primary)",
  },
  "&.cm-focused .cm-cursor": {
    borderLeftColor: "var(--primary)",
  },
  ".cm-selectionBackground": {
    backgroundColor: "var(--accent) / 0.2",
  },
  ".cm-gutters": {
    backgroundColor: "var(--muted)",
    color: "var(--muted-foreground)",
    borderRight: "1px solid var(--border)",
  },
});
```

Dark mode switches automatically — the CSS variables resolve to different values under `.dark`. No theme swapping code needed.

---

## Performance Considerations

| Concern | Mitigation |
|---------|-----------|
| **Bundle size** | CM6 is ~40 KB gzipped. Lazy load via `next/dynamic` with `ssr: false` |
| **Large recipes** | CM6 virtualizes rendering — only visible content in DOM. Handles millions of lines |
| **JSON parsing on every keystroke** | Debounce validation. Only parse when typing pauses (200ms). Lezer is incremental |
| **Sync with visual canvas** | Batch updates. Don't sync on every character — sync on transaction boundaries |
| **Multiple editors (split view)** | CM6 instances are fully independent. No global state conflicts |
| **Mobile** | CM6 has native touch support. Monaco does not |

---

## Implementation Roadmap (Summary)

See [PLAN.md](../PLAN.md) → **Sprint 4B: Code Editor (CodeMirror 6)** for the full task breakdown with per-wave task lists, persona ownership, and package tags. Use `/pickup` to claim tasks.

| Wave | Summary | Package |
|------|---------|---------|
| **Wave 1** | JSON Schema generation from `@bnto/nodes` types | `@bnto/nodes` |
| **Wave 2** | CM6 foundation — editor component, theme, hook, schema validation, autocompletion | `apps/web` |
| **Wave 3** | Slash commands + command registry — Notion-like inline node insertion | `apps/web` |
| **Wave 4** | Store sync + command palette (Cmd-K) + split view (code + visual canvas) | `apps/web` |
| **Wave 5** | Breadcrumbs, format-on-save, template expression hints, E2E tests | `apps/web` |

---

## References

| Resource | What it covers |
|----------|----------------|
| [CodeMirror 6 System Guide](https://codemirror.net/docs/guide/) | Architecture, state, transactions, extensions |
| [CodeMirror 6 Examples](https://codemirror.net/examples/) | Autocompletion, decorations, panels, tooltips, config |
| [codemirror-json-schema](https://github.com/jsonnext/codemirror-json-schema) | JSON Schema validation, autocomplete, hover for CM6 |
| [Sourcegraph: Monaco → CM6 Migration](https://sourcegraph.com/blog/migrating-monaco-codemirror) | Bundle size, mobile, theming comparison |
| [Replit: Betting on CM6](https://blog.replit.com/codemirror) | Mobile support, extensibility |
| [cmdk](https://cmdk.paco.me/) | Command palette component (React) |
| [Xi-Editor Retrospective](https://raphlinus.github.io/xi/2020/06/27/xi-retrospective.html) | Why headless core should be a library, not a protocol |
| `.claude/skills/code-editor-expert/SKILL.md` | The code editor persona — CM6 APIs, patterns, gotchas |
| `.claude/skills/reactflow-expert/SKILL.md` | The visual canvas persona — ReactFlow patterns |
