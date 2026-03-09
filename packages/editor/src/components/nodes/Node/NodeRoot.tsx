import type { ReactNode } from "react";
import { ScaleIn, useButtonProps, cn } from "@bnto/ui";
import { CELL } from "../../../adapters/bentoSlots";
import { resolveNodePresentation } from "../resolveNodePresentation";

/**
 * NodeRoot — the outermost shell for all bento grid nodes.
 *
 * Uses `useButtonProps` to get pressable/surface/elevation slots
 * without wrapping in a Button component. Renders as a plain `<div>`
 * because nodes contain nested interactive elements (delete button, config).
 *
 * NodeRoot owns interaction state: `selected` and `status` drive
 * pressed/hovered/elevation/variant via resolveNodePresentation.
 * Consuming nodes pass through what ReactFlow gives them.
 */

const JUSTIFY_MAP = {
  start: "justify-start",
  center: "justify-center",
  end: "justify-end",
} as const;

type NodeVariant = "primary" | "secondary" | "muted" | "destructive" | "success" | "warning";

interface NodeRootProps {
  /** Inner card width (defaults to CELL). */
  width?: number;
  /** Inner card height (defaults to CELL). */
  height?: number;
  /** Default surface color variant (e.g. "muted" for I/O nodes). */
  variant?: NodeVariant;
  /** Horizontal alignment of a smaller card inside the CELL×CELL slot. */
  align?: "start" | "center" | "end";
  /** Whether the node is selected in ReactFlow. Drives pressed state. */
  selected?: boolean;
  /** Current execution status. Drives elevation, variant overrides, interaction. */
  status?: string;
  /** Composed content — NodeHeader, NodeBody, NodeFooter. */
  children: ReactNode;
}

function NodeRoot({
  width = CELL,
  height = CELL,
  variant,
  align,
  selected = false,
  status,
  children,
}: NodeRootProps) {
  const presentation = resolveNodePresentation(status, selected);
  const resolvedVariant = presentation.variant ?? variant;
  const slotClass = align ? `pointer-events-none flex items-center ${JUSTIFY_MAP[align]}` : "";

  const { slots } = useButtonProps({
    variant: resolvedVariant,
    elevation: presentation.elevation,
    pressed: presentation.pressed,
    hovered: presentation.hovered,
    muted: presentation.muted,
  });

  return (
    <div style={{ width: CELL, height: CELL }} className={slotClass} data-testid="node-slot">
      <ScaleIn from={0.7} easing="spring-bouncy">
        <div
          {...slots.root.props}
          data-testid="node-card"
          data-state={status}
          className={cn(
            slots.root.props.className,
            slots.surface.props.className,
            !resolvedVariant && "bg-card text-card-foreground",
            "group relative flex flex-col rounded-xl pointer-events-auto",
          )}
          style={{ ...slots.root.props.style, width, height }}
        >
          {children}
        </div>
      </ScaleIn>
    </div>
  );
}

export { NodeRoot };
export type { NodeRootProps, NodeVariant };
