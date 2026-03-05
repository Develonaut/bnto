/**
 * Definition → Bento adapter.
 *
 * Converts a Definition's child nodes into BentoNodes (visual-only data)
 * and a configs map (domain data). Pure function — no React, no DOM.
 *
 * Domain fields (nodeType, name, parameters) go into configs[nodeId],
 * NOT into node.data. This prevents parameter changes from triggering
 * RF's change pipeline and re-rendering CompartmentNode.
 */

import type { Definition, NodeTypeName } from "@bnto/nodes";
import { NODE_TYPE_INFO } from "@bnto/nodes";
import type { BentoNode, BentoLayout, NodeConfigs } from "./types";
import { SLOTS } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";

function definitionToBento(definition: Definition): BentoLayout {
  const children = definition.nodes ?? [];
  const configs: NodeConfigs = {};

  const nodes: BentoNode[] = children.slice(0, SLOTS.length).map((node, index) => {
    const slot = SLOTS[index]!;
    const info = NODE_TYPE_INFO[node.type as NodeTypeName];
    const variant = info ? (CATEGORY_VARIANT[info.category] ?? "muted") : "muted";
    const label = node.name || (info?.label ?? node.type);

    // Domain data → configs map (not node.data)
    configs[node.id] = {
      nodeType: node.type,
      name: node.name,
      parameters: node.parameters,
    };

    return {
      id: node.id,
      type: "compartment" as const,
      position: { x: slot.x, y: slot.y },
      data: {
        label,
        sublabel: info?.category ?? "",
        variant,
        width: slot.w,
        height: slot.h,
        status: "idle" as const,
        icon: info?.icon,
      },
    };
  });

  return { nodes, configs };
}

export { definitionToBento };
