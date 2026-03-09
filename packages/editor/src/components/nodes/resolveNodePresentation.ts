/**
 * resolveNodePresentation — pure function mapping execution status
 * to Pressable + Card visual props.
 *
 * | Status    | Pressable                | Elevation | Rationale                          |
 * |-----------|--------------------------|-----------|-------------------------------------|
 * | idle      | active={selected}        | md/lg     | Current default behavior            |
 * | pending   | pressed={true}           | sm        | All nodes press down at run start   |
 * | active    | hovered={true}           | md        | Active node rises slightly          |
 * | completed | (none)                   | lg        | Node pops up — spring handles anim  |
 * | failed    | (none)                   | lg + ring | Red ring, elevated                  |
 */

import type { CompartmentNodeData } from "../../adapters/types";

type NodeStatus = CompartmentNodeData["status"];

interface NodePresentation {
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
  /** Whether to show a destructive ring. */
  failed: boolean;
}

function resolveNodePresentation(status: NodeStatus, selected: boolean): NodePresentation {
  switch (status) {
    case "pending":
      return {
        active: false,
        hovered: false,
        pressed: true,
        muted: true,
        elevation: "sm",
        failed: false,
      };
    case "active":
      return {
        active: false,
        hovered: true,
        pressed: false,
        muted: false,
        elevation: "md",
        failed: false,
      };
    case "completed":
      return {
        active: false,
        hovered: false,
        pressed: false,
        muted: false,
        elevation: "lg",
        failed: false,
      };
    case "failed":
      return {
        active: false,
        hovered: false,
        pressed: false,
        muted: false,
        elevation: "lg",
        failed: true,
      };
    default:
      return {
        active: selected,
        hovered: false,
        pressed: false,
        muted: false,
        elevation: selected ? "lg" : "md",
        failed: false,
      };
  }
}

export { resolveNodePresentation };
export type { NodePresentation };
