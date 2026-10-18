/**
 * Finds one cycle in a level's edge graph, mirroring the engine's
 * topological check (Kahn's algorithm). Returns the cycle as a closed
 * path (`["a", "b", "a"]`), or `null` when the graph is acyclic.
 */

import type { Edge } from "./definition";

/** Nodes Kahn's algorithm can't order: on a cycle or downstream of one. */
function unorderedNodes(nodeIds: readonly string[], edges: readonly Edge[]): Set<string> {
  const inDegree = new Map(nodeIds.map((id) => [id, 0]));
  for (const e of edges) inDegree.set(e.target, (inDegree.get(e.target) ?? 0) + 1);
  const ready = nodeIds.filter((id) => inDegree.get(id) === 0);
  const remaining = new Set(nodeIds);
  // for-of picks up ids pushed during the walk.
  for (const id of ready) {
    remaining.delete(id);
    for (const e of edges.filter((x) => x.source === id)) {
      const left = (inDegree.get(e.target) ?? 0) - 1;
      inDegree.set(e.target, left);
      if (left === 0) ready.push(e.target);
    }
  }
  return remaining;
}

/**
 * Walks predecessors from `start` (every unordered node has an unordered
 * predecessor) until a node repeats; the repeated stretch is a cycle.
 * Returns it in edge direction, without the closing repeat.
 */
function walkToCycle(start: string, edges: readonly Edge[], remaining: Set<string>): string[] {
  const seen: string[] = [];
  let current = start;
  while (!seen.includes(current)) {
    seen.push(current);
    const into = edges.find((e) => e.target === current && remaining.has(e.source));
    current = into?.source ?? current;
  }
  return seen.slice(seen.indexOf(current)).reverse();
}

/** Rotates a cycle to start at its node listed first in `nodeIds`, then closes it. */
function closeFromFirst(cycle: string[], nodeIds: readonly string[]): string[] {
  const at = Math.max(0, cycle.indexOf(nodeIds.find((id) => cycle.includes(id)) ?? ""));
  const rotated = [...cycle.slice(at), ...cycle.slice(0, at)];
  return [...rotated, ...rotated.slice(0, 1)];
}

/** Finds one cycle among `nodeIds` (edges must only reference these ids). */
export function findEdgeCycle(nodeIds: readonly string[], edges: readonly Edge[]): string[] | null {
  const remaining = unorderedNodes(nodeIds, edges);
  const start = nodeIds.find((id) => remaining.has(id));
  if (start === undefined) return null;
  return closeFromFirst(walkToCycle(start, edges, remaining), nodeIds);
}
