/**
 * RF nodes → Definition adapter.
 *
 * Reads RF nodes (visual-only data) plus the configs map (domain data)
 * and reconstructs a Definition tree. Used at export boundaries only.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import { type Definition, CURRENT_FORMAT_VERSION } from "@bnto/core";
import type { RecipeMetadata } from "../store/types";
import type { BentoNode, NodeConfigs } from "./types";
import { findDefinitionById } from "./findDefinitionById";

/** Filter to only top-level nodes (no parent, depth 0 or undefined). */
function filterTopLevelNodes(rfNodes: BentoNode[]): BentoNode[] {
  return rfNodes.filter(
    (n) => !n.data.parentContainerId && (n.data.depth === undefined || n.data.depth === 0),
  );
}

/** Carry over optional Definition fields that exist on the original. */
function carryOverOptionalFields(originalDef: Definition | null): Partial<Definition> {
  if (!originalDef) return {};
  return {
    ...(originalDef.nodes ? { nodes: originalDef.nodes } : {}),
    ...(originalDef.edges ? { edges: originalDef.edges } : {}),
    ...(originalDef.fields ? { fields: originalDef.fields } : {}),
    ...(originalDef.parentId ? { parentId: originalDef.parentId } : {}),
  };
}

/** Resolve config-driven fields for a child node definition. */
function resolveConfigFields(
  config: NodeConfigs[string] | undefined,
  label: string,
): Pick<Definition, "type" | "name" | "parameters"> {
  return {
    type: config?.nodeType ?? "unknown",
    name: config?.name ?? label,
    parameters: config?.parameters ?? {},
  };
}

/** Resolve fields that come from the original definition (ports, metadata). */
function resolveOriginalFields(
  originalDef: Definition | null,
): Pick<Definition, "metadata" | "inputPorts" | "outputPorts"> {
  return {
    metadata: originalDef?.metadata ?? {},
    inputPorts: originalDef?.inputPorts ?? [],
    outputPorts: originalDef?.outputPorts ?? [],
  };
}

/** Convert a single RF node + config into a child Definition. */
function mapNodeToDefinition(
  rfNode: BentoNode,
  configs: NodeConfigs,
  definition: Definition | null,
): Definition {
  const originalDef = definition ? findDefinitionById(definition, rfNode.id) : null;

  return {
    id: rfNode.id,
    version: CURRENT_FORMAT_VERSION,
    position: rfNode.position,
    ...resolveConfigFields(configs[rfNode.id], rfNode.data.label),
    ...resolveOriginalFields(originalDef),
    ...carryOverOptionalFields(originalDef),
  };
}

/** Carry over optional root-level Definition fields. */
function carryOverRootFields(definition: Definition | null): Partial<Definition> {
  if (!definition) return {};
  return {
    ...(definition.edges ? { edges: definition.edges } : {}),
    ...(definition.settings ? { settings: definition.settings } : {}),
  };
}

/** Resolve root type and version from existing definition or defaults. */
function resolveRootTypeAndVersion(
  definition: Definition | null,
): Pick<Definition, "type" | "version" | "parameters"> {
  return {
    type: definition?.type ?? "group",
    version: definition?.version ?? CURRENT_FORMAT_VERSION,
    parameters: definition?.parameters ?? {},
  };
}

/** Build the root Definition shell from metadata + optional existing definition. */
function buildRootDefinition(
  metadata: RecipeMetadata,
  children: Definition[],
  definition: Definition | null,
): Definition {
  const { slug: _, ...rootMeta } = metadata;
  return {
    ...rootMeta,
    ...resolveRootTypeAndVersion(definition),
    ...resolveOriginalFields(definition),
    position: { x: 0, y: 0 },
    nodes: children,
    ...carryOverRootFields(definition),
  };
}

function rfNodesToDefinition(
  rfNodes: BentoNode[],
  metadata: RecipeMetadata,
  configs: NodeConfigs = {},
  definition: Definition | null = null,
): Definition {
  const topLevelNodes = filterTopLevelNodes(rfNodes);
  const children = topLevelNodes.map((n) => mapNodeToDefinition(n, configs, definition));
  return buildRootDefinition(metadata, children, definition);
}

export { rfNodesToDefinition };
