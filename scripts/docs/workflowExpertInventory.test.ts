/**
 * The /workflow-expert skill's node inventory matches the engine catalog
 * (ISS-17): each node with a processor has a row in its table with its label
 * and whether it runs in the browser, each row names one, and every other
 * catalog node type is mentioned in the section.
 * Run: pnpm exec tsx --test scripts/docs/workflowExpertInventory.test.ts
 */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { describe, it } from "node:test";

const SKILL = ".claude/skills/workflow-expert/SKILL.md";
const CATALOG = "engine/catalog.snapshot.json";

interface Catalog {
  nodeTypes: { name: string }[];
  processors: { nodeType: string; name: string; platforms: string[] }[];
}

const catalog = JSON.parse(readFileSync(CATALOG, "utf8")) as Catalog;

/** The skill's "Current Node Inventory" section. */
function inventorySection(): string {
  const skill = readFileSync(SKILL, "utf8");
  const start = skill.indexOf("### 1. Current Node Inventory");
  assert.notEqual(start, -1, `${SKILL} has no "### 1. Current Node Inventory" section`);
  const end = skill.indexOf("\n### ", start + 1);
  return skill.slice(start, end === -1 ? undefined : end);
}

/** Each table row as `type | label | web`. */
function tableRows(section: string): string[] {
  return [...section.matchAll(/^\| `([a-z-]+)` +\| (.+?) +\| (yes|no) +\|$/gm)]
    .map((m) => `${m[1]} | ${m[2]} | ${m[3]}`)
    .sort();
}

describe("workflow-expert node inventory", () => {
  const section = inventorySection();
  const expected = catalog.processors
    .map((p) => `${p.nodeType} | ${p.name} | ${p.platforms.includes("browser") ? "yes" : "no"}`)
    .sort();

  it("has one row per node with a processor, with its label and Web, and no others", () => {
    assert.deepEqual(tableRows(section), expected);
  });

  it("mentions every other node type in the catalog", () => {
    const missing = catalog.nodeTypes
      .map((t) => t.name)
      .filter((name) => !section.includes(`\`${name}\``));
    assert.deepEqual(missing, []);
  });
});
