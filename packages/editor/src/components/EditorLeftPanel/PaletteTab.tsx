"use client";

import { useCallback, useMemo, useState } from "react";
import { Accordion, Input, SearchIcon } from "@bnto/ui";
import { useEditor } from "../../context";
import { useNodePalette } from "../../hooks/useNodePalette";
import { SLOTS } from "../../adapters/bentoSlots";
import type { NodeTypeName } from "@bnto/core";
import { PaletteCategoryGroup } from "./PaletteCategoryGroup";

/**
 * PaletteTab — available node types grouped by category.
 *
 * Sticky search input filters items by label/description.
 * Each category is a collapsible accordion section. Clicking an item
 * adds the node to the canvas. All categories start expanded.
 */
function PaletteTab() {
  const editor = useEditor();
  const { groups } = useNodePalette();
  const { nodes } = editor.nodes.useNodes();
  const isFull = nodes.length >= SLOTS.length;
  const [query, setQuery] = useState("");

  const filteredGroups = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return groups;
    return groups
      .map((group) => ({
        ...group,
        items: group.items.filter(
          (item) =>
            item.label.toLowerCase().includes(q) || item.description.toLowerCase().includes(q),
        ),
      }))
      .filter((group) => group.items.length > 0);
  }, [groups, query]);

  const defaultExpanded = groups.map((g) => g.category.name);

  const handleAdd = useCallback(
    (type: string) => {
      editor.nodes.addNode(type as NodeTypeName);
    },
    [editor],
  );

  return (
    <div className="flex h-full flex-col">
      <Accordion
        type="multiple"
        defaultValue={defaultExpanded}
        className="-mr-4 min-h-0 flex-1 overflow-x-hidden overflow-y-auto px-2 pr-4"
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
      <div className="shrink-0 border-t border-border px-2 pt-3">
        <div className="relative">
          <SearchIcon className="pointer-events-none absolute left-2.5 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search palette…"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="h-8 pl-8 text-sm"
          />
        </div>
      </div>
    </div>
  );
}

export { PaletteTab };
