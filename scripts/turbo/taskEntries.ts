/**
 * The files a Turbo task starts reading from, for the inputs check (ISS-51):
 * lint from the package's ESLint config, test from all its sources, build
 * from its non-test sources.
 */

const SOURCE = /\.[cm]?[jt]sx?$/;
const TEST = /\.test\.[cm]?[jt]sx?$|(?:^|\/)__tests__\//;
// Read by lint only.
const LINT_CONFIG = /(?:^|\/)eslint\.config\.[cm]?js$/;

/**
 * Entry files, relative to the package folder, for `task` given the package's
 * tracked files (also relative to it).
 */
export function taskEntries(task: string, files: readonly string[]): string[] {
  if (task === "lint") return files.filter((file) => file === "eslint.config.mjs");
  const sources = files.filter((file) => SOURCE.test(file) && !LINT_CONFIG.test(file));
  return task === "build" ? sources.filter((file) => !TEST.test(file)) : sources;
}
