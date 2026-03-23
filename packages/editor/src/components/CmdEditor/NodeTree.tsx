"use client";

import { useMemo } from "react";
import { Text, PlusIcon } from "@bnto/ui";
import { useEditor } from "../../context";
import { CATEGORY_VARIANT } from "../../adapters/categoryVariant";
import type { CompartmentVariant } from "../../adapters/types";
import { buildNodeListTree } from "../../helpers/buildNodeListTree";
import { NodeTreeItem } from "./NodeTreeItem";
import { NodeTreeGroup } from "./NodeTreeGroup";

/**
 * NodeTree — read-only tree rendering of the recipe node hierarchy.
 *
 * Uses buildNodeListTree for proper parent/child nesting. Container
 * nodes render with dashed borders wrapping their children.
 * Selection will be driven by keyboard navigation (Phase 2).
 * All mutation goes through the command palette.
 */
function NodeTree() {
  const editor = useEditor();
  const { nodes, configs, selectedNodeId, expandedContainerIds } = editor.nodes.useNodes();

  const tree = useMemo(() => buildNodeListTree(nodes, configs), [nodes, configs]);

  if (tree.entries.length === 0) {
    return (
      <div className="flex h-full items-center justify-center">
        <Text color="muted" size="sm">
          No nodes yet. Use the command input to add one.
        </Text>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-0.5" role="tree" aria-label="Recipe nodes">
      {tree.entries.map((entry, i) => {
        const showPlaceholder = tree.placeholderIndex === i;
        return (
          <div key={entry.node.id} className="flex flex-col gap-0.5">
            {showPlaceholder && <NodeTreePlaceholder />}
            {entry.isContainer ? (
              <NodeTreeGroup
                entry={entry}
                selectedNodeId={selectedNodeId}
                expandedContainerIds={expandedContainerIds}
              />
            ) : (
              <NodeTreeItem
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
              />
            )}
          </div>
        );
      })}
      {tree.placeholderIndex === tree.entries.length && <NodeTreePlaceholder />}
    </div>
  );
}

/** Dashed-border placeholder shown when no processing nodes exist. */
function NodeTreePlaceholder() {
  return (
    <div className="flex items-center gap-2 rounded-md px-3 py-3 outline outline-2 outline-dashed outline-border -outline-offset-2">
      <PlusIcon className="size-4 shrink-0 text-muted-foreground" />
      <Text size="xs" color="muted">
        Add a node from the command input
      </Text>
    </div>
  );
}

export { NodeTree };
