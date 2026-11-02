/**
 * Tests for targetDir, on a fixture cargo package. Uses node:test.
 * Run: pnpm exec tsx --test scripts/target/targetDir.test.ts
 */

import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterEach, beforeEach, describe, it } from "node:test";
import { targetDir } from "./targetDir.ts";

// The caller's environment minus any target folder of its own.
const { CARGO_TARGET_DIR: _, CARGO_BUILD_TARGET_DIR: __, ...ENV } = process.env;

let root = "";

beforeEach(() => {
  root = realpathSync(mkdtempSync(join(tmpdir(), "target-dir-")));
  writeFileSync(
    join(root, "Cargo.toml"),
    '[package]\nname = "fx"\nversion = "0.1.0"\nedition = "2021"\n',
  );
  mkdirSync(join(root, "src"));
  writeFileSync(join(root, "src/lib.rs"), "");
});

afterEach(() => {
  rmSync(root, { recursive: true, force: true });
});

describe("targetDir", () => {
  it("is the package's own target folder by default", () => {
    assert.equal(targetDir(root, ENV), join(root, "target"));
  });

  it("resolves a relative CARGO_TARGET_DIR from the package, as cargo does", () => {
    assert.equal(
      targetDir(root, { ...ENV, CARGO_TARGET_DIR: "elsewhere" }),
      join(root, "elsewhere"),
    );
  });

  it("follows build.target-dir in the package's cargo config", () => {
    mkdirSync(join(root, ".cargo"));
    writeFileSync(join(root, ".cargo/config.toml"), '[build]\ntarget-dir = "from-config"\n');
    assert.equal(targetDir(root, ENV), join(root, "from-config"));
  });
});
