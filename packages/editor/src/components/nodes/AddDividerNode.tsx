"use client";

import { memo, useCallback, type MouseEvent } from "react";
import type { NodeProps } from "@xyflow/react";
import { Plus } from "lucide-react";
import { Button, Divider } from "@bnto/ui";
import type { BentoNode, CompartmentNodeData } from "../../adapters/types";
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
 * CSS-first hover: the button uses the `dormant` prop to start
 * grounded + muted, waking on ancestor .group hover via CSS.
 */

export const AddDividerNode = memo(function AddDividerNode({ data }: NodeProps<BentoNode>) {
  const { open: openPalette } = usePanels("palette");
  const editor = useEditor();
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

  return <AddDividerLayout data={data} onClick={handleClick} />;
});

interface AddDividerLayoutProps {
  data: CompartmentNodeData;
  onClick: (e: MouseEvent) => void;
}

function AddDividerLayout({ data, onClick }: AddDividerLayoutProps) {
  const direction = data.dividerDirection ?? "horizontal";
  const hideLine = data.dividerHideLine ?? false;
  const dividerOrientation = direction === "horizontal" ? "vertical" : "horizontal";

  return (
    <div
      className="flex items-center justify-center"
      style={{ width: data.width, height: data.height }}
    >
      {!hideLine && (
        <Divider
          orientation={dividerOrientation}
          className={dividerOrientation === "vertical" ? "absolute h-[60%]" : "absolute w-[60%]"}
        />
      )}
      <Button
        icon={<Plus />}
        size="sm"
        variant="primary"
        dormant
        onClick={onClick}
        aria-label="Add node"
        data-testid="add-divider"
        className="nopan nodrag nowheel z-canvas"
      />
    </div>
  );
}
