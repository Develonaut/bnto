import type { Definition } from "@bnto/registry";
import { isIoNodeType, isContainerNodeType } from "@bnto/registry";

/**
 * Clone a definition with user config overrides merged into leaf
 * processing nodes' parameters. Recurses into container children.
 * Skips I/O and container nodes themselves.
 */
export function applyConfigToDefinition(
  definition: Definition,
  config: Record<string, unknown>,
): Definition {
  if (!definition.nodes?.length || !Object.keys(config).length) return definition;
  return { ...definition, nodes: definition.nodes.map((node) => applyToNode(node, config)) };
}

function applyToNode(node: Definition, config: Record<string, unknown>): Definition {
  if (isIoNodeType(node.type)) return node;
  if (isContainerNodeType(node.type)) {
    if (!node.nodes?.length) return node;
    return { ...node, nodes: node.nodes.map((child) => applyToNode(child, config)) };
  }
  return { ...node, parameters: { ...node.parameters, ...config } };
}
