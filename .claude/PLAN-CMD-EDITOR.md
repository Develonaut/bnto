# CmdEditor — Build Plan

**Last Updated:** March 23, 2026
**Branch:** `feat/cmd-editor`
**Strategy:** [cmd-editor.md](.claude/strategy/cmd-editor.md)

---

## Status Key

```
TODO   → available
DOING  → agent is working on this
DONE   → complete
```

---

## Phase 0: Prep — Docs, Branch, PR

### Status: DONE

- [DONE] 0.1 — Write CmdEditor strategy doc (`.claude/strategy/cmd-editor.md`)
- [DONE] 0.2 — Update editor-architecture.md (add CmdEditor as third/default mode)
- [DONE] 0.3 — Create pickup-cmd-editor skill
- [DONE] 0.4 — Create branch + draft PR

---

## Phase 1: Archive RF + Shell

### Status: DONE

Archive RF-specific components. Stand up the CmdEditor shell with a static tree and stub command input.

- [DONE] 1.1 — Archive RF editor components to `components/archive/rf/`, `hooks/archive/rf/`, `adapters/archive/`
- [DONE] 1.2 — CmdEditorShell layout (full-height centered column, content + command slots)
- [DONE] 1.3 — NodeTree + NodeTreeItem + NodeTreeGroup components
- [DONE] 1.4 — Static command input stub (cmdk shell, no logic)
- [DONE] 1.5 — Wire into editor page (replace EditorCanvas, update imports)
- [DONE] 1.6 — Code review gate (x3)

---

## Phase 2: Keyboard Navigation

### Status: TODO

Arrow keys navigate the tree. Focus flows between tree and command input.

- [TODO] 2.1 — Arrow key navigation in tree (`useTreeKeyboard` hook)
- [TODO] 2.2 — Focus management between tree and command (`useFocusZones` hook)
- [TODO] 2.3 — Code review gate (x3)

---

## Phase 3: Global Commands

### Status: TODO

Wire cmdk with real commands. The command input becomes functional.

- [TODO] 3.1 — Command registry types + global commands
- [TODO] 3.2 — Wire commands into CmdInput (rewrite with CommandList, CommandGroup, CommandItem)
- [TODO] 3.3 — Editor keyboard shortcuts (non-RF, `useCmdEditorShortcuts`)
- [TODO] 3.4 — Code review gate (x3)

---

## Phase 4: Node CRUD Commands

### Status: TODO

Add, remove, reorder nodes via the command palette.

- [TODO] 4.1 — Add-node commands (one per node type)
- [TODO] 4.2 — Context-aware command resolver (`resolveCommands`)
- [TODO] 4.3 — Set insertion context from tree selection
- [TODO] 4.4 — Tests for command resolution
- [TODO] 4.5 — Code review gate (x3)

---

## Phase 5: Inline Parameter Editing

### Status: TODO

Select a node → its configurable params appear below it in the tree.

- [TODO] 5.1 — Param commands for selected node
- [TODO] 5.2 — Inline param editor below tree items
- [TODO] 5.3 — Keyboard flow for inline editing
- [TODO] 5.4 — Code review gate (x3)

---

## Phase 6: Execution

### Status: TODO

Drop files, run the recipe, see results. Full execution loop.

- [TODO] 6.1 — File input zone
- [TODO] 6.2 — Execution status on tree items
- [TODO] 6.3 — Results section
- [TODO] 6.4 — Code review gate (x3)

---

## Phase 7: Polish + Ship

### Status: TODO

Status bar, save/load, help, E2E tests, docs.

- [TODO] 7.1 — Status bar
- [TODO] 7.2 — File operations (save, open, export commands)
- [TODO] 7.3 — Onboarding hint
- [TODO] 7.4 — E2E tests
- [TODO] 7.5 — Update strategy docs
- [TODO] 7.6 — Code review gate (x3)

---

## Phase 8: Cleanup + Merge

### Status: TODO

- [TODO] 8.1 — Merge planning docs into official strategy
- [TODO] 8.2 — Remove temporary pickup skill
- [TODO] 8.3 — Final code review gate (x3) + merge prep

---

## Phase Dependencies

```
Phase 0 (docs)
    │
Phase 1 (archive RF + shell + tree + stub command)
    │
    ├── Phase 2 (keyboard nav)      ← can parallel with Phase 3
    ├── Phase 3 (global commands)    ← can parallel with Phase 2
    │       │
    │   Phase 4 (node CRUD)
    │       │
    │   Phase 5 (param editing)
    │
    └── Phase 6 (execution)          ← depends on Phase 3
            │
        Phase 7 (polish + E2E)
            │
        Phase 8 (cleanup + merge)
```

## Checkpoint Protocol

After each phase, before committing:

1. `task ui:build` — must pass
2. `task ui:test` — must pass
3. `/code-review` three times — fix all issues
4. Manually load `/editor?recipe=compress-images` — verify visual state (Phase 1+)
5. Commit with descriptive message
