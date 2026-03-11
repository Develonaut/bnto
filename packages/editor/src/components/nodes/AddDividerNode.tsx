"use client";

import { memo, useCallback, useState, type MouseEvent } from "react";
import type { NodeProps } from "@xyflow/react";
import { Plus } from "lucide-react";
import { Button, Divider } from "@bnto/ui";
import type { BentoNode } from "../../adapters/types";
import { usePanel } from "../../hooks/usePanel";
import { useEditorStoreApi } from "../../hooks/useEditorStoreApi";

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
 * Disabled by default, enabled on hover — matches the NodeDeleteButton
 * pattern. JS state is required because CSS can't toggle the HTML
 * disabled attribute (coordinating parent hover → child prop).
 */

export const AddDividerNode = memo(function AddDividerNode({
  data,
}: NodeProps<BentoNode>) {
  const { open: openPalette } = usePanel("palette");
  const storeApi = useEditorStoreApi();
  const [hovered, setHovered] = useState(false);
  const direction = data.dividerDirection ?? "horizontal";
  const afterNodeId = data.dividerAfterNodeId ?? null;
  const intoContainerId = data.dividerIntoContainerId ?? null;

  const handleClick = useCallback(
    (e: MouseEvent) => {
      e.stopPropagation();
      storeApi.setState({
        insertAfterNodeId: afterNodeId,
        insertIntoContainerId: intoContainerId,
      });
      openPalette();
    },
    [afterNodeId, intoContainerId, storeApi, openPalette],
  );

  // "horizontal" direction = gap between top-level nodes = vertical divider line
  // "vertical" direction = gap between children = horizontal divider line
  const dividerOrientation = direction === "horizontal" ? "vertical" : "horizontal";

  return (
    <div
      className="flex items-center justify-center"
      style={{ width: data.width, height: data.height }}
      onMouseEnter={() => setHovered(true)}
      onMouseLeave={() => setHovered(false)}
    >
      <Divider
        orientation={dividerOrientation}
        className={dividerOrientation === "vertical" ? "absolute h-[60%]" : "absolute w-[60%]"}
      />
      <Button
        icon={<Plus />}
        size="sm"
        variant="primary"
        disabled={!hovered}
        onClick={handleClick}
        aria-label="Add node"
        data-testid="add-divider"
        className="nopan nodrag nowheel pointer-events-auto z-10 size-5 [&_svg]:size-3"
      />
    </div>
  );
});
