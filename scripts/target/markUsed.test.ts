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

// Times with a fraction of a millisecond, as filesystems keep them: a Date
// can't carry one, so setting a time from a Date would move it.
const OLD_SECONDS = OLD.getTime() / 1000 + 0.000_456_7;

function write(path: string) {
  const full = join(target, path);
  mkdirSync(join(full, ".."), { recursive: true });
  writeFileSync(full, "x");
  utimesSync(full, OLD_SECONDS, OLD_SECONDS);
}

function times(path: string) {
  const { atimeNs, mtimeNs } = statSync(join(target, path), { bigint: true });
  return { atimeNs, mtimeNs };
}

beforeEach(() => {
  target = mkdtempSync(join(tmpdir(), "mark-used-"));
  write(`debug/.fingerprint/x-${A}/lib-x`);
  write(`debug/.fingerprint/x-${A}/lib-x.json`);
  write(`debug/.fingerprint/x-${A}/invoked.timestamp`);
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

  it("touches only the fingerprint's .json, leaving every time cargo compares", () => {
    const compared = [
      `debug/deps/libx-${A}.rlib`,
      `debug/.fingerprint/x-${A}/lib-x`,
      `debug/.fingerprint/x-${A}/invoked.timestamp`,
    ];
    const before = compared.map(times);
    markUsed(targetEntries(target), NOW);
    assert.deepEqual(compared.map(times), before);
    assert.equal(
      statSync(join(target, `debug/.fingerprint/x-${A}/lib-x.json`)).atimeMs,
      NOW.getTime(),
    );
  });
});
