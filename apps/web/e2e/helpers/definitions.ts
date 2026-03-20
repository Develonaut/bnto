/**
 * Definition fixture helpers — drift prevention for custom recipe journeys.
 *
 * Compares exported editor JSON against reference definition fixtures
 * in @bnto/registry/src/recipes/generated/. Both use the same .bnto.json
 * Definition format — same schema as engine recipe fixtures.
 */

import path from "path";
import fs from "fs";
import { expect } from "../fixtures";

// ---------------------------------------------------------------------------
// Reference fixture directory
// ---------------------------------------------------------------------------

export const DEFINITIONS_DIR = path.resolve(
  __dirname,
  "../../../../packages/@bnto/registry/src/recipes/generated",
);

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/** A node within a Definition (both exported and reference). */
interface DefinitionNode {
  type: string;
  parameters?: Record<string, unknown>;
  nodes?: DefinitionNode[];
}

/** Root Definition shape (.bnto.json). */
interface Definition {
  nodes: DefinitionNode[];
}

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/** Node types that are I/O infrastructure, not part of the processing pipeline. */
const IO_NODE_TYPES = new Set(["input", "output"]);

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/**
 * Recursively collect processor nodes from a Definition tree.
 * Flattens container nodes (group, loop) and excludes I/O nodes (input, output)
 * since those are editor infrastructure with different defaults than fixtures.
 */
function collectProcessorNodes(nodes: DefinitionNode[]): DefinitionNode[] {
  const result: DefinitionNode[] = [];
  for (const node of nodes) {
    if (node.nodes && node.nodes.length > 0 && (node.type === "group" || node.type === "loop")) {
      result.push(...collectProcessorNodes(node.nodes));
    } else if (!IO_NODE_TYPES.has(node.type)) {
      result.push(node);
    }
  }
  return result;
}

// ---------------------------------------------------------------------------
// Assertion
// ---------------------------------------------------------------------------

/**
 * Assert that the exported definition structurally matches a reference fixture.
 *
 * Both are real .bnto.json Definition objects. Compares:
 * - Processor node count (after flattening containers, excluding I/O nodes)
 * - For each reference processor: a matching exported node exists with the same
 *   type and any specified parameter values.
 *
 * Ignores: I/O nodes (input/output), node IDs, positions, ports, metadata,
 * edges, default params not in the reference.
 */
export function assertDefinitionMatchesFixture(exportedJson: Definition, fixtureName: string) {
  const fixturePath = path.join(DEFINITIONS_DIR, fixtureName);
  const ref: Definition = JSON.parse(fs.readFileSync(fixturePath, "utf-8"));

  const exportedNodes = collectProcessorNodes(exportedJson.nodes);
  const refNodes = collectProcessorNodes(ref.nodes);

  // Processor node count must match
  expect(exportedNodes.length).toBe(refNodes.length);

  // Track which exported nodes have been matched
  const matched = new Set<number>();

  for (const refNode of refNodes) {
    // Find an unmatched exported node with the same type
    const idx = exportedNodes.findIndex((n, i) => {
      if (matched.has(i)) return false;
      return n.type === refNode.type;
    });

    expect(idx, `Missing node: type=${refNode.type}`).not.toBe(-1);
    matched.add(idx);

    // Check specified parameters are present with correct values
    if (refNode.parameters) {
      const exported = exportedNodes[idx];
      for (const [key, value] of Object.entries(refNode.parameters)) {
        if (value !== undefined && value !== null && typeof value !== "object") {
          expect(exported.parameters?.[key]).toBe(value);
        }
      }
    }
  }
}
