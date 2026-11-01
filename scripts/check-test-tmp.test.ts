/**
 * Tests for check-test-tmp.sh, run on a copy of it in a fixture repo.
 * Run: pnpm exec tsx --test scripts/check-test-tmp.test.ts
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { afterEach, beforeEach, describe, it } from "node:test";

const SCRIPT = resolve("scripts/check-test-tmp.sh");

let root = "";
let testTmp = "";
let systemTmp = "";

/** Runs the fixture's copy of the script, its system temp folder the fixture's own. */
function run(mode: string) {
  const env = { ...process.env, TMPDIR: systemTmp, TMP: systemTmp, TEMP: systemTmp };
  return spawnSync("sh", [join(root, "scripts/check-test-tmp.sh"), mode], {
    encoding: "utf-8",
    env,
  });
}

beforeEach(() => {
  root = mkdtempSync(join(tmpdir(), "check-test-tmp-"));
  mkdirSync(join(root, "scripts"));
  copyFileSync(SCRIPT, join(root, "scripts/check-test-tmp.sh"));
  testTmp = join(root, "engine/target/tmp");
  systemTmp = join(root, "system-tmp");
  mkdirSync(systemTmp);
});

afterEach(() => {
  rmSync(root, { recursive: true, force: true });
});

describe("check-test-tmp.sh", () => {
  it("reset empties the tests' temp folder", () => {
    mkdirSync(join(testTmp, "bnto-mod-test-abc"), { recursive: true });
    assert.equal(run("reset").status, 0);
    assert.deepEqual(readdirSync(testTmp), []);
  });

  it("check passes when a run left nothing", () => {
    run("reset");
    const result = run("check");
    assert.equal(result.status, 0, result.stderr);
    assert.equal(result.stderr, "");
  });

  it("check fails naming what a run left", () => {
    run("reset");
    mkdirSync(join(testTmp, "bnto-test-xyz"));
    writeFileSync(join(testTmp, "bnto-abc-output"), "");
    const result = run("check");
    assert.equal(result.status, 1);
    assert.match(result.stderr, /left 2 entries/);
    assert.match(result.stderr, /bnto-test-xyz/);
    assert.match(result.stderr, /bnto-abc-output/);
  });

  it("check warns when the system temp folder gained bnto entries", () => {
    mkdirSync(join(systemTmp, "bnto-old"));
    run("reset");
    mkdirSync(join(systemTmp, "bnto-new"));
    mkdirSync(join(systemTmp, "unrelated"));
    const result = run("check");
    assert.equal(result.status, 0);
    assert.match(result.stderr, /gained 1 bnto\* entries/);
  });

  it("check skips the warning without a reset first", () => {
    mkdirSync(testTmp, { recursive: true });
    mkdirSync(join(systemTmp, "bnto-new"));
    const result = run("check");
    assert.equal(result.status, 0);
    assert.equal(result.stderr, "");
    assert.equal(existsSync(join(root, "engine/target/test-tmp-baseline")), false);
  });

  it("refuses an unknown mode", () => {
    assert.equal(run("wipe").status, 2);
  });
});
