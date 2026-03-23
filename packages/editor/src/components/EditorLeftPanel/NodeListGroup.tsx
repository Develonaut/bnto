"use client";

import type { CompartmentVariant } from "../../adapters/types";
import { CATEGORY_VARIANT } from "../../adapters/categoryVariant";
import { NodeListItem } from "./NodeListItem";
import type { NodeListEntry } from "../../helpers/buildNodeListTree";

interface NodeListGroupProps {
  entry: NodeListEntry;
  selectedNodeId: string | null;
  expandedContainerIds: Set<string>;
  onSelect: (id: string) => void;
  onRemove: (id: string) => void;
}

/**
 * NodeListGroup — container node wrapped in a dashed border,
 * matching the group visual on the canvas.
 *
 * The container header renders as a normal NodeListItem (left-aligned
 * with all other top-level items). Children indent below it inside
 * the same dashed wrapper.
 */
function NodeListGroup({
  entry,
  selectedNodeId,
  expandedContainerIds,
  onSelect,
  onRemove,
}: NodeListGroupProps) {
  const { node, config, children } = entry;
  const variant: CompartmentVariant =
    node.data.variant ?? (CATEGORY_VARIANT["muted"] as CompartmentVariant) ?? "muted";

  const isCanvasExpanded = expandedContainerIds.has(node.id);
  const showChildren = isCanvasExpanded && children.length > 0;

  return (
    <div className="rounded-lg p-1 outline outline-2 outline-dashed outline-border -outline-offset-2">
      <NodeListItem
        nodeId={node.id}
        label={config.displayName ?? config.name}
        icon={node.data.icon}
        variant={variant}
        selected={selectedNodeId === node.id}
        isIoNode={false}
        onSelect={onSelect}
        onRemove={onRemove}
      />
      {showChildren && (
        <div className="ml-4 flex flex-col gap-0.5 pt-0.5">
          {children.map((child) =>
            child.isContainer ? (
              <NodeListGroup
                key={child.node.id}
                entry={child}
                selectedNodeId={selectedNodeId}
                expandedContainerIds={expandedContainerIds}
                onSelect={onSelect}
                onRemove={onRemove}
              />
            ) : (
              <NodeListItem
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
                onSelect={onSelect}
                onRemove={onRemove}
              />
            ),
          )}
        </div>
      )}
    </div>
  );
}

export { NodeListGroup };
