/**
 * Workflow validation — ported from Go engine/pkg/validator/omakase.go.
 *
 * Pure functions that validate `.bnto.json` definitions structurally.
 * Returns arrays of errors (never throws) so the UI can display all issues.
 *
 * Does NOT include preflight checks (shell PATH, file existence) —
 * those are environment-specific and belong in the execution layer.
 */

import type { Definition } from "./definition";
import { isNodeType } from "./nodeTypes";
import { TYPE_VALIDATORS } from "./validateTypeSpecific";

/** A single validation error with location and message. */
export interface ValidationError {
  /** The node ID where the error occurred. */
  nodeId: string;

  /** The field that failed validation (e.g., "type", "url", "mode"). */
  field: string;

  /** Human-readable error message. */
  message: string;
}

/** Validates core required fields: id, type, version. */
function validateCore(def: Definition): ValidationError[] {
  const errors: ValidationError[] = [];

  if (!def.id) {
    errors.push({ nodeId: "", field: "id", message: "node is missing required field 'id'" });
  }

  if (!def.type) {
    errors.push({
      nodeId: def.id || "",
      field: "type",
      message: `node '${def.id || ""}' is missing required field 'type'`,
    });
  }

  if (!def.version) {
    errors.push({
      nodeId: def.id || "",
      field: "version",
      message: `node '${def.id || ""}' is missing required field 'version'`,
    });
  }

  return errors;
}

/** Validates that the node type is a known registered type. */
function validateType(def: Definition): ValidationError[] {
  if (!def.type) return []; // Already caught by validateCore
  if (!isNodeType(def.type)) {
    return [
      {
        nodeId: def.id,
        field: "type",
        message: `node '${def.id}' has unknown type '${def.type}'`,
      },
    ];
  }
  return [];
}

/** Validates type-specific parameters (url, method, mode, etc.). */
function validateTypeSpecific(def: Definition): ValidationError[] {
  if (!def.type || !isNodeType(def.type)) return [];
  const validator = TYPE_VALIDATORS[def.type];
  if (!validator) return [];
  return validator(def);
}

/**
 * Validates that all edges reference existing child nodes.
 *
 * Checks that `source` and `target` on each edge map to a node ID
 * in the definition's `nodes` array.
 */
export function validateEdges(def: Definition): ValidationError[] {
  if (!def.edges?.length) return [];

  const nodeIds = new Set((def.nodes ?? []).map((n) => n.id));
  const errors: ValidationError[] = [];

  for (const edge of def.edges) {
    if (!nodeIds.has(edge.source)) {
      errors.push({
        nodeId: def.id,
        field: "edges",
        message: `edge '${edge.id}' in group '${def.id}' has invalid source '${edge.source}' (node doesn't exist)`,
      });
    }
    if (!nodeIds.has(edge.target)) {
      errors.push({
        nodeId: def.id,
        field: "edges",
        message: `edge '${edge.id}' in group '${def.id}' has invalid target '${edge.target}' (node doesn't exist)`,
      });
    }
  }

  return errors;
}

/** Validates child nodes recursively and edges for group/container nodes. */
function validateChildren(def: Definition): ValidationError[] {
  if (!def.nodes?.length) return [];

  const errors: ValidationError[] = [];

  for (const child of def.nodes) {
    const childErrors = validateDefinition(child);
    errors.push(...childErrors);
  }

  errors.push(...validateEdges(def));

  return errors;
}

/**
 * Validates a single node definition.
 *
 * Checks core fields (id, type, version), type validity,
 * type-specific parameters, and recursively validates children
 * for container nodes (group, loop, parallel).
 */
export function validateDefinition(def: Definition): ValidationError[] {
  const errors: ValidationError[] = [];

  // Core field validation
  errors.push(...validateCore(def));

  // Stop further validation if core fields are missing
  if (!def.id || !def.type) return errors;

  // Type must be known
  errors.push(...validateType(def));

  // Type-specific parameter validation
  errors.push(...validateTypeSpecific(def));

  // Recursive child validation for container nodes
  if (def.type === "group" || def.type === "loop" || def.type === "parallel") {
    errors.push(...validateChildren(def));
  }

  return errors;
}

/**
 * Validates an entire workflow definition tree.
 *
 * Top-level entry point — validates the root definition
 * and all nested children recursively.
 */
export function validateWorkflow(def: Definition): ValidationError[] {
  return validateDefinition(def);
}
