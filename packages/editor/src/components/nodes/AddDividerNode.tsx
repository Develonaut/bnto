"use client";

import { memo, useCallback, type MouseEvent } from "react";
import type { NodeProps } from "@xyflow/react";
import { Plus } from "lucide-react";
import { Button, Divider } from "@bnto/ui";
import type { BentoNode } from "../../adapters/types";
import { usePanels } from "../../hooks/usePanels";
import { useEditor } from "../../context";

/**
 * AddDividerNode — a divider element between nodes on the canvas.
 *
 * Uses the Divider primitive from @bnto/ui with a centered plus button.
 * Positioned in the gap between two consecutive nodes on the canvas.
 *
 * Two directions:
 * - "horizontal": rotated Divider in the GAP_X between top-level nodes
 * - "vertical": standard Divider between children of a container
 *
 * Non-interactive to RF (not selectable, not draggable). The plus
 * button is pointer-events-auto inside a pointer-events-none shell.
 *
 * CSS-first hover: the button is invisible by default and revealed
 * via group-hover / group-focus-within on the parent container.
 */

export const AddDividerNode = memo(function AddDividerNode({ data }: NodeProps<BentoNode>) {
  const { open: openPalette } = usePanels("palette");
  const editor = useEditor();
  const direction = data.dividerDirection ?? "horizontal";
  const afterNodeId = data.dividerAfterNodeId ?? null;
  const intoContainerId = data.dividerIntoContainerId ?? null;

  const handleClick = useCallback(
    (e: MouseEvent) => {
      e.stopPropagation();
      editor.nodes.setInsertAfterNodeId(afterNodeId);
      editor.nodes.setInsertIntoContainerId(intoContainerId);
      openPalette();
    },
    [afterNodeId, intoContainerId, editor, openPalette],
  );

  // "horizontal" direction = gap between top-level nodes = vertical divider line
  // "vertical" direction = gap between children = horizontal divider line
  const dividerOrientation = direction === "horizontal" ? "vertical" : "horizontal";

  return (
    <div
      className="group flex items-center justify-center"
      style={{ width: data.width, height: data.height }}
    >
      <Divider
        orientation={dividerOrientation}
        className={dividerOrientation === "vertical" ? "absolute h-[60%]" : "absolute w-[60%]"}
      />
      <Button
        icon={<Plus />}
        size="sm"
        variant="primary"
        onClick={handleClick}
        aria-label="Add node"
        data-testid="add-divider"
        className="nopan nodrag nowheel pointer-events-none z-canvas size-5 opacity-0 transition-opacity duration-fast group-hover:pointer-events-auto group-hover:opacity-100 group-focus-within:pointer-events-auto group-focus-within:opacity-100 [&_svg]:size-3"
      />
    </div>
  );
});
