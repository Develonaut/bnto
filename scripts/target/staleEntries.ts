/**
 * The target entries to prune: every entry of a unit none of whose entries
 * was used in the last `days` days (ISS-120). A unit goes whole, so cargo
 * never finds a fingerprint whose outputs are gone, or outputs it can't trust.
 */

import type { TargetEntry } from "./targetEntries.ts";

const DAY_MS = 24 * 60 * 60 * 1000;

function lastUseByUnit(entries: TargetEntry[]): Map<string, number> {
  const lastUse = new Map<string, number>();
  for (const { key, lastUsed } of entries) {
    lastUse.set(key, Math.max(lastUse.get(key) ?? 0, lastUsed));
  }
  return lastUse;
}

/** Entries of units last used more than `days` days before `now` (ms). */
export function staleEntries(entries: TargetEntry[], now: number, days: number): TargetEntry[] {
  const lastUse = lastUseByUnit(entries);
  const cutoff = now - days * DAY_MS;
  return entries.filter((entry) => (lastUse.get(entry.key) ?? 0) < cutoff);
}
