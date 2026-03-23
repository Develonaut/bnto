"use client";

import { Surface } from "@bnto/ui";
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
  const variant = node.data.variant ?? "muted";

  const isExpanded = expandedContainerIds.has(node.id);
  const showChildren = isExpanded && children.length > 0;

  return (
    <Surface variant="muted" elevation="none" dashed className="p-3 pl-4 pt-4">
      <NodeTreeItem
        nodeId={node.id}
        label={config.displayName ?? config.name}
        icon={node.data.icon}
        variant={variant}
        selected={selectedNodeId === node.id}
        isIoNode={false}
      />
      {showChildren && (
        <div className="ml-3 flex flex-col gap-2 pt-2" role="group">
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
                variant={child.node.data.variant ?? "muted"}
                selected={selectedNodeId === child.node.id}
                isIoNode={false}
              />
            ),
          )}
        </div>
      )}
    </Surface>
  );
}

export { NodeTreeGroup };
