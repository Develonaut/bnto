"use client";

import { memo, useCallback, type MouseEvent } from "react";
import { Button } from "@bnto/ui";
import { Plus } from "lucide-react";
import { usePanel } from "../../../hooks/usePanel";
import { useEditorStoreApi } from "../../../hooks/useEditorStoreApi";

/**
 * NodeAddButton — add-node action for a compartment node.
 *
 * Rendered inside NodeEdge. Disabled when the node is not
 * selected — enabled (interactive) when selected.
 * Sets the insertion context (insert after this node) and
 * opens the node palette dialog on click.
 */

const NodeAddButton = memo(function NodeAddButton({
  nodeId,
  selected,
}: {
  nodeId: string;
  selected?: boolean;
}) {
  const { open: openPalette } = usePanel("palette");
  const storeApi = useEditorStoreApi();

  const handleAdd = useCallback(
    (e: MouseEvent) => {
      e.stopPropagation();
      storeApi.setState({ insertAfterNodeId: nodeId });
      openPalette();
    },
    [nodeId, storeApi, openPalette],
  );

  return (
    <Button
      icon={<Plus />}
      size="sm"
      variant="primary"
      elevation="sm"
      disabled={!selected}
      onClick={handleAdd}
      aria-label="Add node"
      data-testid="add-node"
      className="nopan nodrag nowheel"
    />
  );
});

export { NodeAddButton };
