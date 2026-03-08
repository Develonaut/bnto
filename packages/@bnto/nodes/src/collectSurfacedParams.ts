/**
 * collectSurfacedParams — walk container children, find leaf nodes,
 * and surface their user-facing parameters.
 *
 * Pure function. No React, no DOM, fully testable.
 *
 * Used by the editor when a user clicks a composite group node
 * (e.g., "Batch Compress"). Instead of showing the group's own params
 * (mode: sequential/parallel), the config panel surfaces the leaf
 * node params (quality, format, width) that users actually care about.
 */

import type { Definition, SurfacedParamsConfig } from "./definition";
import type { NodeSchemaDefinition } from "./schemas/types";
import { isContainerNodeType } from "./isContainerNodeType";
import { getNodeSchema } from "./schemas";

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** A group of surfaced parameters from one leaf node. */
export interface SurfacedGroup {
  /** The leaf node's ID (for write-through updates). */
  leafNodeId: string;
  /** Display label for the group header. */
  label: string;
  /** The leaf node's type (e.g., "image", "spreadsheet"). */
  nodeType: string;
  /** The leaf node's full schema definition. */
  schema: NodeSchemaDefinition;
  /** Current parameter values from the leaf node. */
  values: Record<string, unknown>;
  /** Parameter names visible in this surfaced group. */
  visibleParams: string[];
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/** Recursively collect all leaf (non-container) nodes from a definition tree. */
function collectLeaves(def: Definition): Definition[] {
  const leaves: Definition[] = [];
  for (const child of def.nodes ?? []) {
    if (isContainerNodeType(child.type)) {
      leaves.push(...collectLeaves(child));
    } else {
      leaves.push(child);
    }
  }
  return leaves;
}

/**
 * Determine which params from a leaf node should be surfaced.
 *
 * Filters by:
 * 1. `surfaceable` flag in the schema (engine declares which params are eligible)
 * 2. `hidden` flag (hidden params are never surfaced)
 * 3. `operation` is excluded (it's the operation selector, not a user knob)
 */
function getSurfaceableParams(schema: NodeSchemaDefinition): string[] {
  return Object.entries(schema.params)
    .filter(([name, meta]) => {
      if (name === "operation") return false;
      if (meta.hidden) return false;
      if (meta.surfaceable === false) return false;
      return true;
    })
    .map(([name]) => name);
}

/** Apply include/exclude overrides from SurfacedParamsConfig. */
function applyOverrides(
  groups: SurfacedGroup[],
  config: SurfacedParamsConfig | undefined,
): SurfacedGroup[] {
  if (!config) return groups;

  if (config.mode === "manual" && config.include) {
    const includeMap = new Map(config.include.map((i) => [i.nodeId, i.params]));
    return groups
      .filter((g) => includeMap.has(g.leafNodeId))
      .map((g) => {
        const allowedParams = includeMap.get(g.leafNodeId);
        if (!allowedParams) return g;
        return { ...g, visibleParams: g.visibleParams.filter((p) => allowedParams.includes(p)) };
      })
      .filter((g) => g.visibleParams.length > 0);
  }

  if (config.mode === "auto" && config.exclude) {
    const excludeMap = new Map(config.exclude.map((e) => [e.nodeId, new Set(e.params)]));
    return groups
      .map((g) => {
        const excluded = excludeMap.get(g.leafNodeId);
        if (!excluded) return g;
        return { ...g, visibleParams: g.visibleParams.filter((p) => !excluded.has(p)) };
      })
      .filter((g) => g.visibleParams.length > 0);
  }

  return groups;
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Walk a container definition's children, find leaf nodes, and return
 * their surfaceable parameters grouped by leaf node.
 *
 * @param containerDef - The container (group/loop/parallel) definition to walk
 * @returns Array of surfaced parameter groups, one per leaf node with visible params
 */
export function collectSurfacedParams(containerDef: Definition): SurfacedGroup[] {
  if (!isContainerNodeType(containerDef.type)) return [];

  const leaves = collectLeaves(containerDef);
  const groups: SurfacedGroup[] = [];

  for (const leaf of leaves) {
    const schema = getNodeSchema(leaf.type);
    if (!schema) continue;

    const surfaceableParams = getSurfaceableParams(schema);
    if (surfaceableParams.length === 0) continue;

    // Filter to only params visible given current leaf values
    const visibleParams = surfaceableParams.filter((paramName) => {
      const meta = schema.params[paramName];
      if (!meta?.visibleWhen) return true;

      const condition = meta.visibleWhen;
      if (Array.isArray(condition)) {
        return condition.some((c) => leaf.parameters[c.param] === c.equals);
      }
      return leaf.parameters[condition.param] === condition.equals;
    });

    if (visibleParams.length === 0) continue;

    const displayName = leaf.metadata?.customData?.displayName;
    groups.push({
      leafNodeId: leaf.id,
      label: displayName ?? leaf.name,
      nodeType: leaf.type,
      schema,
      values: leaf.parameters,
      visibleParams,
    });
  }

  return applyOverrides(groups, containerDef.surfacedParams);
}
