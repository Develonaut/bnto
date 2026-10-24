/**
 * Fail naming every package whose ESLint config reads a file outside the
 * package that its Turbo lint task doesn't hash (ISS-9). Reads
 * `turbo run lint --dry=json`, so nothing runs. Run from the repo root:
 *   pnpm exec tsx scripts/turbo/checkLintInputs.ts
 */

import { execFileSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { posix } from "node:path";
import { missingLintInputs, type LintTask } from "./missingLintInputs";

interface DryRunTask extends Omit<LintTask, "inputs"> {
  taskId: string;
  inputs: Record<string, string>;
}

function lintTasks(): DryRunTask[] {
  // On Windows `pnpm` is a `pnpm.cmd` shim, which only a shell can start.
  const out = execFileSync("pnpm", ["-s", "turbo", "run", "lint", "--dry=json"], {
    encoding: "utf8",
    shell: process.platform === "win32",
  });
  const { tasks } = JSON.parse(out.slice(out.indexOf("{"))) as { tasks: DryRunTask[] };
  return tasks.filter((t) => t.taskId.endsWith("#lint"));
}

const read = (file: string) => readFileSync(file, "utf8");

const failures = lintTasks()
  .filter((t) => existsSync(posix.join(t.directory, "eslint.config.mjs")))
  .flatMap((t) =>
    missingLintInputs({ directory: t.directory, inputs: Object.keys(t.inputs) }, read).map(
      (file) => `  ${t.taskId}: reads ${file}; add "$TURBO_ROOT$/${file}" to its inputs`,
    ),
  );

if (failures.length > 0) {
  console.error(
    `Turbo lint inputs miss files the ESLint configs read (turbo.json):\n${failures.join("\n")}`,
  );
  process.exit(1);
}
console.log("Every Turbo lint task hashes the files its ESLint config reads.");
