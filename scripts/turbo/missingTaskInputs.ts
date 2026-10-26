/**
 * The files outside a package that a Turbo task's sources import but the task
 * doesn't hash. Turbo hashes only a package's own files by default, so a
 * change to such a file would replay a stale cached result (ISS-9, ISS-51).
 */

import { posix } from "node:path";
import ts from "typescript";

/** A `<pkg>#<task>` entry from `turbo run … --dry=json`. */
export interface Task {
  /** Package folder, relative to the repo root (`packages/@bnto/auth`). */
  directory: string;
  /** Hashed files, relative to the package folder (`../../../eslint.config.base.mjs`). */
  inputs: readonly string[];
}

/** Repo-relative file access. */
export interface Repo {
  read: (file: string) => string;
  /** Whether `file` is a file (not a folder). */
  exists: (file: string) => boolean;
}

// What an extensionless or folder import can name, in the order TypeScript tries.
const SUFFIXES = [
  "",
  ".ts",
  ".tsx",
  ".mts",
  ".js",
  ".mjs",
  ".json",
  "/index.ts",
  "/index.tsx",
  "/index.js",
];

// Sources whose own imports are followed; JSON and assets are only collected.
const SCRIPT = /\.(?:[cm]?[jt]sx?)$/;

const isOutside = (dir: string, file: string) => posix.relative(dir, file).startsWith("../");

const isRelative = (specifier: string) => specifier.startsWith("./") || specifier.startsWith("../");

function resolve(path: string, repo: Repo): string | undefined {
  return SUFFIXES.map((suffix) => path + suffix).find(repo.exists);
}

/** Files `file` imports through relative specifiers (static, re-exported or dynamic) that exist. */
function relativeImports(file: string, repo: Repo): string[] {
  if (!SCRIPT.test(file)) return [];
  const { importedFiles } = ts.preProcessFile(repo.read(file), true, true);
  return importedFiles
    .map((imported) => imported.fileName)
    .filter(isRelative)
    .map((specifier) => resolve(posix.join(posix.dirname(file), specifier), repo))
    .filter((resolved): resolved is string => resolved !== undefined);
}

/**
 * Repo-relative paths of files outside `task.directory` that `entries`
 * (relative to the package folder) import, directly or through other relative
 * imports, and that `task.inputs` lacks.
 */
export function missingTaskInputs(task: Task, entries: readonly string[], repo: Repo): string[] {
  const hashed = new Set(task.inputs.map((input) => posix.join(task.directory, input)));
  const queue = entries.map((entry) => posix.join(task.directory, entry));
  const seen = new Set(queue);
  for (const file of queue) {
    const unseen = relativeImports(file, repo).filter((next) => !seen.has(next));
    for (const next of unseen) seen.add(next);
    queue.push(...unseen);
  }
  return queue.filter((file) => isOutside(task.directory, file) && !hashed.has(file));
}
