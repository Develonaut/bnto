/**
 * Graph rules for one level's edges (the root, or one container's children),
 * mirroring what the engine refuses before a run: duplicate edges, edges into
 * an `input` node or out of an `output` node, cycles (self-loops included),
 * and nodes that touch no edge. Edges naming unknown nodes are reported by
 * `validateEdges` and left out of these checks.
 */

import type { Definition, Edge } from "./definition";
import type { ValidationError } from "./validationError";
import { NODE_TYPES } from "./generated/catalog";
import { findEdgeCycle } from "./findEdgeCycle";

function edgeError(level: Definition, message: string): ValidationError {
  return { nodeId: level.id, field: "edges", message };
}

/** Refuses a second edge with the same source and target; keeps the first. */
function splitDuplicates(level: Definition, edges: readonly Edge[]) {
  const firstByPair = new Map<string, Edge>();
  const errors: ValidationError[] = [];
  for (const e of edges) {
    const first = firstByPair.get(`${e.source}\u0000${e.target}`);
    if (!first) firstByPair.set(`${e.source}\u0000${e.target}`, e);
    else {
      const pair = `${e.source} → ${e.target}`;
      const message = `edge '${e.id}' in group '${level.id}' duplicates edge '${first.id}' (${pair})`;
      errors.push(edgeError(level, message));
    }
  }
  return { unique: [...firstByPair.values()], errors };
}

/** Refuses edges that point into an `input` node or leave an `output` node. */
function checkIoDirection(level: Definition, edges: readonly Edge[], types: Map<string, string>) {
  const at = `in group '${level.id}'`;
  return edges
    .flatMap((e) => [
      ...(types.get(e.target) === NODE_TYPES.input
        ? [
            `edge '${e.id}' ${at} points into input node '${e.target}' (nothing flows into an input)`,
          ]
        : []),
      ...(types.get(e.source) === NODE_TYPES.output
        ? [`edge '${e.id}' ${at} leaves output node '${e.source}' (nothing flows out of an output)`]
        : []),
    ])
    .map((message) => edgeError(level, message));
}

function checkCycle(level: Definition, ids: string[], edges: readonly Edge[]) {
  const cycle = findEdgeCycle(ids, edges);
  if (!cycle) return [];
  return [edgeError(level, `edges in group '${level.id}' form a cycle: ${cycle.join(" → ")}`)];
}

/** Refuses processing and `output` nodes that no edge touches (an `input` may stand alone). */
function checkDisconnected(level: Definition, nodes: Definition[], edges: readonly Edge[]) {
  const touched = new Set(edges.flatMap((e) => [e.source, e.target]));
  return nodes
    .filter((n) => n.type !== NODE_TYPES.input && !touched.has(n.id))
    .map((n) =>
      edgeError(
        level,
        `node '${n.id}' in group '${level.id}' touches no edge (connect it or remove it)`,
      ),
    );
}

/**
 * Validates the graph a level's edges form. Applies only when the level has
 * edges — a level without edges runs as a linear chain in node order.
 */
export function validateEdgeGraph(level: Definition): ValidationError[] {
  const nodes = level.nodes ?? [];
  const allEdges = level.edges ?? [];
  if (!allEdges.length) return [];
  const types = new Map(nodes.map((n) => [n.id, n.type]));
  const known = allEdges.filter((e) => types.has(e.source) && types.has(e.target));
  const { unique, errors } = splitDuplicates(level, known);
  return [
    ...errors,
    ...checkIoDirection(level, unique, types),
    ...checkCycle(level, [...types.keys()], unique),
    ...checkDisconnected(level, nodes, allEdges),
  ];
}
