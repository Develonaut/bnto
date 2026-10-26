/**
 * Fail naming every Turbo lint, test or build task that doesn't hash a file
 * outside its package that its sources import (ISS-9, ISS-51). Lint starts
 * from the package's ESLint config, test from all its sources, build from its
 * non-test sources. Reads `turbo run … --dry=json`, so nothing runs. Runtime
 * reads (`readFileSync`) aren't seen: list those by hand. Run from the root:
 *   pnpm exec tsx scripts/turbo/checkTaskInputs.ts
 */

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync, statSync } from "node:fs";
import { posix } from "node:path";
import { missingTaskInputs, type Task } from "./missingTaskInputs";

interface DryRunTask extends Omit<Task, "inputs"> {
  taskId: string;
  task: string;
  inputs: Record<string, string>;
}

const CHECKED = ["lint", "test", "build"];
const SOURCE = /\.(?:[cm]?[jt]sx?)$/;
const TEST = /\.test\.[cm]?[jt]sx?$|\/__tests__\//;
// Read by lint only.
const LINT_CONFIG = /(?:^|\/)eslint\.config\.[cm]?js$/;

function dryRunTasks(): DryRunTask[] {
  // On Windows `pnpm` is a `pnpm.cmd` shim, which only a shell can start.
  const out = execFileSync("pnpm", ["-s", "turbo", "run", ...CHECKED, "--dry=json"], {
    encoding: "utf8",
    shell: process.platform === "win32",
  });
  const { tasks } = JSON.parse(out.slice(out.indexOf("{"))) as { tasks: DryRunTask[] };
  return tasks.filter((t) => CHECKED.includes(t.task) && t.directory);
}

const tracked = execFileSync("git", ["ls-files"], { encoding: "utf8" }).trim().split("\n");

// Package sources, relative to its folder.
function sources(directory: string): string[] {
  return tracked
    .filter(
      (file) => file.startsWith(`${directory}/`) && SOURCE.test(file) && !LINT_CONFIG.test(file),
    )
    .map((file) => posix.relative(directory, file));
}

function entries(task: DryRunTask): string[] {
  if (task.task === "lint") {
    return existsSync(posix.join(task.directory, "eslint.config.mjs")) ? ["eslint.config.mjs"] : [];
  }
  const all = sources(task.directory);
  return task.task === "build" ? all.filter((file) => !TEST.test(file)) : all;
}

// A folder isn't a file an import resolves to (`./lib` is `./lib/index.ts`).
const isFile = (path: string) => statSync(path, { throwIfNoEntry: false })?.isFile() ?? false;
const repo = { read: (file: string) => readFileSync(file, "utf8"), exists: isFile };

const failures = dryRunTasks().flatMap((t) =>
  missingTaskInputs(
    { directory: t.directory, inputs: Object.keys(t.inputs) },
    entries(t),
    repo,
  ).map((file) => `  ${t.taskId}: imports ${file}; add "$TURBO_ROOT$/${file}" to its inputs`),
);

if (failures.length > 0) {
  console.error(
    `Turbo task inputs miss files their sources import (turbo.json):\n${failures.join("\n")}`,
  );
  process.exit(1);
}
console.log("Every Turbo lint, test and build task hashes the files its sources import.");
