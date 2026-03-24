"use client";

import { memo, useCallback } from "react";
import {
  EllipsisVerticalIcon,
  Menu,
  MenuContent,
  MenuItem,
  MenuTrigger,
  TrashIcon,
} from "@bnto/ui";
import { useEditor } from "../../context";

/**
 * ConfigPanelNodeMenu — ellipsis menu for the selected node in the config panel.
 *
 * Actions: Delete (and later Rename).
 */

const ConfigPanelNodeMenu = memo(function ConfigPanelNodeMenu({
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
    <Menu>
      <MenuTrigger
        icon={<EllipsisVerticalIcon />}
        variant="primary"
        dormant
        aria-label={`Actions for ${nodeName}`}
        data-testid="config-node-menu"
      />
      <MenuContent align="end">
        <MenuItem onClick={handleDelete} className="text-destructive">
          <TrashIcon />
          Delete
        </MenuItem>
      </MenuContent>
    </Menu>
  );
});

export { ConfigPanelNodeMenu };
