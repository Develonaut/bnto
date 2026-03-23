"use client";

import { useCallback, useMemo, useState } from "react";
import { Text, Input, SearchIcon } from "@bnto/ui";
import { useEditor } from "../../context";
import { CATEGORY_VARIANT } from "../../adapters/categoryVariant";
import type { CompartmentVariant } from "../../adapters/types";
import { NodeListItem } from "./NodeListItem";

/**
 * NodeListTab — vertical list of nodes currently on the canvas.
 *
 * Reads from the editor store (not the display pipeline) so synthetic
 * nodes (placeholder, containerGroup, addDivider) are excluded.
 * Click selects the node on the canvas + opens config panel.
 * Search input at the bottom filters by node name.
 */
function NodeListTab() {
  const editor = useEditor();
  const { nodes, configs, selectedNodeId } = editor.nodes.useNodes();
  const [query, setQuery] = useState("");

  const filteredNodes = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return nodes;
    return nodes.filter((node) => {
      const config = configs[node.id];
      if (!config) return false;
      const label = config.displayName ?? config.name;
      return label.toLowerCase().includes(q);
    });
  }, [nodes, configs, query]);

  const handleSelect = useCallback(
    (id: string) => {
      editor.nodes.selectNode(id);
    },
    [editor],
  );

  const handleRemove = useCallback(
    (id: string) => {
      editor.nodes.removeNode(id);
    },
    [editor],
  );

  return (
    <div className="flex h-full flex-col">
      <div className="-mr-4 min-h-0 flex-1 overflow-y-auto pr-4">
        {filteredNodes.length === 0 && nodes.length === 0 ? (
          <Text size="xs" color="muted" className="px-4 py-6 text-center">
            No nodes yet. Add one from the Palette tab.
          </Text>
        ) : filteredNodes.length === 0 ? (
          <Text size="xs" color="muted" className="px-4 py-6 text-center">
            No matching nodes.
          </Text>
        ) : (
          <div className="flex flex-col gap-0.5 p-2">
            {filteredNodes.map((node) => {
              const config = configs[node.id];
              if (!config) return null;
              const variant: CompartmentVariant =
                node.data.variant ?? (CATEGORY_VARIANT["muted"] as CompartmentVariant) ?? "muted";
              return (
                <NodeListItem
                  key={node.id}
                  nodeId={node.id}
                  label={config.displayName ?? config.name}
                  icon={node.data.icon}
                  variant={variant}
                  selected={selectedNodeId === node.id}
                  isIoNode={node.data.isIoNode ?? false}
                  onSelect={handleSelect}
                  onRemove={handleRemove}
                />
              );
            })}
          </div>
        )}
      </div>
      <div className="shrink-0 border-t border-border px-2 pt-3">
        <div className="relative">
          <SearchIcon className="pointer-events-none absolute left-2.5 top-1/2 size-3.5 -translate-y-1/2 text-muted-foreground" />
          <Input
            placeholder="Search nodes…"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            className="h-8 pl-8 text-sm"
          />
        </div>
      </div>
    </div>
  );
}

export { NodeListTab };
