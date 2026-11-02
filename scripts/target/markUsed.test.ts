/**
 * Tests for markUsed, on a fixture target folder. Uses node:test.
 * Run: pnpm exec tsx --test scripts/target/markUsed.test.ts
 */

import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, statSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, it } from "node:test";
import { markUsed } from "./markUsed.ts";
import { staleEntries } from "./staleEntries.ts";
import { targetEntries } from "./targetEntries.ts";

const A = "aaaaaaaaaaaaaaaa";
const OLD = new Date("2026-01-01T00:00:00Z");
const NOW = new Date("2026-03-01T00:00:00Z");

let target = "";

function write(path: string) {
  const full = join(target, path);
  mkdirSync(join(full, ".."), { recursive: true });
  writeFileSync(full, "x");
  utimesSync(full, OLD, OLD);
}

beforeEach(() => {
  target = mkdtempSync(join(tmpdir(), "mark-used-"));
  write(`debug/.fingerprint/x-${A}/lib-x`);
  write(`debug/deps/libx-${A}.rlib`);
});

afterEach(() => {
  rmSync(target, { recursive: true, force: true });
});

describe("markUsed", () => {
  it("makes a unit built long ago count as used now", () => {
    markUsed(targetEntries(target), NOW);
    assert.deepEqual(staleEntries(targetEntries(target), NOW.getTime(), 7), []);
  });

  it("leaves modification times alone, which cargo's freshness checks read", () => {
    markUsed(targetEntries(target), NOW);
    const rlib = statSync(join(target, `debug/deps/libx-${A}.rlib`));
    const fingerprint = statSync(join(target, `debug/.fingerprint/x-${A}/lib-x`));
    assert.equal(rlib.mtimeMs, OLD.getTime());
    assert.equal(fingerprint.mtimeMs, OLD.getTime());
  });
});
