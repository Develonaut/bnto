/**
 * Recipe-level validation — I/O checks, structural checks, connectivity.
 *
 * Returns an array of errors (never throws) so the UI can show all issues.
 * Combines structural definition validation with recipe-specific business rules.
 */

import { validateDefinition, isIoNodeType } from "@bnto/nodes";
import type { ValidationError } from "@bnto/nodes";
import type { Recipe } from "./recipe";

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

/** Validates that exactly one input node exists and has outputPorts. */
function validateInputNode(recipe: Recipe): ValidationError[] {
  const nodes = recipe.definition.nodes ?? [];
  const inputNodes = nodes.filter((n) => n.type === "input");

  if (inputNodes.length === 0) {
    return [
      {
        nodeId: recipe.definition.id,
        field: "nodes",
        message: `recipe '${recipe.slug}' must have exactly one input node`,
      },
    ];
  }
  if (inputNodes.length > 1) {
    return [
      {
        nodeId: recipe.definition.id,
        field: "nodes",
        message: `recipe '${recipe.slug}' has ${inputNodes.length} input nodes (expected 1)`,
      },
    ];
  }
  const input = inputNodes[0]!;
  if (!input.outputPorts?.length) {
    return [
      {
        nodeId: input.id,
        field: "outputPorts",
        message: `input node '${input.id}' must have at least one outputPort`,
      },
    ];
  }
  return [];
}

/** Validates that exactly one output node exists and has inputPorts. */
function validateOutputNode(recipe: Recipe): ValidationError[] {
  const nodes = recipe.definition.nodes ?? [];
  const outputNodes = nodes.filter((n) => n.type === "output");

  if (outputNodes.length === 0) {
    return [
      {
        nodeId: recipe.definition.id,
        field: "nodes",
        message: `recipe '${recipe.slug}' must have exactly one output node`,
      },
    ];
  }
  if (outputNodes.length > 1) {
    return [
      {
        nodeId: recipe.definition.id,
        field: "nodes",
        message: `recipe '${recipe.slug}' has ${outputNodes.length} output nodes (expected 1)`,
      },
    ];
  }
  const output = outputNodes[0]!;
  if (!output.inputPorts?.length) {
    return [
      {
        nodeId: output.id,
        field: "inputPorts",
        message: `output node '${output.id}' must have at least one inputPort`,
      },
    ];
  }
  return [];
}

/** Builds an undirected adjacency map from edges for reachability checks. */
function buildAdjacency(
  nodes: Recipe["definition"]["nodes"],
  edges: Recipe["definition"]["edges"],
): Map<string, Set<string>> {
  const adjacency = new Map<string, Set<string>>();
  for (const node of nodes ?? []) {
    adjacency.set(node.id, new Set());
  }
  for (const edge of edges ?? []) {
    adjacency.get(edge.source)?.add(edge.target);
    adjacency.get(edge.target)?.add(edge.source);
  }
  return adjacency;
}

/** BFS from the input node, returning all reachable node IDs. */
function reachableFromInput(
  nodes: Recipe["definition"]["nodes"],
  adjacency: Map<string, Set<string>>,
): Set<string> {
  const inputNode = (nodes ?? []).find((n) => n.type === "input");
  if (!inputNode) return new Set();

  const visited = new Set<string>();
  const queue = [inputNode.id];
  while (queue.length > 0) {
    const current = queue.shift()!;
    if (visited.has(current)) continue;
    visited.add(current);
    for (const neighbor of adjacency.get(current) ?? []) {
      if (!visited.has(neighbor)) queue.push(neighbor);
    }
  }
  return visited;
}

/** Validates that edges form a path from input to output (no disconnected nodes). */
function validateConnectivity(recipe: Recipe): ValidationError[] {
  const nodes = recipe.definition.nodes ?? [];
  const edges = recipe.definition.edges ?? [];
  if (nodes.length < 2 || edges.length === 0) return [];

  const adjacency = buildAdjacency(nodes, edges);
  const visited = reachableFromInput(nodes, adjacency);

  const errors: ValidationError[] = [];
  for (const node of nodes) {
    if (!isIoNodeType(node.type) && !visited.has(node.id)) {
      errors.push({
        nodeId: node.id,
        field: "edges",
        message: `node '${node.id}' is disconnected (no edge path from input)`,
      });
    }
  }
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
  return [
    ...validateDefinition(recipe.definition),
    ...validateRootShape(recipe),
    ...validateInputNode(recipe),
    ...validateOutputNode(recipe),
    ...validateConnectivity(recipe),
  ];
}
