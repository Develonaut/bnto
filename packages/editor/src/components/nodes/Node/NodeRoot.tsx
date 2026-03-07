import type { ReactNode, ComponentProps } from "react";
import { ScaleIn, Card, Pressable } from "@bnto/ui";
import { CELL } from "../../../adapters/bentoSlots";

type CardProps = ComponentProps<typeof Card>;

/**
 * NodeRoot — the outermost shell for all bento grid nodes.
 *
 * Follows the same Pressable → Card pattern as RecipeCard:
 * Pressable wraps Card, CSS handles all interaction states.
 * When `selected`, `data-active` on the wrapper makes the card
 * sit flush with the ground via `.pressable[data-active]` CSS.
 */

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
  /** Destructive ring when a node has failed during execution. */
  failed?: boolean;
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
  failed = false,
  children,
}: NodeRootProps) {
  const justifyMap = {
    start: "justify-start",
    center: "justify-center",
    end: "justify-end",
  } as const;

  const slotClass = align ? `pointer-events-none flex items-center ${justifyMap[align]}` : "";

  return (
    <div style={{ width: CELL, height: CELL }} className={slotClass} data-testid="node-slot">
      <ScaleIn from={0.7} easing="spring-bouncy">
        <Pressable asChild spring="bounciest" muted={muted} active={selected}>
          <Card
            elevation={elevation}
            color={color}
            className={`group relative flex flex-col rounded-xl pointer-events-auto${failed ? " ring-2 ring-destructive/60" : ""}`}
            style={{ width, height }}
            data-testid="node-card"
          >
            {children}
          </Card>
        </Pressable>
      </ScaleIn>
    </div>
  );
}

export { NodeRoot };
export type { NodeRootProps };
