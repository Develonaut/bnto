/**
 * Tests for unitKey. Uses node:test.
 * Run: pnpm exec tsx --test scripts/target/unitKey.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { unitKey } from "./unitKey.ts";

const HASH = "c7e8d4d29fedd94f";

describe("unitKey", () => {
  it("reads the hash from every file a unit writes into deps/", () => {
    for (const name of [
      `libadler2-${HASH}.rlib`,
      `libadler2-${HASH}.rmeta`,
      `adler2-${HASH}.d`,
      `adler2-${HASH}.adler2.8297be63c448ff7f-cgu.0.rcgu.o`,
      `cli-${HASH}`,
      `cli-${HASH}.exe`,
      `cli-${HASH}.pdb`,
    ]) {
      assert.equal(unitKey(name), HASH, name);
    }
  });

  it("reads it from a fingerprint or build folder named after a hyphenated package", () => {
    assert.equal(unitKey(`bnto-core-${HASH}`), HASH);
  });

  it("returns null for names cargo gives no unit hash", () => {
    for (const name of ["bnto", "bnto.d", ".cargo-lock", "bnto-010s5tz5p75xc", "libfoo-abc.rlib"]) {
      assert.equal(unitKey(name), null, name);
    }
  });
});
