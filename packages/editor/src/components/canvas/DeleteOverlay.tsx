"use client";

import { memo, useCallback, type MouseEvent } from "react";
import { Button } from "@bnto/ui";
import { X } from "lucide-react";
import { useRemoveNode } from "../../hooks/useRemoveNode";

/**
 * DeleteOverlay — hover-reveal delete button for compartment nodes.
 *
 * Full-bleed overlay inside a `group` parent with inset padding.
 * Appears on group hover/focus-within. Delete button sits top-right.
 *
 * Hidden for I/O nodes (caller controls rendering).
 */

const DeleteOverlay = memo(function DeleteOverlay({ nodeId }: { nodeId: string }) {
  const removeNode = useRemoveNode();

  const handleDelete = useCallback(
    (e: MouseEvent) => {
      e.stopPropagation();
      removeNode(nodeId);
    },
    [nodeId, removeNode],
  );

  return (
    <div
      className="absolute inset-0 flex items-start justify-end p-3 opacity-0
        transition-opacity
        group-hover:opacity-100 group-focus-within:opacity-100"
    >
      <Button
        icon={<X />}
        size="sm"
        variant="destructive"
        elevation="sm"
        onClick={handleDelete}
        aria-label="Delete node"
        data-testid="delete-node"
      />
    </div>
  );
});

export { DeleteOverlay };
