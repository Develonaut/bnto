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
import { CURRENT_FORMAT_VERSION } from "./formatVersion";
import { NODE_SCHEMA_DEFS } from "./schemas/registry";
import { validateDefinition } from "./validate";

/** Unwrap Zod wrappers (ZodDefault, ZodOptional) to find the inner type. */
function unwrapZod(field: unknown): {
  typeName?: string;
  values?: unknown[];
  defaultValue?: () => unknown;
} {
  const def = (field as { _def?: Record<string, unknown> })?._def;
  if (!def) return {};
  const typeName = def.typeName as string | undefined;
  if (typeName === "ZodDefault") {
    const inner = unwrapZod(def.innerType);
    return { ...inner, defaultValue: def.defaultValue as (() => unknown) | undefined };
  }
  if (typeName === "ZodOptional" || typeName === "ZodNullable") {
    return unwrapZod(def.innerType);
  }
  return { typeName, values: def.values as unknown[] | undefined };
}

/** Builds default parameters from the Zod schema for a node type. */
function buildDefaultParams(nodeType: NodeTypeName): Record<string, unknown> {
  const schemaDef = NODE_SCHEMA_DEFS[nodeType];
  if (!schemaDef) return {};
  const result = schemaDef.schema.safeParse({});
  if (result.success) return { ...result.data };
  // Required fields missing — extract defaults and first enum values
  const shape = schemaDef.schema.shape as Record<string, unknown>;
  const params: Record<string, unknown> = {};
  for (const [name, field] of Object.entries(shape)) {
    const info = unwrapZod(field);
    if (info.defaultValue && typeof info.defaultValue === "function") {
      params[name] = info.defaultValue();
    } else if (info.typeName === "ZodEnum" && info.values?.length) {
      params[name] = info.values[0];
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
      inputPorts: [{ id: crypto.randomUUID(), name: "input" }] as Definition["inputPorts"],
      outputPorts: [] as Definition["outputPorts"],
    };
  }

  // Non-container nodes get one output port by default
  return {
    inputPorts: [] as Definition["inputPorts"],
    outputPorts: [{ id: crypto.randomUUID(), name: "output" }] as Definition["outputPorts"],
  };
}

/** Creates a new node definition with defaults from its type schema. */
function createNode(nodeType: NodeTypeName, position: Position): Definition {
  const info = NODE_TYPE_INFO[nodeType];
  const ports = buildDefaultPorts(nodeType);

  return {
    id: crypto.randomUUID(),
    type: nodeType,
    version: CURRENT_FORMAT_VERSION,
    name: info.label,
    position,
    metadata: {},
    parameters: buildDefaultParams(nodeType),
    inputPorts: ports.inputPorts,
    outputPorts: ports.outputPorts,
    ...(info.isContainer ? { nodes: [], edges: [] } : {}),
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
  const newNode = createNode(nodeType, position ?? { x: 0, y: 0 });
  const updated: Definition = {
    ...definition,
    nodes: [...(definition.nodes ?? []), newNode],
  };

  return {
    definition: updated,
    errors: validateDefinition(updated),
  };
}
