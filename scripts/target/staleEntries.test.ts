/**
 * Tests for staleEntries. Uses node:test.
 * Run: pnpm exec tsx --test scripts/target/staleEntries.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { staleEntries } from "./staleEntries.ts";
import type { TargetEntry } from "./targetEntries.ts";

const DAY = 24 * 60 * 60 * 1000;
const NOW = 100 * DAY;

function entry(path: string, key: string, daysAgo: number): TargetEntry {
  return { path, key, lastUsed: NOW - daysAgo * DAY, bytes: 10 };
}

function paths(entries: TargetEntry[]): string[] {
  return staleEntries(entries, NOW, 7).map((e) => e.path);
}

describe("staleEntries", () => {
  it("takes every entry of a unit no file of which was used in the last week", () => {
    const old = [entry("deps/libx-a.rlib", "a", 9), entry(".fingerprint/x-a", "a", 8)];
    assert.deepEqual(paths(old), ["deps/libx-a.rlib", ".fingerprint/x-a"]);
  });

  it("keeps a whole unit when any of its files was used this week", () => {
    // The fingerprint cargo reads on every build keeps the unit's old objects.
    const unit = [entry("deps/x-a.o", "a", 30), entry(".fingerprint/x-a", "a", 1)];
    assert.deepEqual(paths(unit), []);
  });

  it("judges each unit by its own files", () => {
    const entries = [entry("deps/x-a", "a", 1), entry("deps/x-b", "b", 8)];
    assert.deepEqual(paths(entries), ["deps/x-b"]);
  });

  it("keeps an entry used exactly the given number of days ago", () => {
    assert.deepEqual(paths([entry("deps/x-a", "a", 7)]), []);
  });
});
