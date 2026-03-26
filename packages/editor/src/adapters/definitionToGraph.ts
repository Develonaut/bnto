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

import type { Definition, NodeTypeName } from "@bnto/core";
import {
  NODE_TYPE_INFO,
  getNodeIcon,
  getNodeSublabel,
  isIoNodeType,
  isContainerNodeType,
} from "@bnto/core";
import type { BentoNode, BentoLayout, NodeConfigs } from "./types";
import { SLOTS, IO_CARD_SIZE } from "./bentoSlots";
import { CATEGORY_VARIANT } from "./categoryVariant";

/** Resolve display label for a definition node. */
function resolveLabel(
  node: Definition,
  isIo: boolean,
  info: { label: string } | undefined,
): string {
  const displayName = node.metadata?.customData?.displayName;
  if (isIo) return info?.label ?? node.type;
  return displayName || node.name || (info?.label ?? node.type);
}

/** Convert a single Definition child node into a BentoNode. */
function mapDefinitionChild(node: Definition, slot: (typeof SLOTS)[number]): BentoNode {
  const nodeType = node.type as NodeTypeName;
  const info = NODE_TYPE_INFO[nodeType];
  const variant = info ? (CATEGORY_VARIANT[info.category] ?? "muted") : "muted";
  const isIo = isIoNodeType(node.type);
  const isContainer = isContainerNodeType(node.type);

  return {
    id: node.id,
    type: isIo ? ("io" as const) : ("compartment" as const),
    position: { x: slot.x, y: slot.y },
    data: {
      label: resolveLabel(node, isIo, info),
      sublabel: getNodeSublabel(nodeType, node.parameters, node.metadata),
      variant,
      width: isIo ? IO_CARD_SIZE : slot.w,
      height: isIo ? IO_CARD_SIZE : slot.h,
      status: "idle" as const,
      icon: getNodeIcon(nodeType, node.parameters),
      isIoNode: isIo,
      ...(isContainer ? { isContainer: true, isExpanded: true, depth: 0 } : { depth: 0 }),
    },
  };
}

/** Build the NodeConfig for a single definition child. */
function buildConfig(node: Definition): NodeConfigs[string] {
  const displayName = node.metadata?.customData?.displayName;
  return {
    nodeType: node.type,
    name: node.name,
    ...(displayName ? { displayName } : {}),
    parameters: node.parameters,
  };
}

function definitionToGraph(definition: Definition): BentoLayout {
  const children = definition.nodes ?? [];
  const configs: NodeConfigs = {};

  const nodes: BentoNode[] = children.slice(0, SLOTS.length).flatMap((node, i) => {
    const slot = SLOTS[i];
    if (!slot) return [];
    configs[node.id] = buildConfig(node);
    return [mapDefinitionChild(node, slot)];
  });

  return { nodes, configs };
}

export { definitionToGraph };
