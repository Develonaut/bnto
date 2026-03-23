"use client";

import type { CompartmentVariant } from "../../adapters/types";
import { CATEGORY_VARIANT } from "../../adapters/categoryVariant";
import { NodeTreeItem } from "./NodeTreeItem";
import type { NodeListEntry } from "../../helpers/buildNodeListTree";

interface NodeTreeGroupProps {
  entry: NodeListEntry;
  selectedNodeId: string | null;
  expandedContainerIds: Set<string>;
}

/**
 * NodeTreeGroup — container node wrapped in a dashed border,
 * matching the group visual language from the canvas.
 *
 * Children indent below the container header inside the same
 * dashed wrapper.
 */
function NodeTreeGroup({ entry, selectedNodeId, expandedContainerIds }: NodeTreeGroupProps) {
  const { node, config, children } = entry;
  const variant: CompartmentVariant =
    node.data.variant ?? (CATEGORY_VARIANT["muted"] as CompartmentVariant) ?? "muted";

  const isExpanded = expandedContainerIds.has(node.id);
  const showChildren = isExpanded && children.length > 0;

  return (
    <div className="rounded-lg p-1 outline outline-2 outline-dashed outline-border -outline-offset-2">
      <NodeTreeItem
        nodeId={node.id}
        label={config.displayName ?? config.name}
        icon={node.data.icon}
        variant={variant}
        selected={selectedNodeId === node.id}
        isIoNode={false}
      />
      {showChildren && (
        <div className="ml-4 flex flex-col gap-0.5 pt-0.5">
          {children.map((child) =>
            child.isContainer ? (
              <NodeTreeGroup
                key={child.node.id}
                entry={child}
                selectedNodeId={selectedNodeId}
                expandedContainerIds={expandedContainerIds}
              />
            ) : (
              <NodeTreeItem
                key={child.node.id}
                nodeId={child.node.id}
                label={child.config.displayName ?? child.config.name}
                icon={child.node.data.icon}
                variant={
                  child.node.data.variant ??
                  (CATEGORY_VARIANT["muted"] as CompartmentVariant) ??
                  "muted"
                }
                selected={selectedNodeId === child.node.id}
                isIoNode={false}
              />
            ),
          )}
        </div>
      )}
    </div>
  );
}

export { NodeTreeGroup };
