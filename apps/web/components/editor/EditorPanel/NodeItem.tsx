"use client";

import { cn } from "@/lib/cn";
import { Text } from "@/components/ui/Text";
import type { BentoNode, CompartmentVariant } from "@/editor/adapters/types";

/**
 * NodeItem — single row in the sidebar node list.
 *
 * Shows a colored dot (matching the node's variant), the label,
 * and an optional sublabel. Highlights when selected.
 */

const DOT_COLOR: Record<CompartmentVariant, string> = {
  primary: "bg-primary",
  secondary: "bg-secondary",
  accent: "bg-accent",
  muted: "bg-muted-foreground",
  success: "bg-green-500",
  warning: "bg-amber-500",
};

interface NodeItemProps {
  node: BentoNode;
  selected: boolean;
  onSelect: (nodeId: string) => void;
}

function NodeItem({ node, selected, onSelect }: NodeItemProps) {
  const variant = node.data.variant ?? "muted";

  return (
    <li role="option" aria-selected={selected}>
      <button
        type="button"
        onClick={() => onSelect(node.id)}
        data-selected={selected}
        className="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-left transition-colors duration-fast hover:bg-muted data-[selected=true]:bg-muted data-[selected=true]:ring-2 data-[selected=true]:ring-ring"
      >
        <span
          className={cn("size-2 shrink-0 rounded-full", DOT_COLOR[variant])}
        />
        <span className="flex min-w-0 flex-1 flex-col">
          <Text size="sm" className="truncate font-medium leading-tight">
            {node.data.label}
          </Text>
          {node.data.sublabel && (
            <Text size="xs" color="muted" className="truncate leading-tight">
              {node.data.sublabel}
            </Text>
          )}
        </span>
      </button>
    </li>
  );
}

export { NodeItem };
