/**
 * The files outside a package that its ESLint config reads but its Turbo
 * lint task doesn't hash. Turbo hashes only a package's own files by default,
 * so a change to such a file would replay a stale cached lint result (ISS-9).
 */

import { posix } from "node:path";
import ts from "typescript";

/** A `<pkg>#lint` task from `turbo run lint --dry=json`. */
export interface LintTask {
  /** Package folder, relative to the repo root (`packages/@bnto/auth`). */
  directory: string;
  /** Hashed files, relative to the package folder (`../../../eslint.config.base.mjs`). */
  inputs: readonly string[];
}

const isOutside = (dir: string, file: string) => posix.relative(dir, file).startsWith("../");

const isRelative = (specifier: string) => specifier.startsWith("./") || specifier.startsWith("../");

/** Repo-relative files `file` imports: static, re-exported or dynamic. TypeScript's scanner skips comments and strings. */
function relativeImports(file: string, source: string): string[] {
  const { importedFiles } = ts.preProcessFile(source, true, true);
  return importedFiles
    .map((imported) => imported.fileName)
    .filter(isRelative)
    .map((specifier) => posix.join(posix.dirname(file), specifier));
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
  for (const file of queue) {
    const unseen = relativeImports(file, read(file)).filter((next) => !queue.includes(next));
    queue.push(...new Set(unseen));
  }
  return queue.filter((file) => isOutside(task.directory, file) && !hashed.has(file));
}
