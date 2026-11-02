/**
 * Mark build units as used now, so the prune keeps them (ISS-120). A read
 * doesn't move a file's atime on macOS when atime equals mtime (APFS updates
 * it only when it's older than mtime), and Windows mostly keeps no atime, so
 * there only a rebuild would count as use. Setting the atime of each entry's
 * own files (a folder's direct files) is enough: the prune takes a unit's
 * newest file. Modification times stay, since cargo's freshness checks read them.
 */

import { lstatSync, readdirSync, utimesSync } from "node:fs";
import { join } from "node:path";
import type { TargetEntry } from "./targetEntries.ts";

function ownFiles(path: string): string[] {
  if (!lstatSync(path).isDirectory()) return [path];
  return readdirSync(path)
    .map((name) => join(path, name))
    .filter((child) => lstatSync(child).isFile());
}

/** Sets the access time of every entry's own files to `now`. */
export function markUsed(entries: TargetEntry[], now: Date): void {
  for (const file of entries.flatMap((entry) => ownFiles(entry.path))) {
    utimesSync(file, now, lstatSync(file).mtime);
  }
}
