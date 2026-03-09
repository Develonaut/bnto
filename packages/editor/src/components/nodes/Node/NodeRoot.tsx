import type { ReactNode, ComponentProps } from "react";
import { cn, ScaleIn, Card, Pressable } from "@bnto/ui";
import { CELL } from "../../../adapters/bentoSlots";
import { NodeProgressFill } from "./NodeProgressFill";

type CardProps = ComponentProps<typeof Card>;

/**
 * NodeRoot — the outermost shell for all bento grid nodes.
 *
 * Follows the same Pressable → Card pattern as RecipeCard:
 * Pressable wraps Card, CSS handles all interaction states.
 * During execution, status-driven props (pressed/hovered/active)
 * animate the card through elevation states, and a progress fill
 * overlay shows per-node progress.
 */

const JUSTIFY_MAP = {
  start: "justify-start",
  center: "justify-center",
  end: "justify-end",
} as const;

interface NodeRootProps {
  /** Inner card width (defaults to CELL). */
  width?: number;
  /** Inner card height (defaults to CELL). */
  height?: number;
  /** Card elevation level. */
  elevation?: CardProps["elevation"];
  /** Card color variant (e.g. "muted" for I/O nodes). */
  color?: CardProps["color"];
  /** Horizontal alignment of a smaller card inside the CELL×CELL slot. */
  align?: "start" | "center" | "end";
  /** Pressable muted state (e.g. pending status). */
  muted?: boolean;
  /** Card sits flush with ground when selected. */
  selected?: boolean;
  /** Pressable: programmatic pressed state (flush with ground). */
  pressed?: boolean;
  /** Pressable: programmatic hover state (partially sunk). */
  hovered?: boolean;
  /** Pressable: programmatic active state (flush with ground). */
  active?: boolean;
  /** Shows a destructive ring when the node has failed. */
  failed?: boolean;
  /** Per-node execution progress (0–100). */
  progress?: number;
  /** Current execution status — exposed as data attribute for testing. */
  status?: string;
  /** Composed content — NodeHeader, NodeBody, NodeFooter. */
  children: ReactNode;
}

function NodeRoot({
  width = CELL,
  height = CELL,
  elevation = "md",
  color,
  align,
  muted = false,
  selected = false,
  pressed = false,
  hovered = false,
  active = false,
  failed = false,
  progress,
  status,
  children,
}: NodeRootProps) {
  const slotClass = align ? `pointer-events-none flex items-center ${JUSTIFY_MAP[align]}` : "";

  return (
    <div style={{ width: CELL, height: CELL }} className={slotClass} data-testid="node-slot">
      <ScaleIn from={0.7} easing="spring-bouncy">
        <Pressable
          asChild
          spring="bounciest"
          muted={muted}
          pressed={pressed}
          hovered={hovered}
          active={active}
        >
          <Card
            elevation={elevation}
            color={color}
            className={cn(
              "group relative flex flex-col rounded-xl overflow-hidden pointer-events-auto",
              failed && "ring-2 ring-destructive",
            )}
            style={{ width, height }}
            data-testid="node-card"
            data-state={status}
            data-progress={progress !== undefined ? progress : undefined}
          >
            <NodeProgressFill progress={progress} />
            {children}
          </Card>
        </Pressable>
      </ScaleIn>
    </div>
  );
}

export { NodeRoot };
export type { NodeRootProps };
