"use client";

import { useCallback, useMemo, useState } from "react";
import { Accordion, DialogShell, Input, SearchIcon, Text } from "@bnto/ui";
import type { NodeTypeName } from "@bnto/core";
import { useEditor } from "../../context";
import { useNodePalette } from "../../hooks/useNodePalette";
import { SLOTS } from "../../adapters/bentoSlots";
import { PaletteCategoryGroup } from "./PaletteCategoryGroup";

/**
 * NodePaletteDialog — dialog-based node palette.
 *
 * Shows a searchable categorized list of palette items with icons.
 * Each category is a collapsible accordion section. Clicking an item
 * adds the node to the canvas and closes the dialog.
 */

interface NodePaletteDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function NodePaletteDialogRoot({ open, onOpenChange }: NodePaletteDialogProps) {
  const [search, setSearch] = useState("");
  const editor = useEditor();
  const { groups } = useNodePalette();
  const { nodes, insertAfterNodeId, insertIntoContainerId } = editor.nodes.useNodes();
  const isFull = nodes.length >= SLOTS.length;

  const filteredGroups = useMemo(() => {
    const term = search.trim().toLowerCase();
    if (!term) return groups;
    return groups
      .map((group) => ({
        ...group,
        items: group.items.filter(
          (item) =>
            item.type.toLowerCase().includes(term) ||
            item.label.toLowerCase().includes(term) ||
            item.description.toLowerCase().includes(term),
        ),
      }))
      .filter((group) => group.items.length > 0);
  }, [groups, search]);

  const defaultExpanded = useMemo(() => groups.map((g) => g.category.name), [groups]);

  const handleClose = useCallback(
    (nextOpen: boolean) => {
      if (!nextOpen) {
        editor.nodes.setInsertAfterNodeId(null);
        editor.nodes.setInsertIntoContainerId(null);
      }
      onOpenChange(nextOpen);
    },
    [editor, onOpenChange],
  );

  const handleAdd = useCallback(
    (type: string) => {
      editor.nodes.addNode(type as NodeTypeName, insertAfterNodeId, insertIntoContainerId);
      handleClose(false);
    },
    [editor, insertAfterNodeId, insertIntoContainerId, handleClose],
  );

  const hasResults = filteredGroups.length > 0;

  return (
    <DialogShell
      open={open}
      onOpenChange={handleClose}
      title="Add Node"
      description="Search and select a node type to add to the canvas."
      size="lg"
      contentProps={{ "data-testid": "node-palette-dialog" }}
      headerClassName="pb-2"
    >
      <div className="flex h-[28rem] flex-col">
        {/* Search */}
        <div className="shrink-0 pb-3">
          <div className="relative">
            <SearchIcon className="pointer-events-none absolute left-2.5 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground" />
            <Input
              placeholder="Search nodes..."
              aria-label="Search node types"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="h-8 pl-8 text-sm"
              autoFocus
            />
          </div>
        </div>

        {isFull && (
          <Text size="xs" color="muted" className="pb-2">
            Canvas is full ({SLOTS.length} nodes max).
          </Text>
        )}

        {/* Node list — Accordion sections per category */}
        <Accordion
          type="multiple"
          defaultValue={defaultExpanded}
          className="-mr-8 min-h-0 flex-1 overflow-x-hidden overflow-y-auto border-t border-border pr-8 pt-3"
        >
          {filteredGroups.map((group) => (
            <PaletteCategoryGroup
              key={group.category.name}
              group={group}
              isFull={isFull}
              onAdd={handleAdd}
            />
          ))}
        </Accordion>

        {!hasResults && search.trim() && (
          <Text size="xs" color="muted" className="px-3 py-4 text-center">
            No nodes match &ldquo;{search}&rdquo;
          </Text>
        )}
      </div>
    </DialogShell>
  );
}

export { NodePaletteDialogRoot };
