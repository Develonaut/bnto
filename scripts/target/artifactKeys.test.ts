/**
 * Tests for artifactKeys. Uses node:test.
 * Run: pnpm exec tsx --test scripts/target/artifactKeys.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { artifactKeys } from "./artifactKeys.ts";

function keys(output: string) {
  return [...artifactKeys(output)].sort();
}

const A = "aaaaaaaaaaaaaaaa";
const B = "bbbbbbbbbbbbbbbb";

function artifact(filenames: string[], executable: string | null = null) {
  return JSON.stringify({ reason: "compiler-artifact", filenames, executable, fresh: true });
}

describe("artifactKeys", () => {
  it("collects the unit of every artifact, fresh ones included", () => {
    const out = [
      artifact([`/t/debug/deps/libx-${A}.rlib`, `/t/debug/deps/libx-${A}.rmeta`]),
      artifact([`C:\\t\\debug\\deps\\cli-${B}.exe`], `C:\\t\\debug\\deps\\cli-${B}.exe`),
    ].join("\n");
    assert.deepEqual(keys(out), [A, B]);
  });

  it("takes a build script's unit from its folder", () => {
    const out = artifact([`/t/debug/build/x-${A}/build-script-build`]);
    assert.deepEqual(keys(out), [A]);
  });

  it("takes a build script run's unit from its output folder", () => {
    const out = JSON.stringify({
      reason: "build-script-executed",
      out_dir: `/t/debug/build/x-${B}/out`,
    });
    assert.deepEqual(keys(out), [B]);
  });

  it("skips other messages and lines that aren't JSON", () => {
    const out = [
      "   Compiling x v0.1.0",
      JSON.stringify({ reason: "build-finished", success: true }),
    ].join("\n");
    assert.deepEqual(keys(out), []);
  });

  it("skips a binary's copy and a cdylib crate's files, which carry no unit hash", () => {
    const out = [
      artifact(["/t/debug/bnto"], "/t/debug/bnto"),
      artifact(["/t/debug/deps/libbnto_wasm.dylib", "/t/debug/deps/libbnto_wasm.rlib"]),
    ].join("\n");
    assert.deepEqual(keys(out), []);
  });
});
