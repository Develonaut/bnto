/**
 * Adds a new child node to the root group of a definition.
 *
 * Creates a node with:
 * - A unique UUID
 * - Default parameters from the node type's schema
 * - Default input/output ports from NODE_TYPE_INFO
 * - The specified position (or {x: 0, y: 0} if omitted)
 *
 * Returns a new Definition (immutable — never mutates the input).
 */

import type { Definition, Position } from "./definition";
import type { DefinitionResult } from "./definitionResult";
import type { NodeTypeName } from "./nodeTypes";
import { NODE_TYPE_INFO } from "./nodeTypes";
import { NODE_SCHEMAS } from "./schemas/registry";
import { validateDefinition } from "./validate";

/** Builds a default parameter object from the schema for a node type. */
function buildDefaultParams(nodeType: NodeTypeName): Record<string, unknown> {
  const schema = NODE_SCHEMAS[nodeType];
  if (!schema) return {};

  const params: Record<string, unknown> = {};
  for (const param of schema.parameters) {
    if (param.default !== undefined) {
      params[param.name] = param.default;
    }
  }
  return params;
}

/** Creates default ports for a node type based on its role. */
function buildDefaultPorts(nodeType: NodeTypeName) {
  const info = NODE_TYPE_INFO[nodeType];

  // Container nodes get an input port for receiving items
  if (info.isContainer) {
    return {
      inputPorts: [{ id: "in-1", name: "input" }] as Definition["inputPorts"],
      outputPorts: [] as Definition["outputPorts"],
    };
  }

  // Non-container nodes get one output port by default
  return {
    inputPorts: [] as Definition["inputPorts"],
    outputPorts: [{ id: "out-1", name: "output" }] as Definition["outputPorts"],
  };
}

/**
 * Adds a new child node to the root group definition.
 *
 * The definition must be a container (group, loop, or parallel).
 * The new node is appended to the `nodes` array.
 */
export function addNode(
  definition: Definition,
  nodeType: NodeTypeName,
  position?: Position,
): DefinitionResult {
  const info = NODE_TYPE_INFO[nodeType];
  const ports = buildDefaultPorts(nodeType);

  const newNode: Definition = {
    id: crypto.randomUUID(),
    type: nodeType,
    version: "1.0.0",
    name: info.label,
    position: position ?? { x: 0, y: 0 },
    metadata: {},
    parameters: buildDefaultParams(nodeType),
    inputPorts: ports.inputPorts,
    outputPorts: ports.outputPorts,
    ...(info.isContainer ? { nodes: [], edges: [] } : {}),
  };

  const updated: Definition = {
    ...definition,
    nodes: [...(definition.nodes ?? []), newNode],
  };

  return {
    definition: updated,
    errors: validateDefinition(updated),
  };
}
