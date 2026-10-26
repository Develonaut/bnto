import { existsSync, readdirSync, rmSync } from "node:fs";
import { resolve } from "node:path";

/** Remove files in a directory that aren't in the expected set. */
export function cleanStaleFiles(dir: string, expectedFiles: Set<string>): void {
  if (!existsSync(dir)) return;
  for (const file of readdirSync(dir)) {
    if (!expectedFiles.has(file)) {
      rmSync(resolve(dir, file));
    }
  }
}
