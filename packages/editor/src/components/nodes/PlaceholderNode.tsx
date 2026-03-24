"use client";

import { memo, type CSSProperties } from "react";
import { Button, Card } from "@bnto/ui";
import { Plus } from "lucide-react";
import { usePanels } from "../../hooks/usePanels";
import { CELL } from "../../adapters/bentoSlots";

/**
 * PlaceholderNode — a ReactFlow node type for the empty slot CTA.
 *
 * Rendered as a dashed semi-transparent card with a centered icon button.
 * Positioned at the next available slot in the bento grid (after
 * input + output). Clicking the button opens the node palette menu
 * (controlled by store state). When a processing node is added, it
 * takes this slot's position and the placeholder disappears.
 *
 * Registered as the `placeholder` node type alongside CompartmentNode.
 * Non-interactive to ReactFlow (not selectable, not draggable).
 */

const PlaceholderNode = memo(function PlaceholderNode() {
  const { open: openPalette } = usePanels("palette");

  return (
    <Card
      border="dashed"
      elevation="none"
      className="pointer-events-auto flex items-center justify-center rounded-xl"
      style={
        {
          width: CELL,
          height: CELL,
          transformStyle: "flat",
          "--face-bg": "color-mix(in oklch, var(--card) 60%, transparent)",
        } as CSSProperties
      }
      data-testid="placeholder-node"
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

export { PlaceholderNode };
