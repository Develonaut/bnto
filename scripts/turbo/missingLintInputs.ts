/**
 * The files outside a package that its ESLint config reads but its Turbo
 * lint task doesn't hash. Turbo hashes only a package's own files by default,
 * so a change to such a file would replay a stale cached lint result (ISS-9).
 */

import { posix } from "node:path";

/** A `<pkg>#lint` task from `turbo run lint --dry=json`. */
export interface LintTask {
  /** Package folder, relative to the repo root (`packages/@bnto/auth`). */
  directory: string;
  /** Hashed files, relative to the package folder (`../../../eslint.config.base.mjs`). */
  inputs: readonly string[];
}

/** `import … from "./x"`, `import "../x"` and `export … from "./x"`. */
const RELATIVE_IMPORT = /\b(?:from|import)\s*["'](\.{1,2}\/[^"']+)["']/g;

/** Block comments and whole-line `//` comments, whose usage examples aren't imports. */
const COMMENT = /\/\*[\s\S]*?\*\/|^\s*\/\/.*$/gm;

const isOutside = (dir: string, file: string) => posix.relative(dir, file).startsWith("../");

function relativeImports(file: string, source: string): string[] {
  const dir = posix.dirname(file);
  const code = source.replace(COMMENT, "");
  return [...code.matchAll(RELATIVE_IMPORT)].map((m) => posix.join(dir, m[1]));
}

/**
 * Repo-relative paths of files outside `task.directory` that `entry` imports,
 * directly or through other relative imports, and that `task.inputs` lacks.
 * `read` returns a repo-relative file's source.
 */
export function missingLintInputs(
  task: LintTask,
  read: (file: string) => string,
  entry = "eslint.config.mjs",
): string[] {
  const hashed = new Set(task.inputs.map((input) => posix.join(task.directory, input)));
  const queue = [posix.join(task.directory, entry)];
  const seen = new Set(queue);
  const missing: string[] = [];
  for (const file of queue) {
    if (isOutside(task.directory, file) && !hashed.has(file)) missing.push(file);
    for (const next of relativeImports(file, read(file))) {
      if (seen.has(next)) continue;
      seen.add(next);
      queue.push(next);
    }
  }
  return missing;
}
