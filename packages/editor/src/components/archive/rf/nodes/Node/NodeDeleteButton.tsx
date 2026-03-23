"use client";

import { memo, useCallback, type MouseEvent } from "react";
import { Button, TrashIcon } from "@bnto/ui";
import { useEditor } from "../../../../../context";

/**
 * NodeDeleteButton — delete action for a node.
 *
 * Rendered inside NodeHeader. Disabled when the node is not
 * selected — enabled (interactive) when selected.
 */

const NodeDeleteButton = memo(function NodeDeleteButton({
  nodeId,
  selected,
}: {
  nodeId: string;
  selected?: boolean;
}) {
  const editor = useEditor();

  const handleDelete = useCallback(
    (e: MouseEvent) => {
      e.stopPropagation();
      editor.nodes.removeNode(nodeId);
    },
    [nodeId, editor],
  );

  return (
    <Button
      icon={<TrashIcon />}
      size="sm"
      variant="destructive"
      disabled={!selected}
      onClick={handleDelete}
      aria-label="Delete node"
      data-testid="delete-node"
      className="nopan nodrag nowheel size-6 [&_svg]:size-3"
    />
  );
});

export { NodeDeleteButton };
