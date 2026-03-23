"use client";

import { useCallback, useMemo, useState } from "react";
import { Text, Input, SearchIcon } from "@bnto/ui";
import { useEditor } from "../../context";
import { CATEGORY_VARIANT } from "../../adapters/categoryVariant";
import type { CompartmentVariant } from "../../adapters/types";
import { buildNodeListTree } from "../../helpers/buildNodeListTree";
import type { NodeListEntry } from "../../helpers/buildNodeListTree";
import { NodeListItem } from "./NodeListItem";
import { NodeListGroup } from "./NodeListGroup";
import { NodeListPlaceholder } from "./NodeListPlaceholder";

/**
 * NodeListTab — hierarchical list of nodes currently on the canvas.
 *
 * Preserves the exact array order from the store (1:1 with the canvas).
 * Container nodes render with a dashed border wrapping their children,
 * matching the visual language of groups on the canvas.
 * Search filters recursively — matching children reveal their parent.
 */
function NodeListTab() {
  const editor = useEditor();
  const { nodes, configs, selectedNodeId, expandedContainerIds } = editor.nodes.useNodes();
  const [query, setQuery] = useState("");

  const tree = useMemo(() => buildNodeListTree(nodes, configs), [nodes, configs]);

  /** Recursively filter entries — keep parent if any child matches. */
  const filterEntries = useCallback((entries: NodeListEntry[], q: string): NodeListEntry[] => {
    return entries.reduce<NodeListEntry[]>((acc, entry) => {
      const label = entry.config.displayName ?? entry.config.name;
      const selfMatches = label.toLowerCase().includes(q);
      const filteredChildren = filterEntries(entry.children, q);

      if (selfMatches || filteredChildren.length > 0) {
        acc.push({
          ...entry,
          children: selfMatches ? entry.children : filteredChildren,
        });
      }
      return acc;
    }, []);
  }, []);

  const { entries, hasResults } = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) {
      return { entries: tree.entries, hasResults: tree.entries.length > 0 };
    }
    const filtered = filterEntries(tree.entries, q);
    return { entries: filtered, hasResults: filtered.length > 0 };
  }, [tree, query, filterEntries]);

  const isSearching = query.trim().length > 0;

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

  const noNodes = nodes.length === 0;

  return (
    <div className="flex h-full flex-col">
      <div className="-mr-4 min-h-0 flex-1 overflow-y-auto pr-4">
        {noNodes ? (
          <Text size="xs" color="muted" className="px-4 py-6 text-center">
            No nodes yet. Add one from the Palette tab.
          </Text>
        ) : !hasResults ? (
          <Text size="xs" color="muted" className="px-4 py-6 text-center">
            No matching nodes.
          </Text>
        ) : (
          <div className="flex flex-col gap-0.5 p-2">
            {entries.map((entry, i) => {
              const showPlaceholder = !isSearching && tree.placeholderIndex === i;
              return (
                <div key={entry.node.id} className="flex flex-col gap-0.5">
                  {showPlaceholder && <NodeListPlaceholder />}
                  {entry.isContainer ? (
                    <NodeListGroup
                      entry={entry}
                      selectedNodeId={selectedNodeId}
                      expandedContainerIds={expandedContainerIds}
                      onSelect={handleSelect}
                      onRemove={handleRemove}
                    />
                  ) : (
                    <NodeListItem
                      nodeId={entry.node.id}
                      label={entry.config.displayName ?? entry.config.name}
                      icon={entry.node.data.icon}
                      variant={
                        entry.node.data.variant ??
                        (CATEGORY_VARIANT["muted"] as CompartmentVariant) ??
                        "muted"
                      }
                      selected={selectedNodeId === entry.node.id}
                      isIoNode={entry.node.data.isIoNode ?? false}
                      onSelect={handleSelect}
                      onRemove={handleRemove}
                    />
                  )}
                </div>
              );
            })}
            {!isSearching && tree.placeholderIndex === entries.length && <NodeListPlaceholder />}
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
