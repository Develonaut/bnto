/**
 * Tests for the pre-commit hook's Turbo wrapper (ISS-39). Uses node:test.
 * Run: pnpm exec tsx --test scripts/hooks/turbo.test.ts
 */

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { describe, it } from "node:test";

const BASE = "eslint.config.base.mjs";
const DRY_RUN = ["run", "lint", "--filter=@bnto/auth", "--dry=json"];

interface DryTask {
  hash: string;
  inputs: Record<string, string>;
}

function lintTask(command: string, args: string[], env: NodeJS.ProcessEnv): DryTask {
  const json = execFileSync(command, args, { encoding: "utf-8", env });
  return (JSON.parse(json) as { tasks: DryTask[] }).tasks[0];
}

// Git sets GIT_DIR for hooks run in a worktree.
function hookEnv(): NodeJS.ProcessEnv {
  const gitDir = execFileSync("git", ["rev-parse", "--absolute-git-dir"], { encoding: "utf-8" });
  return { ...process.env, GIT_DIR: gitDir.trim() };
}

describe("scripts/hooks/turbo.sh", () => {
  it("hashes a task as a plain run does when GIT_DIR is set", () => {
    const { GIT_DIR: _unset, ...plainEnv } = process.env;
    const plain = lintTask("pnpm", ["-s", "turbo", ...DRY_RUN], plainEnv);
    const hook = lintTask("sh", ["scripts/hooks/turbo.sh", ...DRY_RUN], hookEnv());
    assert.ok(Object.keys(hook.inputs).some((file) => file.endsWith(BASE)));
    assert.equal(hook.hash, plain.hash);
  });

  it("runs every Turbo command in lefthook.yml", () => {
    const turboRuns = readFileSync("lefthook.yml", "utf-8")
      .split("\n")
      .filter((line) => /^\s*run:.*\bturbo\b/.test(line));
    assert.ok(turboRuns.length > 0);
    for (const line of turboRuns) assert.match(line, /run: sh scripts\/hooks\/turbo\.sh /);
  });
});
