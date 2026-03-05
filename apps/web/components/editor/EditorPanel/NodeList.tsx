"use client";

import { Text } from "@bnto/ui";
import type { BentoNode } from "@/editor/adapters/types";
import { NodeItem } from "./NodeItem";

/**
 * NodeList — scrollable list of canvas nodes, or an empty prompt.
 */

interface NodeListProps {
  nodes: BentoNode[];
  selectedNodeId: string | null;
  onSelect: (nodeId: string) => void;
}

function NodeList({ nodes, selectedNodeId, onSelect }: NodeListProps) {
  if (nodes.length === 0) {
    return (
      <div className="px-3 py-4">
        <Text size="xs" color="muted" className="text-center">
          No nodes yet. Add one from the toolbar.
        </Text>
      </div>
    );
  }

  return (
    <ul
      className="flex flex-1 flex-col gap-0.5 overflow-y-auto p-1.5"
      role="listbox"
      aria-label="Canvas nodes"
    >
      {nodes.map((node) => (
        <NodeItem
          key={node.id}
          node={node}
          selected={node.id === selectedNodeId}
          onSelect={onSelect}
        />
      ))}
    </ul>
  );
}

export { NodeList };
