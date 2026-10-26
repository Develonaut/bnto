/**
 * Tests for taskEntries. Uses node:test.
 * Run: pnpm exec tsx --test scripts/turbo/taskEntries.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { taskEntries } from "./taskEntries";

const FILES = [
  "eslint.config.mjs",
  "vitest.config.ts",
  "src/index.ts",
  "src/Button.tsx",
  "src/format.test.ts",
  "src/__tests__/integration/run.ts",
  "README.md",
  "src/styles.css",
];

describe("taskEntries", () => {
  it("starts lint from the ESLint config", () => {
    assert.deepEqual(taskEntries("lint", FILES), ["eslint.config.mjs"]);
  });

  it("starts test from every source but the ESLint config", () => {
    assert.deepEqual(taskEntries("test", FILES), [
      "vitest.config.ts",
      "src/index.ts",
      "src/Button.tsx",
      "src/format.test.ts",
      "src/__tests__/integration/run.ts",
    ]);
  });

  it("starts build from the sources that aren't tests", () => {
    assert.deepEqual(taskEntries("build", FILES), [
      "vitest.config.ts",
      "src/index.ts",
      "src/Button.tsx",
    ]);
  });

  it("gives lint nothing without an ESLint config", () => {
    assert.deepEqual(taskEntries("lint", ["src/index.ts"]), []);
  });
});
