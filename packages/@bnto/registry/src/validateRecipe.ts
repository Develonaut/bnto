/**
 * Recipe-level validation — I/O checks, structural checks, connectivity.
 *
 * Returns an array of errors (never throws) so the UI can show all issues.
 * Combines structural definition validation with recipe-specific business rules.
 */

import { validateDefinition, isIoNodeType } from "@bnto/nodes";
import type { ValidationError } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import type { Recipe } from "./recipe";

function err(nodeId: string, field: string, message: string): ValidationError {
  return { nodeId, field, message };
}

function validateRootType(recipe: Recipe): ValidationError | null {
  const { definition: def, slug } = recipe;
  if (def.type !== "group") {
    return err(def.id, "type", `recipe '${slug}' root must be type 'group', got '${def.type}'`);
  }
  return null;
}

function validateRootNodes(recipe: Recipe): ValidationError | null {
  const { definition: def, slug } = recipe;
  if (!def.nodes?.length) {
    return err(def.id, "nodes", `recipe '${slug}' root must have a nodes array`);
  }
  return null;
}

function validateRootId(recipe: Recipe): ValidationError | null {
  const { definition: def, slug } = recipe;
  if (def.id !== slug) {
    return err(def.id, "id", `recipe '${slug}' definition ID '${def.id}' must match recipe slug`);
  }
  return null;
}

function validateRootShape(recipe: Recipe): ValidationError[] {
  return [validateRootType(recipe), validateRootNodes(recipe), validateRootId(recipe)].filter(
    (e): e is ValidationError => e !== null,
  );
}

/** Validates that exactly one node of the given type exists with the required ports. */
function validateIoNode(recipe: Recipe, ioType: "input" | "output"): ValidationError[] {
  const nodes = recipe.definition.nodes ?? [];
  const matches = nodes.filter((n) => n.type === ioType);
  const rootId = recipe.definition.id;

  if (matches.length !== 1) {
    const msg =
      matches.length === 0
        ? `recipe '${recipe.slug}' must have exactly one ${ioType} node`
        : `recipe '${recipe.slug}' has ${matches.length} ${ioType} nodes (expected 1)`;
    return [err(rootId, "nodes", msg)];
  }

  const node = matches[0]!;
  const portField = ioType === "input" ? "outputPorts" : "inputPorts";
  const ports = ioType === "input" ? node.outputPorts : node.inputPorts;
  if (!ports?.length) {
    return [
      err(
        node.id,
        portField,
        `${ioType} node '${node.id}' must have at least one ${portField.slice(0, -1)}`,
      ),
    ];
  }
  return [];
}

/** Builds an undirected adjacency map from edges for reachability checks. */
function buildAdjacency(
  nodes: Definition["nodes"],
  edges: Definition["edges"],
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
  nodes: Definition["nodes"],
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

/** Finds disconnected non-IO nodes unreachable from input. */
function findDisconnectedNodes(
  nodes: Definition["nodes"],
  visited: Set<string>,
): ValidationError[] {
  return (nodes ?? [])
    .filter((n) => !isIoNodeType(n.type) && !visited.has(n.id))
    .map((n) => err(n.id, "edges", `node '${n.id}' is disconnected (no edge path from input)`));
}

/** Checks if the output node is reachable from input. */
function checkOutputReachable(nodes: Definition["nodes"], visited: Set<string>): ValidationError[] {
  const outputNode = (nodes ?? []).find((n) => n.type === "output");
  if (outputNode && !visited.has(outputNode.id)) {
    return [
      err(outputNode.id, "edges", `output node '${outputNode.id}' is not reachable from input`),
    ];
  }
  return [];
}

/** Validates that edges form a path from input to output (no disconnected nodes). */
function validateConnectivity(recipe: Recipe): ValidationError[] {
  const nodes = recipe.definition.nodes ?? [];
  const edges = recipe.definition.edges ?? [];
  if (nodes.length < 2 || edges.length === 0) return [];

  const adjacency = buildAdjacency(nodes, edges);
  const visited = reachableFromInput(nodes, adjacency);

  return [...findDisconnectedNodes(nodes, visited), ...checkOutputReachable(nodes, visited)];
}

/**
 * Validates a recipe — structural definition checks + recipe-level business rules.
 *
 * Checks root shape, I/O nodes, connectivity, plus structural validation.
 */
export function validateRecipe(recipe: Recipe): ValidationError[] {
  return [
    ...validateDefinition(recipe.definition),
    ...validateRootShape(recipe),
    ...validateIoNode(recipe, "input"),
    ...validateIoNode(recipe, "output"),
    ...validateConnectivity(recipe),
  ];
}
