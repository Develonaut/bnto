/**
 * Definition → Graph adapter.
 *
 * Converts a Definition (nested tree) into Graph state (flat editor
 * working state): BentoNode[] + NodeConfigs. Pure function — no React, no DOM.
 *
 * Domain fields (nodeType, name, parameters) go into configs[nodeId],
 * NOT into node.data. This prevents parameter changes from triggering
 * RF's change pipeline and re-rendering CompartmentNode.
 */

import type { Definition, NodeTypeName } from "@bnto/nodes";
import { NODE_TYPE_INFO, getNodeIcon, getNodeSublabel, isIoNodeType, isContainerNodeType } from "@bnto/nodes";
import type { BentoNode, BentoLayout, NodeConfigs } from "./types";
import { SLOTS, IO_CARD_SIZE } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";

function definitionToGraph(definition: Definition): BentoLayout {
  const children = definition.nodes ?? [];
  const configs: NodeConfigs = {};

  // All nodes occupy the same CELL×CELL slot — uniform grid, no cursor math.
  // I/O nodes render a smaller inner card; alignment is handled by the renderer.
  const nodes: BentoNode[] = children.slice(0, SLOTS.length).map((node, i) => {
    const slot = SLOTS[i]!;
    const nodeType = node.type as NodeTypeName;
    const info = NODE_TYPE_INFO[nodeType];
    const variant = info ? (CATEGORY_VARIANT[info.category] ?? "muted") : "muted";

    const displayName = node.metadata?.customData?.displayName;
    configs[node.id] = {
      nodeType: node.type,
      name: node.name,
      ...(displayName ? { displayName } : {}),
      parameters: node.parameters,
    };

    const isIo = isIoNodeType(node.type);
    const label = isIo
      ? (info?.label ?? node.type)
      : displayName || node.name || (info?.label ?? node.type);
    const icon = getNodeIcon(nodeType, node.parameters);
    const sublabel = getNodeSublabel(nodeType, node.parameters, node.metadata);

    const isContainer = isContainerNodeType(node.type);

    return {
      id: node.id,
      type: isIo ? ("io" as const) : ("compartment" as const),
      position: { x: slot.x, y: slot.y },
      data: {
        label,
        sublabel,
        variant,
        width: isIo ? IO_CARD_SIZE : slot.w,
        height: isIo ? IO_CARD_SIZE : slot.h,
        status: "idle" as const,
        icon,
        isIoNode: isIo,
        ...(isContainer ? { isContainer: true, isExpanded: true, depth: 0 } : { depth: 0 }),
      },
    };
  });

  return { nodes, configs };
}

export { definitionToGraph };
