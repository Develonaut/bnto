/**
 * Definition fixture helpers — drift prevention for custom recipe journeys.
 *
 * Compares exported editor JSON against reference definition fixtures
 * in test-fixtures/definitions/. Catches recipe creation drift — when the
 * editor produces different node types, operations, or parameter values.
 */

import path from "path";
import fs from "fs";
import { expect } from "../fixtures";

// ---------------------------------------------------------------------------
// Reference fixture directory
// ---------------------------------------------------------------------------

export const DEFINITIONS_DIR = path.resolve(
  __dirname,
  "../../../../test-fixtures/definitions",
);

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/**
 * A single node spec from a reference definition fixture.
 *
 * - `type` and `operation` identify the node.
 * - `parameters` (optional) lists param keys/values the exported definition
 *   MUST contain — the export may have additional default params.
 */
interface RefNode {
  type: string;
  operation: string | null;
  parameters?: Record<string, unknown>;
}

/** Shape of a reference .bnto.json fixture file. */
interface RefDefinition {
  description: string;
  expectedNodeCount: number;
  nodes: RefNode[];
}

/** Exported definition node shape (from editor export). */
interface ExportedNode {
  type: string;
  parameters?: Record<string, unknown>;
}

// ---------------------------------------------------------------------------
// Assertion
// ---------------------------------------------------------------------------

/**
 * Assert that the exported definition structurally matches a reference fixture.
 *
 * Compares:
 * - Total node count
 * - For each reference node: a matching exported node exists with the same
 *   type, operation, and any specified parameter values.
 *
 * Ignores: node IDs, positions, ports, metadata, default params not in ref.
 */
export function assertDefinitionMatchesFixture(
  exportedJson: { nodes: ExportedNode[] },
  fixtureName: string,
) {
  const fixturePath = path.join(DEFINITIONS_DIR, fixtureName);
  const ref: RefDefinition = JSON.parse(fs.readFileSync(fixturePath, "utf-8"));

  // Node count must match
  expect(exportedJson.nodes.length).toBe(ref.expectedNodeCount);

  // Track which exported nodes have been matched (avoid double-matching)
  const matched = new Set<number>();

  for (const refNode of ref.nodes) {
    // Find an unmatched exported node with same type + operation
    const idx = exportedJson.nodes.findIndex((n, i) => {
      if (matched.has(i)) return false;
      if (n.type !== refNode.type) return false;
      const exportedOp = n.parameters?.operation ?? null;
      return exportedOp === refNode.operation;
    });

    expect(idx, `Missing node: type=${refNode.type} operation=${refNode.operation}`).not.toBe(-1);
    matched.add(idx);

    // Check specified parameters are present with correct values
    if (refNode.parameters) {
      const exported = exportedJson.nodes[idx];
      for (const [key, value] of Object.entries(refNode.parameters)) {
        expect(exported.parameters?.[key]).toBe(value);
      }
    }
  }
}
