"use client";

import { Text, IconBadge } from "@bnto/ui";
import { ICON_COMPONENTS } from "../../adapters/nodeIcons";
import type { CompartmentVariant } from "../../adapters/types";

/**
 * NodeTreeItem — single row in the CmdEditor node tree.
 *
 * Displays variant-colored IconBadge + label. Selection is driven
 * by keyboard navigation (Phase 2). Read-only display — all mutation
 * goes through the command input.
 */

interface NodeTreeItemProps {
  nodeId: string;
  label: string;
  icon?: string;
  variant: CompartmentVariant;
  selected: boolean;
  isIoNode: boolean;
}

function NodeTreeItem({ nodeId, label, icon, variant, selected, isIoNode }: NodeTreeItemProps) {
  const Icon = icon ? ICON_COMPONENTS[icon] : undefined;

  return (
    <div
      data-active={selected}
      className="flex w-full items-center gap-2.5 rounded-md px-2 py-1.5 data-[active=true]:bg-muted"
      data-testid={`node-tree-item-${nodeId}`}
      aria-current={selected ? "true" : undefined}
    >
      {Icon && (
        <IconBadge variant={variant} size="sm" className="size-7 shrink-0">
          <Icon className="size-3.5" />
        </IconBadge>
      )}
      <Text
        size="sm"
        weight={selected ? "medium" : "normal"}
        color={isIoNode ? "muted" : "default"}
        className="min-w-0 flex-1 truncate"
      >
        {label}
      </Text>
    </div>
  );
}

export { NodeTreeItem };
export type { NodeTreeItemProps };
