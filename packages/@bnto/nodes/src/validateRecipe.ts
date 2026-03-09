/**
 * Recipe-level validation — I/O checks, structural checks, connectivity.
 *
 * Returns an array of errors (never throws) so the UI can show all issues.
 * Combines structural definition validation with recipe-specific business rules.
 */

import type { Recipe } from "./recipe";
import type { ValidationError } from "./validationError";
import { validateDefinition } from "./validate";
import { isIoNodeType } from "./isIoNodeType";

/** Validates that the root definition has the expected shape for a recipe. */
function validateRootShape(recipe: Recipe): ValidationError[] {
  const errors: ValidationError[] = [];
  const def = recipe.definition;

  if (def.type !== "group") {
    errors.push({
      nodeId: def.id,
      field: "type",
      message: `recipe '${recipe.slug}' root definition must be type 'group', got '${def.type}'`,
    });
  }

  if (!def.nodes?.length) {
    errors.push({
      nodeId: def.id,
      field: "nodes",
      message: `recipe '${recipe.slug}' root definition must have a nodes array`,
    });
  }

  if (def.id !== recipe.slug) {
    errors.push({
      nodeId: def.id,
      field: "id",
      message: `recipe '${recipe.slug}' definition ID '${def.id}' must match recipe slug`,
    });
  }

  return errors;
}

/** Validates I/O node presence and configuration at the root level. */
function validateIoNodes(recipe: Recipe): ValidationError[] {
  const errors: ValidationError[] = [];
  const nodes = recipe.definition.nodes ?? [];

  const inputNodes = nodes.filter((n) => n.type === "input");
  const outputNodes = nodes.filter((n) => n.type === "output");

  if (inputNodes.length === 0) {
    errors.push({
      nodeId: recipe.definition.id,
      field: "nodes",
      message: `recipe '${recipe.slug}' must have exactly one input node`,
    });
  } else if (inputNodes.length > 1) {
    errors.push({
      nodeId: recipe.definition.id,
      field: "nodes",
      message: `recipe '${recipe.slug}' has ${inputNodes.length} input nodes (expected 1)`,
    });
  } else {
    const input = inputNodes[0]!;
    if (!input.outputPorts?.length) {
      errors.push({
        nodeId: input.id,
        field: "outputPorts",
        message: `input node '${input.id}' must have at least one outputPort`,
      });
    }
  }

  if (outputNodes.length === 0) {
    errors.push({
      nodeId: recipe.definition.id,
      field: "nodes",
      message: `recipe '${recipe.slug}' must have exactly one output node`,
    });
  } else if (outputNodes.length > 1) {
    errors.push({
      nodeId: recipe.definition.id,
      field: "nodes",
      message: `recipe '${recipe.slug}' has ${outputNodes.length} output nodes (expected 1)`,
    });
  } else {
    const output = outputNodes[0]!;
    if (!output.inputPorts?.length) {
      errors.push({
        nodeId: output.id,
        field: "inputPorts",
        message: `output node '${output.id}' must have at least one inputPort`,
      });
    }
  }

  return errors;
}

/** Validates that edges form a path from input to output (no disconnected nodes). */
function validateConnectivity(recipe: Recipe): ValidationError[] {
  const errors: ValidationError[] = [];
  const nodes = recipe.definition.nodes ?? [];
  const edges = recipe.definition.edges ?? [];

  if (nodes.length < 2 || edges.length === 0) return errors;

  // Build adjacency list (undirected for reachability check)
  const adjacency = new Map<string, Set<string>>();
  for (const node of nodes) {
    adjacency.set(node.id, new Set());
  }
  for (const edge of edges) {
    adjacency.get(edge.source)?.add(edge.target);
    adjacency.get(edge.target)?.add(edge.source);
  }

  // BFS from input node
  const inputNode = nodes.find((n) => n.type === "input");
  if (!inputNode) return errors; // Already caught by I/O validation

  const visited = new Set<string>();
  const queue = [inputNode.id];
  while (queue.length > 0) {
    const current = queue.shift()!;
    if (visited.has(current)) continue;
    visited.add(current);
    const neighbors = adjacency.get(current);
    if (neighbors) {
      for (const neighbor of neighbors) {
        if (!visited.has(neighbor)) queue.push(neighbor);
      }
    }
  }

  // Check that all non-I/O nodes are reachable
  for (const node of nodes) {
    if (!isIoNodeType(node.type) && !visited.has(node.id)) {
      errors.push({
        nodeId: node.id,
        field: "edges",
        message: `node '${node.id}' is disconnected (no edge path from input)`,
      });
    }
  }

  // Check output is reachable
  const outputNode = nodes.find((n) => n.type === "output");
  if (outputNode && !visited.has(outputNode.id)) {
    errors.push({
      nodeId: outputNode.id,
      field: "edges",
      message: `output node '${outputNode.id}' is not reachable from input`,
    });
  }

  return errors;
}

/**
 * Validates a recipe — structural definition checks + recipe-level business rules.
 *
 * Checks:
 * - Root definition must be type "group" with a nodes array
 * - Definition ID must match recipe slug
 * - Exactly one input node with outputPorts
 * - Exactly one output node with inputPorts
 * - All nodes must be connected (reachable from input)
 * - Plus all structural validation from `validateDefinition()`
 */
export function validateRecipe(recipe: Recipe): ValidationError[] {
  const structuralErrors = validateDefinition(recipe.definition);
  const rootErrors = validateRootShape(recipe);
  const ioErrors = validateIoNodes(recipe);
  const connectivityErrors = validateConnectivity(recipe);

  return [...structuralErrors, ...rootErrors, ...ioErrors, ...connectivityErrors];
}
