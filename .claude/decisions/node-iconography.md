# Node Iconography System

**Status:** Decision made — ready to implement  
**Sprint:** 5D  
**Date:** 2026-03-06  
**Depends on:** Sprint 5A Wave 2 (config panel wiring)

---

## Problem

The current `icon` field in `NODE_TYPE_INFO` is flat and static. Input always shows `square-arrow-right` and Output always shows `square-arrow-right-exit` regardless of what either node is configured to do. These are directional arrows that describe data flow direction, not data shape or intent. The `"Io"` sublabel visible on nodes exposes internal category metadata directly to users.

**Root cause:** Nothing reads the node's `params` to derive the icon. Every node resolves its icon identically, making I/O nodes visually anonymous.

---

## Decision

### 1. Add `getNodeIcon(nodeType, params?)` to `@bnto/nodes`

A pure utility function — no React dependency, no runtime imports. Returns a Lucide icon name string. Processing nodes delegate to the static `icon` field in `NODE_TYPE_INFO`. I/O nodes resolve from params.

```ts
// packages/@bnto/nodes/src/getNodeIcon.ts

import type { NodeTypeName } from "./nodeTypes";
import type { InputParams } from "./schemas/input";
import type { OutputParams } from "./schemas/output";
import { NODE_TYPE_INFO } from "./nodeTypes";

const INPUT_MODE_ICONS: Record<InputParams["mode"], string> = {
  "file-upload": "file-up",
  text: "text-cursor-input",
  url: "link",
};

const OUTPUT_MODE_ICONS: Record<OutputParams["mode"], string> = {
  download: "download",
  display: "monitor",
  preview: "eye",
};

export function getNodeIcon(nodeType: NodeTypeName, params?: Record<string, unknown>): string {
  if (nodeType === "input" && params?.mode) {
    return INPUT_MODE_ICONS[params.mode as InputParams["mode"]] ?? "file-up";
  }
  if (nodeType === "output" && params?.mode) {
    return OUTPUT_MODE_ICONS[params.mode as OutputParams["mode"]] ?? "download";
  }
  return NODE_TYPE_INFO[nodeType].icon;
}
```

**Why this lives in `@bnto/nodes` and not in the editor:** The icon is metadata about a node type, not a UI concern. The editor consuming `getNodeIcon(nodeType, params)` is correct layering — the node definition owns what icon represents a file-upload input.

### 2. Update `NODE_TYPE_INFO` static icons

Several processing node icons are weak. These should be updated in `nodeTypes.ts`:

| Node            | Current           | Updated            | Reason                                          |
| --------------- | ----------------- | ------------------ | ----------------------------------------------- |
| `image`         | `image`           | `image`            | ✅ keep                                         |
| `spreadsheet`   | `table`           | `sheet`            | `sheet` reads as spreadsheet, more specific     |
| `transform`     | `shuffle`         | `arrow-left-right` | Shuffle implies randomness; transform = mapping |
| `edit-fields`   | `pen-line`        | `pen-line`         | ✅ keep                                         |
| `loop`          | `refresh-cw`      | `repeat`           | `refresh` = reload, `repeat` = iteration        |
| `parallel`      | `columns-3`       | `git-fork`         | Fork conveys branching/concurrent paths         |
| `group`         | `braces`          | `box`              | Braces feel code-y; box = container             |
| `http-request`  | `globe`           | `globe`            | ✅ keep                                         |
| `file-system`   | `folder-open`     | `folder-open`      | ✅ keep                                         |
| `shell-command` | `terminal-square` | `terminal`         | Cleaner at small sizes                          |

### 3. Fix the "Io" sublabel

Replace raw category name with a human-readable mode label or category label:

```ts
// packages/editor/src/utils/getNodeSubLabel.ts

export function getNodeSubLabel(nodeType: string, params?: Record<string, unknown>): string {
  if (nodeType === "input") {
    const modeLabels: Record<string, string> = {
      "file-upload": "File Upload",
      text: "Text Input",
      url: "URL",
    };
    return modeLabels[params?.mode as string] ?? "Input";
  }
  if (nodeType === "output") {
    const modeLabels: Record<string, string> = {
      download: "Download",
      display: "Display",
      preview: "Preview",
    };
    return modeLabels[params?.mode as string] ?? "Output";
  }
  return getCategoryInfo(nodeType)?.label ?? nodeType;
}
```

