import type { Definition } from "@bnto/registry";
import { isIoNodeType, isContainerNodeType } from "@bnto/registry";

/**
 * Clone a definition with per-node config overrides merged into leaf
 * processing nodes' parameters. Recurses into container children.
 * Skips I/O and container nodes themselves.
 */
export function applyConfigToDefinition(
  definition: Definition,
  perNodeConfig: Record<string, Record<string, unknown>>,
): Definition {
  if (!definition.nodes?.length || !Object.keys(perNodeConfig).length) return definition;
  return {
    ...definition,
    nodes: definition.nodes.map((node) => applyToNode(node, perNodeConfig)),
  };
}

function applyToNode(
  node: Definition,
  perNodeConfig: Record<string, Record<string, unknown>>,
): Definition {
  if (isIoNodeType(node.type)) return node;
  if (isContainerNodeType(node.type)) {
    if (!node.nodes?.length) return node;
    return { ...node, nodes: node.nodes.map((child) => applyToNode(child, perNodeConfig)) };
  }
  const nodeOverrides = perNodeConfig[node.id];
  if (!nodeOverrides) return node;
  return { ...node, parameters: { ...node.parameters, ...nodeOverrides } };
}
