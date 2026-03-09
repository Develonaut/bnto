/**
 * resolveNodePresentation — maps execution status to Button props.
 *
 * Status → variant is a simple map. Status → interaction/elevation
 * is a lookup table. Selected always means pressed.
 */

import type { NodeVariant } from "./Node/NodeRoot";

/** Status-to-variant map. Undefined = use node's default variant. */
const STATUS_VARIANT: Partial<Record<string, NodeVariant>> = {
  pending: "muted",
  failed: "destructive",
};

/** Status-to-elevation map. Undefined = use default (md). */
const STATUS_ELEVATION: Record<string, "sm" | "md" | "lg"> = {
  pending: "sm",
  active: "md",
  completed: "lg",
  failed: "lg",
};

interface NodePresentation {
  variant?: NodeVariant;
  hovered: boolean;
  pressed: boolean;
  muted: boolean;
  elevation: "sm" | "md" | "lg";
}

function resolveNodePresentation(status: string | undefined, selected: boolean): NodePresentation {
  const variant = status ? STATUS_VARIANT[status] : undefined;
  const baseElevation = status ? STATUS_ELEVATION[status] : undefined;

  return {
    variant,
    hovered: status === "active",
    pressed: selected || status === "pending",
    muted: false,
    elevation: baseElevation ?? (selected ? "lg" : "md"),
  };
}

export { resolveNodePresentation };
export type { NodePresentation };
