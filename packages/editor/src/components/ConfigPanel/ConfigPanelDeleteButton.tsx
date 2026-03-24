"use client";

import { memo, useCallback } from "react";
import { Button, TrashIcon } from "@bnto/ui";
import { useEditor } from "../../context";

/**
 * ConfigPanelDeleteButton — trash button for the selected node in the config panel.
 */

const ConfigPanelDeleteButton = memo(function ConfigPanelDeleteButton({
  nodeId,
  nodeName,
}: {
  nodeId: string;
  nodeName: string;
}) {
  const editor = useEditor();

  const handleDelete = useCallback(() => {
    editor.nodes.removeNode(nodeId);
  }, [nodeId, editor]);

  return (
    <Button
      icon={<TrashIcon />}
      variant="destructive"
      dormant
      onClick={handleDelete}
      aria-label={`Delete ${nodeName}`}
      data-testid="config-node-delete"
    />
  );
});

export { ConfigPanelDeleteButton };
