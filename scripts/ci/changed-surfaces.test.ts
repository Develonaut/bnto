/**
 * Tests for changed-surfaces.sh, the one list of paths that decides which CI
 * and release jobs run (ISS-38). Checks that every root file a job reads is
 * on it. Run: pnpm exec tsx --test scripts/ci/changed-surfaces.test.ts
 */

import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, statSync } from "node:fs";
import { describe, it } from "node:test";
import { engineRootReads } from "./engineRootReads";

type Surfaces = Record<string, string>;

function surfaces(paths: string[], ...args: string[]): Surfaces {
  const out = execFileSync("sh", ["scripts/ci/changed-surfaces.sh", ...args], {
    input: paths.join("\n"),
    encoding: "utf-8",
  });
  return Object.fromEntries(
    out
      .trim()
      .split("\n")
      .map((line) => line.split("=")),
  );
}

const gitFiles = (pattern: string) =>
  execFileSync("git", ["ls-files", pattern], { encoding: "utf-8" }).trim().split("\n");

// `$TURBO_ROOT$/…` inputs: root files a TypeScript task reads.
function turboRootInputs(): string[] {
  const inputs = gitFiles("*turbo.json").flatMap((file) =>
    [...readFileSync(file, "utf-8").matchAll(/"\$TURBO_ROOT\$\/([^"]+)"/g)].map((m) => m[1]),
  );
  return [...new Set(inputs)];
}

interface GatedJob {
  name: string;
  flags: string[];
  paths: string[];
}

// ci.yml's jobs that run only on some surfaces, with the repo scripts and
// recipes their steps name.
function gatedJobs(): GatedJob[] {
  const jobs = readFileSync(".github/workflows/ci.yml", "utf-8").split(/^ {2}(?=[a-z][\w-]*:$)/m);
  return jobs.slice(1).flatMap((body) => {
    const flags = [...body.matchAll(/detect-changes\.outputs\.(\w+)/g)].map((m) => m[1]);
    const code = body
      .split("\n")
      .filter((line) => !line.trim().startsWith("#"))
      .join("\n");
    const paths = [...code.matchAll(/(?:scripts|recipes)\/[\w./-]+/g)].map((m) => m[0]);
    return flags.length > 0 ? [{ name: body.slice(0, body.indexOf(":")), flags, paths }] : [];
  });
}

describe("changed-surfaces.sh", () => {
  it("sorts paths into engine, web and convex", () => {
    assert.deepEqual(surfaces(["engine/crates/bnto/src/main.rs"]), {
      engine: "true",
      web: "false",
      convex: "false",
    });
    assert.deepEqual(surfaces(["pnpm-lock.yaml", "packages/@bnto/backend/convex/users.ts"]), {
      engine: "false",
      web: "true",
      convex: "true",
    });
    assert.deepEqual(surfaces(["test-fixtures/images/medium.png"]), {
      engine: "true",
      web: "true",
      convex: "false",
    });
    assert.deepEqual(surfaces([".gitattributes"]), {
      engine: "true",
      web: "true",
      convex: "false",
    });
    assert.deepEqual(surfaces(["docs/notes.txt"]), {
      engine: "false",
      web: "false",
      convex: "false",
    });
  });

  it("leaves CI's own files out for a release", () => {
    const ciOnly = [".github/workflows/ci.yml", "scripts/ci/changed-surfaces.sh"];
    assert.deepEqual(surfaces(ciOnly, "--release"), {
      engine: "false",
      web: "false",
      convex: "false",
    });
    assert.deepEqual(surfaces(ciOnly), { engine: "true", web: "true", convex: "false" });
    assert.deepEqual(surfaces([], "--release"), { engine: "false", web: "false", convex: "false" });
  });

  it("runs each gated CI job for the scripts and recipes it runs", () => {
    const jobs = gatedJobs();
    assert.ok(jobs.some((job) => job.paths.includes("scripts/turbo/checkLintInputs.ts")));
    for (const { name, flags, paths } of jobs) {
      for (const path of paths) {
        const result = surfaces([path]);
        assert.ok(
          flags.some((flag) => result[flag] === "true"),
          `${name} runs ${path}`,
        );
      }
    }
  });

  it("runs the TypeScript job for every root file a Turbo task reads", () => {
    const inputs = turboRootInputs();
    assert.ok(inputs.length > 0);
    for (const file of inputs) assert.equal(surfaces([file]).web, "true", file);
  });

  it("runs the Rust jobs for every root file the engine reads", () => {
    const read = (file: string) => readFileSync(file, "utf-8");
    const roots = engineRootReads(gitFiles("engine/**/*.rs"), read, existsSync);
    assert.ok(roots.some((path) => path.startsWith("test-fixtures/")));
    for (const path of roots) {
      // A folder read stands for any file in it.
      const changed = statSync(path).isDirectory() ? `${path}/any` : path;
      assert.equal(surfaces([changed]).engine, "true", path);
    }
  });

  it("is the only change filter in CI and release", () => {
    for (const workflow of [".github/workflows/ci.yml", ".github/workflows/release.yml"]) {
      const source = readFileSync(workflow, "utf-8");
      assert.match(source, /scripts\/ci\/changed-surfaces\.sh/, workflow);
      // Without it a moved file shows only its new path, and moving one out
      // of a surface would skip that surface's jobs.
      assert.match(source, /git diff --name-only --no-renames /, workflow);
      assert.doesNotMatch(source, /paths-filter|grep -qE? '\^/, workflow);
    }
  });
});
