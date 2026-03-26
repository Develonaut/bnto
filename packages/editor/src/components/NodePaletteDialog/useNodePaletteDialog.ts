"use client";

import { useCallback, useMemo, useState } from "react";
import type { NodeTypeName } from "@bnto/core";
import { useEditor } from "../../context";
import { useNodePalette, type PaletteGroup } from "../../hooks/useNodePalette";
import { SLOTS } from "../../adapters/bentoSlots";

/** State + handlers for the node palette dialog. */
function useNodePaletteDialog(onOpenChange: (open: boolean) => void) {
  const [search, setSearch] = useState("");
  const editor = useEditor();
  const { groups } = useNodePalette();
  const { nodes, insertAfterNodeId, insertIntoContainerId } = editor.nodes.useNodes();
  const isFull = nodes.length >= SLOTS.length;

  const filteredGroups = useMemo(() => filterGroups(groups, search), [groups, search]);
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

  return { search, setSearch, isFull, filteredGroups, defaultExpanded, handleClose, handleAdd };
}

function filterGroups(groups: PaletteGroup[], search: string) {
  const term = search.trim().toLowerCase();
  if (!term) return groups;
  return groups
    .map((g) => ({ ...g, items: g.items.filter((item) => matchesSearch(item, term)) }))
    .filter((g) => g.items.length > 0);
}

function matchesSearch(
  item: { type: string; label: string; description: string },
  term: string,
): boolean {
  return (
    item.type.toLowerCase().includes(term) ||
    item.label.toLowerCase().includes(term) ||
    item.description.toLowerCase().includes(term)
  );
}

export { useNodePaletteDialog };
