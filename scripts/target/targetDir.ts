/**
 * The target folder cargo builds into for a workspace (ISS-120), as cargo
 * itself resolves it: `$CARGO_TARGET_DIR` (relative to the workspace),
 * `build.target-dir` in a cargo config, else `<workspace>/target`.
 */

import { execFileSync } from "node:child_process";

/** Absolute target folder for the cargo workspace at `workspace`. */
export function targetDir(workspace: string, env: NodeJS.ProcessEnv = process.env): string {
  const metadata = execFileSync("cargo", ["metadata", "--no-deps", "--format-version", "1"], {
    cwd: workspace,
    encoding: "utf8",
    env,
    maxBuffer: 64 * 1024 * 1024,
  });
  return (JSON.parse(metadata) as { target_directory: string }).target_directory;
}
