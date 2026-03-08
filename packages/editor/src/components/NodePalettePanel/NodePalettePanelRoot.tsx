"use client";

import { Fragment, useCallback, useMemo, useState } from "react";
import type { NodeTypeName } from "@bnto/nodes";
import {
  Badge,
  BlocksIcon,
  Divider,
  MenuLabel,
  MenuItem,
  MenuSeparator,
  SearchIcon,
  Text,
} from "@bnto/ui";
import { useEditorStore } from "../../hooks/useEditorStore";
import { useEditorActions } from "../../hooks/useEditorActions";
import { useNodePalette } from "../../hooks/useNodePalette";
import { SLOTS } from "../../adapters/bentoSlots";
import { EditorMenuPanel } from "../EditorMenuPanel";

/**
 * NodePalettePanel — Menu-based node palette panel.
 *
 * Opens to the right from the left toolbar trigger. Shows a searchable
 * categorized grid of node types. Clicking a node adds it to the canvas.
 * Toggled via toolbar button. Same-side siblings close automatically.
 */

function NodePalettePanelRoot() {
  const [search, setSearch] = useState("");
  const { addNode } = useEditorActions();
  const { groups } = useNodePalette();
  const nodeCount = useEditorStore((s) => s.nodes.length);
  const isFull = nodeCount >= SLOTS.length;

  const filteredGroups = useMemo(() => {
    const term = search.trim().toLowerCase();
    if (!term) return groups;
    return groups
      .map((group) => ({
        ...group,
        items: group.items.filter(
          (item) =>
            item.label.toLowerCase().includes(term) ||
            item.description.toLowerCase().includes(term) ||
            item.name.toLowerCase().includes(term),
        ),
      }))
      .filter((group) => group.items.length > 0);
  }, [groups, search]);

  const handleAdd = useCallback(
    (typeName: NodeTypeName) => {
      addNode(typeName);
    },
    [addNode],
  );

  const hasResults = filteredGroups.length > 0;

  return (
    <EditorMenuPanel
      panelId="palette"
      side="right"
      width="w-[28rem]"
      label="Add node"
      icon={<BlocksIcon className="size-4" />}
    >
      <div className="shrink-0 px-3 pt-3 pb-0">
        <div className="flex items-center gap-2 rounded-md bg-input px-2.5 py-1.5">
          <SearchIcon className="size-3.5 shrink-0 text-muted-foreground" />
          <input
            type="text"
            placeholder="Search nodes…"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full bg-transparent text-sm outline-none placeholder:text-muted-foreground"
            autoFocus
          />
        </div>
      </div>

      {isFull && (
        <Text size="xs" color="muted" className="px-3 py-2">
          Canvas is full ({SLOTS.length} nodes max).
        </Text>
      )}

      <Divider />

      <div className="min-h-0 flex-1 overflow-y-auto p-3">
        <div className="grid grid-cols-2 gap-1">
          {filteredGroups.map((group, i) => (
            <Fragment key={group.category.name}>
              {i > 0 && <MenuSeparator className="col-span-2" />}
              <MenuLabel className="col-span-2">{group.category.label}</MenuLabel>
              {group.items.map((item) => {
                const isServerOnly = !item.browserCapable;
                const disabled = isFull || isServerOnly;
                return (
                  <MenuItem
                    key={item.name}
                    onClick={() => handleAdd(item.name as NodeTypeName)}
                    disabled={disabled}
                    className="flex-col items-start gap-1 py-2.5 text-left"
                  >
                    <span className="flex w-full items-center gap-2">
                      <span className="text-sm leading-normal font-medium">{item.label}</span>
                      {isServerOnly && (
                        <Badge variant="outline" className="shrink-0 text-xs">
                          Pro
                        </Badge>
                      )}
                    </span>
                    <span className="text-xs leading-normal text-muted-foreground text-wrap">
                      {item.description}
                    </span>
                  </MenuItem>
                );
              })}
            </Fragment>
          ))}

          {!hasResults && search.trim() && (
            <Text size="xs" color="muted" className="col-span-2 px-3 py-4 text-center">
              No nodes match &ldquo;{search}&rdquo;
            </Text>
          )}
        </div>
      </div>
    </EditorMenuPanel>
  );
}

export { NodePalettePanelRoot };
