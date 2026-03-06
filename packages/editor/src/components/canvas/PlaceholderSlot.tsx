"use client";

import { memo } from "react";
import { Button, Card } from "@bnto/ui";
import { Plus } from "lucide-react";
import { useEditorPanels } from "../../hooks/useEditorPanels";
import { CELL } from "../../adapters/bentoSlots";

/**
 * PlaceholderSlot — a ReactFlow node type for the empty slot CTA.
 *
 * Rendered as a dashed muted card with a centered icon button.
 * Positioned at the next available slot in the bento grid (after
 * input + output). Clicking the button opens the node palette menu
 * (controlled by store state). When a processing node is added, it
 * takes this slot's position and the placeholder disappears.
 *
 * Registered as the `placeholder` node type alongside CompartmentNode.
 * Non-interactive to ReactFlow (not selectable, not draggable).
 */

const PlaceholderSlot = memo(function PlaceholderSlot() {
  const { openPalette } = useEditorPanels();

  return (
    <Card
      dashed
      color="muted"
      elevation="none"
      className="pointer-events-auto flex items-center justify-center rounded-xl"
      style={{ width: CELL, height: CELL, transformStyle: "flat" }}
      data-testid="placeholder-slot"
    >
      <Button
        size="icon"
        variant="primary"
        elevation="sm"
        onClick={openPalette}
        aria-label="Add node"
        className="nopan nodrag nowheel"
      >
        <Plus className="size-4" />
      </Button>
    </Card>
  );
});

export { PlaceholderSlot };
