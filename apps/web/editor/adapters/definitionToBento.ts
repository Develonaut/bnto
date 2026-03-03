/**
 * Definition → Bento adapter.
 *
 * Converts a Definition's child nodes into CompartmentNode data for
 * the BentoCanvas. Pure function — no React, no DOM, fully testable.
 *
 * The bento box visual editor doesn't show edges. Execution order is
 * implied by compartment position (top-left → bottom-right).
 */

import type { Definition, NodeTypeName } from "@bnto/nodes";
import { NODE_TYPE_INFO } from "@bnto/nodes";
import type { BentoNode, BentoLayout } from "./types";
import { SLOTS } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";
function definitionToBento(definition: Definition): BentoLayout {
  const children = definition.nodes ?? [];

  const nodes: BentoNode[] = children
    .slice(0, SLOTS.length)
    .map((node, index) => {
      const slot = SLOTS[index]!;
      const info = NODE_TYPE_INFO[node.type as NodeTypeName];
      const variant = info
        ? CATEGORY_VARIANT[info.category] ?? "muted"
        : "muted";

      return {
        id: node.id,
        type: "compartment" as const,
        position: { x: slot.x, y: slot.y },
        data: {
          label: node.name || (info?.label ?? node.type),
          sublabel: info?.category ?? "",
          variant,
          width: slot.w,
          height: slot.h,
          status: "idle" as const,
          nodeId: node.id,
        },
      };
    });

  return { nodes };
}

export { definitionToBento };
