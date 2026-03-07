"use client";

import { memo, useCallback, type MouseEvent } from "react";
import { Button } from "@bnto/ui";
import { X } from "lucide-react";
import { useRemoveNode } from "../../../hooks/useRemoveNode";

/**
 * NodeDeleteButton — delete action for a node.
 *
 * Always rendered inside NodeHeader. Disabled when the node
 * is not selected — enabled (interactive) when selected.
 */

const NodeDeleteButton = memo(function NodeDeleteButton({
  nodeId,
  selected,
}: {
  nodeId: string;
  selected?: boolean;
}) {
  const removeNode = useRemoveNode();

  const handleDelete = useCallback(
    (e: MouseEvent) => {
      e.stopPropagation();
      removeNode(nodeId);
    },
    [nodeId, removeNode],
  );

  return (
    <Button
      icon={<X />}
      size="sm"
      variant="destructive"
      elevation="sm"
      disabled={!selected}
      onClick={handleDelete}
      aria-label="Delete node"
      data-testid="delete-node"
      className="nopan nodrag nowheel"
    />
  );
});

export { NodeDeleteButton };
