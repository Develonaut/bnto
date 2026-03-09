/**
 * resolveNodePresentation — pure function mapping execution status
 * to Pressable + Card visual props.
 *
 * | Status    | Pressable                | Elevation | Rationale                          |
 * |-----------|--------------------------|-----------|-------------------------------------|
 * | idle      | active={selected}        | md/lg     | Current default behavior            |
 * | pending   | pressed={true}           | sm        | All nodes press down, muted card    |
 * | active    | hovered={true}           | md        | Active node rises slightly          |
 * | completed | (none)                   | lg        | Node pops up — spring handles anim  |
 * | failed    | (none)                   | lg        | Elevated                            |
 */

import type { ComponentProps } from "react";
import type { Card } from "@bnto/ui";
import type { CompartmentNodeData } from "../../adapters/types";

type NodeStatus = CompartmentNodeData["status"];
type CardColor = ComponentProps<typeof Card>["color"];

interface NodePresentation {
  /** Card color variant (e.g. "muted" for pending). */
  color?: CardColor;
  /** Pressable: programmatic active (flush with ground). */
  active: boolean;
  /** Pressable: programmatic hover (partially sunk). */
  hovered: boolean;
  /** Pressable: programmatic pressed (flush with ground). */
  pressed: boolean;
  /** Pressable: muted appearance. */
  muted: boolean;
  /** Card elevation level. */
  elevation: "sm" | "md" | "lg";
}

function resolveNodePresentation(status: NodeStatus, selected: boolean): NodePresentation {
  switch (status) {
    case "pending":
      return {
        color: "muted",
        active: false,
        hovered: false,
        pressed: true,
        muted: false,
        elevation: "sm",
      };
    case "active":
      return {
        active: false,
        hovered: true,
        pressed: false,
        muted: false,
        elevation: "md",
      };
    case "completed":
      return {
        active: false,
        hovered: false,
        pressed: false,
        muted: false,
        elevation: "lg",
      };
    case "failed":
      return {
        active: false,
        hovered: false,
        pressed: false,
        muted: false,
        elevation: "lg",
      };
    default:
      return {
        active: selected,
        hovered: false,
        pressed: false,
        muted: false,
        elevation: selected ? "lg" : "md",
      };
  }
}

export { resolveNodePresentation };
export type { NodePresentation };
