# CmdEditor — Command-Based Recipe Editor

**Last Updated:** March 2026
**Status:** Architecture defined — Phase 0

---

## What This Is

The CmdEditor is the default recipe editor — a terminal-inspired, keyboard-first interface built from two UI primitives: a **Node Tree** (read-only display) and a **Command Input** (cmdk-powered action surface). Users see their recipe as a tree, and interact entirely through commands. Like Claude.ai's chat: content fills the top, input anchored at bottom.

This replaces the ReactFlow visual canvas as the primary editor. The RF canvas is archived for a potential future "Advanced View."

---

## Why This Architecture

Every workflow tool leads with node graphs. Bnto's value is simplicity. The CmdEditor:

- **Eliminates canvas complexity** — no drag, no zoom, no edge management, no layout algorithms
- **Keyboard-first** — power users (our core audience) work faster with commands than mice
- **TUI-inspired** — lessons transfer directly to the Tauri desktop app and CLI
- **Minimal bundle** — cmdk (14.9KB gzip) vs ReactFlow (~50KB+)

---

## Two Primitives

### Node Tree

Display-only rendering of the recipe's node hierarchy. Icon + label per node, indented groups for containers. Selection via arrow keys. No drag, no click-to-edit — all mutation goes through the command input.

### Command Input (cmdk)

cmdk is already in `@bnto/ui` (14.9KB gzip, native inline mode). Commands are context-aware: what you can do depends on what's selected in the tree. Five command groups:

| Group         | Examples                                 |
| ------------- | ---------------------------------------- |
| **Global**    | Run, Undo, Redo, Export, New, Open, Help |
| **Add Node**  | Add Compress, Add Resize, Add Clean CSV  |
| **Edit**      | Delete Node, Duplicate Node              |
| **Configure** | Set Quality: 80, Set Format: webp        |
| **File**      | Save, Save As, Open, Export JSON         |

---

## Command Registry Pattern

```typescript
interface Command {
  id: string;
  label: string;
  icon?: string;
  group: string;
  keywords?: string[];
  shortcut?: string;
  disabled?: boolean;
  execute: () => void;
}
```

The `Command` type is **context-agnostic** — not `EditorCommand`. Commands carry a pre-bound `execute` closure. The editor binds editor actions; a future dashboard could bind navigation actions.

`resolveCommands(editor, state)` is a pure function that merges command providers and applies contextual filtering. Testable with plain state objects.

---

## Inline Parameter Editing

When a processing node is selected, its configurable params expand below the tree item. Controls are simple/TUI-style: `<Input>` for strings/numbers, `<Select>` for enums, `<Switch>` for booleans. Changes update the definition immediately.

---

## Keyboard Navigation Model

| Key                 | Context               | Action                  |
| ------------------- | --------------------- | ----------------------- |
| Arrow Up/Down       | Tree focused          | Navigate between nodes  |
| Home/End            | Tree focused          | Jump to first/last node |
| `/` or Cmd+K        | Anywhere              | Focus command input     |
| Escape              | Command input (empty) | Return focus to tree    |
| Tab                 | Tree                  | Move to command input   |
| Cmd+Z / Cmd+Shift+Z | Anywhere              | Undo / Redo             |
| Cmd+Enter           | Anywhere              | Run recipe              |
| Delete/Backspace    | Tree focused          | Delete selected node    |

---

## Layout

Terminal-style centered column. Full height, max-width constrained.

```
┌─────────────────────────────────┐
│         Recipe Header           │  ← minimal: logo + user
├─────────────────────────────────┤
│                                 │
│         Node Tree               │  ← scrollable, fills space
│         (read-only display)     │
│                                 │
├─────────────────────────────────┤
│         Status Bar              │  ← recipe name, dirty, undo count
├─────────────────────────────────┤
│         Command Input           │  ← anchored at bottom
│         (cmdk inline)           │
└─────────────────────────────────┘
```

---

## What's Reused

The CmdEditor shares the same foundation as the RF editor:

- **Zustand store** — same `EditorState`, same actions, same undo/redo
- **Pure actions** — `addNode`, `removeNode`, `updateParams`, `loadDefinition`, `createBlank`
- **Services/clients** — `nodeService`, `definitionService`, `executionService`, `historyService`
- **`createEditor` / `createReactEditor`** — same factory, same context
- **`@bnto/form`** — schema-driven controls for inline param editing
- **`@bnto/ui`** — Command, IconBadge, Text, animation components
- **Node icons + category variants** — `ICON_COMPONENTS`, `CATEGORY_VARIANT`
- **Execution state machine** — same `runPipeline`, same progress events

---

## What's Archived

RF-specific components move to `components/archive/rf/`:

- Canvas, CanvasShell, EditorOverlay
- CompartmentNode, IoNode, PlaceholderNode, ContainerGroupNode, AddDividerNode
- layoutNodes, useLayoutNodes, usePlaceholderNodes, useAddDividerNodes

---

## Composability: Beyond the Editor

The command palette may become bnto's primary interaction surface for power users. The architecture has seams for this:

- `Command` type is context-agnostic — works for editor, dashboard, or global nav
- `resolveCommands()` is a pure function — takes any context, returns filtered commands
- `CmdEditorShell` is a reusable layout — "scrollable content + anchored command input"
- Command groups are pluggable — registry accepts providers, not a hardcoded list

We build the editor use case now with seams for future reuse.

---

## References

| Document                                         | What It Covers                                          |
| ------------------------------------------------ | ------------------------------------------------------- |
| [editor-architecture.md](editor-architecture.md) | Shared editor layer — store, hooks, three-layer pattern |
| [PLAN-CMD-EDITOR.md](../PLAN-CMD-EDITOR.md)      | Build plan — phases, tasks, checkpoint protocol         |
