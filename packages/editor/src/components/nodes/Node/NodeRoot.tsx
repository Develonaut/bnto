import type { ReactNode } from "react";
import { Button, ScaleIn, cn } from "@bnto/ui";
import { CELL } from "../../../adapters/bentoSlots";
import { resolveNodePresentation } from "../resolveNodePresentation";

/**
 * NodeRoot — the outermost shell for all bento grid nodes.
 *
 * Uses `<Button as="div">` to get pressable/surface/elevation behavior
 * without rendering a `<button>` element. Nodes contain nested interactive
 * elements (delete button, config) so the root must be a plain `<div>`.
 *
 * The card interior uses a single-cell CSS Grid overlay where all
 * child zones share the same cell and self-align to their edges:
 *
 *   +------------------+
 *   |   header         |  <- self-start (top overlay)
 *   |                  |
 *   |      body        |  <- place-self-center
 *   |                  |
 *   +------------------+
 *
 * Zones overlay without displacing body content -- the body stays
 * perfectly centered regardless of which zones are present.
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
  /** Horizontal alignment of a smaller card inside the CELL x CELL slot. */
  align?: "start" | "center" | "end";
  /** Whether the node is selected in ReactFlow. Drives pressed state. */
  selected?: boolean;
  /** Current execution status. Drives elevation, variant overrides, interaction. */
  status?: string;
  /** Accessible label describing this node (e.g. "Compress Images node"). */
  "aria-label"?: string;
  /** Composed content -- NodeBody, NodeIcon, NodeLabel, etc. */
  children: ReactNode;
}

/** Resolve alignment class for the CELL x CELL slot container. */
function slotClassName(align: NodeRootProps["align"]) {
  return align ? `pointer-events-none flex items-center ${JUSTIFY_MAP[align]}` : "";
}

function NodeRoot(props: NodeRootProps) {
  const { variant, align, children, status } = props;
  const { width = CELL, height = CELL, selected = false, "aria-label": ariaLabel } = props;
  const presentation = resolveNodePresentation(status, selected);

  return (
    <div
      style={{ width: CELL, height: CELL }}
      className={cn("group relative", slotClassName(align))}
      data-testid="node-slot"
    >
      <NodeCard
        width={width}
        height={height}
        variant={presentation.variant ?? variant}
        presentation={presentation}
        ariaLabel={ariaLabel}
        selected={selected}
        status={status}
      >
        {children}
      </NodeCard>
    </div>
  );
}

interface NodeCardProps {
  width: number;
  height: number;
  variant: NodeVariant | undefined;
  presentation: ReturnType<typeof resolveNodePresentation>;
  ariaLabel: string | undefined;
  selected: boolean;
  status: string | undefined;
  children: ReactNode;
}

/** Resolve card className based on variant presence. */
function cardClassName(variant: NodeVariant | undefined) {
  return cn(
    !variant && "bg-card text-card-foreground",
    "relative grid grid-cols-1 grid-rows-1 rounded-xl pointer-events-auto",
  );
}

function NodeCard({
  width,
  height,
  variant,
  presentation,
  ariaLabel,
  selected,
  status,
  children,
}: NodeCardProps) {
  return (
    <ScaleIn from={0.7} easing="spring-bouncy">
      <Button
        as="div"
        variant={variant}
        elevation={presentation.elevation}
        pressed={presentation.pressed}
        hovered={presentation.hovered}
        muted={presentation.muted}
        role="button"
        aria-label={ariaLabel}
        aria-selected={selected}
        data-testid="node-card"
        data-state={status}
        className={cardClassName(variant)}
        style={{ width, height }}
      >
        {children}
      </Button>
    </ScaleIn>
  );
}

export { NodeRoot };
export type { NodeRootProps, NodeVariant };
