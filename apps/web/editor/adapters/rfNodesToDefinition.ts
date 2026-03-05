/**
 * RF nodes → Definition adapter.
 *
 * Reads RF nodes (visual-only data) plus the configs map (domain data)
 * and reconstructs a Definition tree. Used at export boundaries only.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import type { Definition } from "@bnto/nodes";
import type { BentoNode, NodeConfigs } from "./types";

/**
 * Build a Definition from the current RF node state + configs.
 *
 * @param rfNodes - Compartment nodes (visual-only data)
 * @param rootDefinition - The parent definition (for root-level metadata)
 * @param configs - Domain data keyed by node ID
 * @returns Full Definition with child nodes
 */
function rfNodesToDefinition(
  rfNodes: BentoNode[],
  rootDefinition: Definition,
  configs: NodeConfigs = {},
): Definition {
  const children: Definition[] = rfNodes.map((rfNode) => {
    const config = configs[rfNode.id];
    return {
      id: rfNode.id,
      type: config?.nodeType ?? "unknown",
      version: "1.0.0",
      name: config?.name ?? rfNode.data.label,
      position: rfNode.position,
      parameters: config?.parameters ?? {},
      metadata: {},
      inputPorts: [],
      outputPorts: [],
    };
  });

  return {
    ...rootDefinition,
    nodes: children,
  };
}

export { rfNodesToDefinition };
