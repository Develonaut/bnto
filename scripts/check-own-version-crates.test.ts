/**
 * Tests for check-own-version-crates.sh when crates.io can't be reached.
 * Run: pnpm exec tsx --test scripts/check-own-version-crates.test.ts
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { describe, it } from "node:test";

// Nothing listens on port 9, so every request through this proxy fails.
function offlineRun(ci: boolean) {
  const env = { ...process.env, HTTPS_PROXY: "http://127.0.0.1:9", https_proxy: "" };
  delete env.NO_PROXY;
  delete env.no_proxy;
  delete env.CI;
  if (ci) env.CI = "true";
  return spawnSync("bash", ["scripts/check-own-version-crates.sh"], { encoding: "utf-8", env });
}

describe("check-own-version-crates.sh offline", () => {
  it("warns that it skipped, and doesn't claim the crate will publish", () => {
    const result = offlineRun(false);
    assert.equal(result.status, 0);
    assert.match(result.stderr, /couldn't check tonkotsu on crates\.io \(HTTP 000\): skipped/);
    assert.doesNotMatch(result.stdout, /will publish/);
  });

  it("fails in CI, where the check must run", () => {
    const result = offlineRun(true);
    assert.equal(result.status, 1);
    assert.match(result.stderr, /couldn't check tonkotsu on crates\.io/);
  });
});