### 4. Wire into `CompartmentNode`

- Replace static `NODE_TYPE_INFO[nodeType].icon` lookup with `getNodeIcon(nodeType, params)`
- Replace `"Io"` sublabel with `getNodeSubLabel(nodeType, params)`
- Result: changing Mode in the config panel immediately updates the node icon (live)

---

## Icon Map — I/O Nodes (config-aware)

| Node   | Mode          | Lucide Icon         | Rationale                          |
| ------ | ------------- | ------------------- | ---------------------------------- |
| Input  | `file-upload` | `file-up`           | File moving upward into the system |
| Input  | `text`        | `text-cursor-input` | Text entry metaphor                |
| Input  | `url`         | `link`              | URL = link                         |
| Output | `download`    | `download`          | Files coming down to user          |
| Output | `display`     | `monitor`           | Shown on screen                    |
| Output | `preview`     | `eye`               | Look at it                         |

---

## Icon Library Decision

**Lucide only.** All icons mapped above exist in Lucide. No second library needed for the current 12 node types.

Future consideration: if a dedicated PDF node, Audio node, or Database node is added, **Phosphor Icons** would be the addition to reach for (it has `FilePdf`, `FileAudio`, `Database` variants with multiple weight options). That's not a today problem.

---

## Alternatives Considered

**Option A: Add `getIcon(params?)` method to each node schema definition**
Rejected — would require React awareness in `@bnto/nodes` or awkward abstraction. Static icon strings in the schema registry is the correct level; the resolver function sits just above it.

**Option B: Config-aware icons on processing nodes too (e.g., image node shows JPEG icon based on output format)**
Deferred — not a priority for the current 12 nodes. Can be layered in later via the same `getNodeIcon` resolver without breaking changes.

**Option C: Custom SVG icons per node**
Rejected — unnecessary complexity. Lucide covers all current needs. Custom SVGs would require design work and asset management.

---

## Sprint 5D — Implementation Waves

### Wave 1 — `getNodeIcon` in `@bnto/nodes`

- [ ] Add `getNodeIcon.ts` with the function above
- [ ] Export from `index.ts`
- [ ] Unit tests: each input mode, each output mode, all 10 processing nodes, unknown params fallback
- [ ] Update `NODE_TYPE_INFO` static icons per the table above

### Wave 2 — Wire into `CompartmentNode`

- [ ] Replace static icon lookup with `getNodeIcon(nodeType, params)`
- [ ] Replace `"Io"` sublabel with `getNodeSubLabel(nodeType, params)`
- [ ] Verify live update: changing Mode in config panel immediately changes node icon

### Wave 3 — Tests + screenshot

- [ ] E2E: change Input Mode `file-upload` → `text` → `url`, assert icon updates each time
- [ ] Screenshot regression update

---

## Files Changed

| File                                                 | Change                                  |
| ---------------------------------------------------- | --------------------------------------- |
| `packages/@bnto/nodes/src/getNodeIcon.ts`            | NEW                                     |
| `packages/@bnto/nodes/src/getNodeIcon.test.ts`       | NEW                                     |
| `packages/@bnto/nodes/src/nodeTypes.ts`              | Update static `icon` values for 5 nodes |
| `packages/@bnto/nodes/src/index.ts`                  | Export `getNodeIcon`                    |
| `packages/editor/src/utils/getNodeSubLabel.ts`       | NEW                                     |
| `packages/editor/src/components/CompartmentNode.tsx` | Wire `getNodeIcon` + `getNodeSubLabel`  |

---

## Permanent Rules

- `getNodeIcon` is the single source of truth for what icon a node shows. Never compute the icon directly in a component.
- I/O node icons are always derived from params. Never hardcode `file-up` or `download` in a component.
- Adding a new input mode or output mode requires: (1) updating `INPUT_MODES`/`OUTPUT_MODES` in the schema, (2) adding the entry to `INPUT_MODE_ICONS`/`OUTPUT_MODE_ICONS` in `getNodeIcon.ts`, (3) adding the label to `getNodeSubLabel.ts`.
