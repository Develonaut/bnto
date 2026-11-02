/**
 * Tests for targetEntries, on a fixture target folder. Uses node:test.
 * Run: pnpm exec tsx --test scripts/target/targetEntries.test.ts
 */

import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative } from "node:path";
import { afterEach, beforeEach, describe, it } from "node:test";
import { targetEntries } from "./targetEntries.ts";

const A = "aaaaaaaaaaaaaaaa";
const OLD = new Date("2026-01-01T00:00:00Z");
const NEW = new Date("2026-02-01T00:00:00Z");

let target = "";

function write(path: string, bytes: number, time = OLD) {
  const full = join(target, path);
  mkdirSync(join(full, ".."), { recursive: true });
  writeFileSync(full, "x".repeat(bytes));
  utimesSync(full, time, time);
}

function found() {
  return targetEntries(target)
    .map((e) => ({ ...e, path: relative(target, e.path).split("\\").join("/") }))
    .sort((a, b) => a.path.localeCompare(b.path));
}

beforeEach(() => {
  target = mkdtempSync(join(tmpdir(), "target-entries-"));
});

afterEach(() => {
  rmSync(target, { recursive: true, force: true });
});

describe("targetEntries", () => {
  it("lists the unit entries of every profile, a cross-compiled one too", () => {
    write(`debug/.fingerprint/x-${A}/lib-x`, 1);
    write(`debug/deps/libx-${A}.rlib`, 5);
    write(`wasm32-unknown-unknown/debug/.fingerprint/y-${A}/lib-y`, 1);
    const listed = found().map((e) => [e.path, e.key]);
    assert.deepEqual(listed, [
      [`debug/.fingerprint/x-${A}`, A],
      [`debug/deps/libx-${A}.rlib`, A],
      [`wasm32-unknown-unknown/debug/.fingerprint/y-${A}`, A],
    ]);
  });

  it("sums a folder's bytes and takes its newest use", () => {
    write(`debug/.fingerprint/x-${A}/lib-x`, 3);
    write(`debug/build/x-${A}/out/gen.rs`, 4, NEW);
    const build = found().find((e) => e.path.startsWith("debug/build/"));
    assert.equal(build?.bytes, 4);
    assert.equal(build?.lastUsed, NEW.getTime());
  });

  it("keys each incremental cache by its own path", () => {
    write(`debug/.fingerprint/x-${A}/lib-x`, 1);
    write("debug/incremental/bnto-010s5tz5p75xc/s-abc/dep-graph.bin", 2);
    const cache = found().find((e) => e.path.startsWith("debug/incremental/"));
    assert.equal(cache?.key, join(target, "debug/incremental/bnto-010s5tz5p75xc"));
  });

  it("leaves out what no unit owns: copied binaries, cargo's lock, other folders", () => {
    write(`debug/.fingerprint/x-${A}/lib-x`, 1);
    write("debug/bnto", 1);
    write("debug/.cargo-lock", 1);
    write("debug/deps/README", 1);
    write("package/bnto-0.17.1.crate", 1);
    assert.deepEqual(
      found().map((e) => e.path),
      [`debug/.fingerprint/x-${A}`],
    );
  });

  it("finds nothing in a missing folder", () => {
    assert.deepEqual(targetEntries(join(target, "missing")), []);
  });
});
