/**
 * buildChildDefinition — creates a Definition node for a new child
 * being added to a container.
 *
 * Shared by addChildIntoContainer and addSiblingChild to avoid
 * duplicating the Definition construction.
 */

import type { Definition, NodeTypeName } from "@bnto/core";
import type { NodeConfig } from "../adapters/types";

function buildChildDefinition(nodeId: string, type: NodeTypeName, config: NodeConfig): Definition {
  return {
    id: nodeId,
    type,
    version: "1.0.0",
    name: config.name,
    position: { x: 0, y: 0 },
    metadata: {},
    parameters: config.parameters,
    inputPorts: [],
    outputPorts: [],
  };
}

export { buildChildDefinition };
