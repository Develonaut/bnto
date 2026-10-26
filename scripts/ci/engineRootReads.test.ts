/**
 * Tests for engineRootReads. Uses node:test.
 * Run: pnpm exec tsx --test scripts/ci/engineRootReads.test.ts
 */

import assert from "node:assert/strict";
import { describe, it } from "node:test";
import { engineRootReads } from "./engineRootReads";

const TESTS = "engine/crates/bnto-csv/tests/common/mod.rs";
const SRC = "engine/crates/bnto-csv/src/io.rs";

function reads(files: Record<string, string>, existing: string[]) {
  const exists = (path: string) => existing.some((e) => e === path || e.startsWith(`${path}/`));
  return engineRootReads(Object.keys(files), (file) => files[file], exists);
}

describe("engineRootReads", () => {
  it("resolves an include_bytes! path from the file's folder", () => {
    const files = { [TESTS]: 'include_bytes!("../../../../../test-fixtures/csv/a.csv")' };
    assert.deepEqual(reads(files, ["test-fixtures/csv/a.csv"]), ["test-fixtures/csv/a.csv"]);
  });

  it("resolves a CARGO_MANIFEST_DIR join from the crate's folder", () => {
    const files = {
      [SRC]: 'Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../test-fixtures/x.jpg")',
    };
    assert.deepEqual(reads(files, ["test-fixtures/x.jpg"]), ["test-fixtures/x.jpg"]);
  });

  it("takes the folder of a templated path", () => {
    const files = { [SRC]: 'dir.join(format!("../../../test-fixtures/images/{name}"))' };
    assert.deepEqual(reads(files, ["test-fixtures/images/a.png"]), ["test-fixtures/images"]);
  });

  it("ignores engine/ paths, the repo root, ts-rs export targets and missing paths", () => {
    const files = {
      [TESTS]: 'read("../fixtures/a.json"); read("../../../../../nope/a.txt"); root("../../..")',
      [SRC]: '#[ts(export_to = "../../../../packages/@bnto/nodes/src/generated/")]',
    };
    assert.deepEqual(reads(files, ["packages/@bnto/nodes/src/generated"]), []);
  });
});
