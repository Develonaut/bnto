/**
 * Tests for check-crate-publish-list.sh, run against a fixture workspace.
 * Run: pnpm exec tsx --test scripts/check-crate-publish-list.test.ts
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { describe, it } from "node:test";

const SCRIPT = resolve("scripts/check-crate-publish-list.sh");

function write(root: string, file: string, content: string) {
  mkdirSync(join(root, file, ".."), { recursive: true });
  writeFileSync(join(root, file), content);
}

// A workspace with crates `a` and `b`, and a release workflow that publishes `c`.
function fixture(): string {
  const root = mkdtempSync(join(tmpdir(), "publish-list-"));
  write(root, "engine/Cargo.toml", '[workspace]\nmembers = [\n  "crates/a",\n  "crates/b",\n]\n');
  write(root, "engine/crates/a/Cargo.toml", '[package]\nname = "a"\n');
  write(root, "engine/crates/b/Cargo.toml", '[package]\nname = "b"\n');
  write(root, ".github/workflows/release.yml", "jobs:\n  steps:\n    run: |\n      publish c\n");
  return root;
}

describe("check-crate-publish-list.sh", () => {
  it("reports every mismatch, not just the first", () => {
    const root = fixture();
    const result = spawnSync("bash", [SCRIPT], { cwd: root, encoding: "utf-8" });
    rmSync(root, { recursive: true, force: true });
    assert.equal(result.status, 1);
    assert.match(result.stdout, /MISSING from release workflow: a/);
    assert.match(result.stdout, /MISSING from release workflow: b/);
    assert.match(result.stdout, /STALE in release workflow: c/);
    assert.match(result.stdout, /Fix \.github\/workflows\/release\.yml/);
  });
});
