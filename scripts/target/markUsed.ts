/**
 * Mark build units as used now, so the prune keeps them (ISS-120). A read
 * doesn't move a file's atime on macOS when atime equals mtime (APFS updates
 * it only when it's older than mtime), and Windows mostly keeps no atime, so
 * there only a rebuild would count as use. It touches one file per unit, the
 * fingerprint's `.json` (the prune takes a unit's newest file): cargo reads
 * that file only to explain a rebuild and never compares its times. Setting
 * a time from a JS Date rounds the mtime to the millisecond, so files whose
 * times cargo does compare (outputs, `invoked.timestamp`) are never touched.
 */

import { readdirSync, statSync, utimesSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import type { TargetEntry } from "./targetEntries.ts";

function fingerprintJson(entry: TargetEntry): string[] {
  if (basename(dirname(entry.path)) !== ".fingerprint") return [];
  return readdirSync(entry.path)
    .filter((name) => name.endsWith(".json"))
    .map((name) => join(entry.path, name));
}

/** Sets the access time of each entry's fingerprint `.json` to `now`. */
export function markUsed(entries: TargetEntry[], now: Date): void {
  for (const file of entries.flatMap(fingerprintJson)) {
    utimesSync(file, now, statSync(file).mtime);
  }
}
