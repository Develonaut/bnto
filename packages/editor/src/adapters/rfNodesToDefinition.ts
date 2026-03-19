/**
 * RF nodes → Definition adapter.
 *
 * Reads RF nodes (visual-only data) plus the configs map (domain data)
 * and reconstructs a Definition tree. Used at export boundaries only.
 *
 * When a stored `definition` is provided, container nodes preserve their
 * nested children. This ensures the full recursive structure round-trips
 * through export.
 *
 * Pure function — no React, no DOM, fully testable.
 */

import { type Definition, CURRENT_FORMAT_VERSION } from "@bnto/core";
import type { RecipeMetadata } from "../store/types";
import type { BentoNode, NodeConfigs } from "./types";
import { findDefinitionById } from "./findDefinitionById";

/**
 * Build a Definition from the current RF node state + configs.
 *
 * Accepts lightweight RecipeMetadata (id, name, slug) instead of
 * a full Definition — callers don't need to construct fake objects with
 * empty ports/position/metadata just to satisfy the type.
 *
 * @param rfNodes - Compartment nodes (visual-only data)
 * @param metadata - Root-level recipe metadata (id, name, slug)
 * @param configs - Domain data keyed by node ID
 * @param definition - Full nested definition for preserving container children and type/version
 * @returns Full Definition with child nodes
 */
function rfNodesToDefinition(
  rfNodes: BentoNode[],
  metadata: RecipeMetadata,
  configs: NodeConfigs = {},
  definition: Definition | null = null,
): Definition {
  // Only include top-level nodes in the export — child nodes are already
  // in the definition tree from write-through operations.
  const topLevelNodes = rfNodes.filter(
    (n) => !n.data.parentContainerId && (n.data.depth === undefined || n.data.depth === 0),
  );
  const children: Definition[] = topLevelNodes.map((rfNode) => {
    const config = configs[rfNode.id];

    // For container nodes, preserve nested children from definition
    const originalDef = definition ? findDefinitionById(definition, rfNode.id) : null;

    return {
      id: rfNode.id,
      type: config?.nodeType ?? "unknown",
      version: CURRENT_FORMAT_VERSION,
      name: config?.name ?? rfNode.data.label,
      position: rfNode.position,
      parameters: config?.parameters ?? {},
      metadata: originalDef?.metadata ?? {},
      inputPorts: originalDef?.inputPorts ?? [],
      outputPorts: originalDef?.outputPorts ?? [],
      ...(originalDef?.nodes ? { nodes: originalDef.nodes } : {}),
      ...(originalDef?.edges ? { edges: originalDef.edges } : {}),
      ...(originalDef?.fields ? { fields: originalDef.fields } : {}),
      ...(originalDef?.parentId ? { parentId: originalDef.parentId } : {}),
    };
  });

  const { cloudId: _, slug: __, ...rootMeta } = metadata;
  return {
    ...rootMeta,
    type: definition?.type ?? "group",
    version: definition?.version ?? CURRENT_FORMAT_VERSION,
    position: { x: 0, y: 0 },
    metadata: definition?.metadata ?? {},
    parameters: definition?.parameters ?? {},
    inputPorts: definition?.inputPorts ?? [],
    outputPorts: definition?.outputPorts ?? [],
    nodes: children,
    ...(definition?.edges ? { edges: definition.edges } : {}),
  };
}

export { rfNodesToDefinition };
