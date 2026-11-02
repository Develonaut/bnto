/**
 * Delete the build units and incremental caches in a cargo target folder that
 * no build used for a week (ISS-120). Cargo never deletes what it built
 * before, so without this `engine/target` grows without bound (57 GB on one
 * machine). Something still in use is rebuilt by the next build that needs it.
 * `task check` runs it; `task clean:stale` runs it alone.
 *   pnpm exec tsx scripts/target/pruneTarget.ts [--days N] [--dry-run] [target]
 * The target defaults to `engine/target`, from the repo root.
 */

import { rmSync } from "node:fs";
import { parseArgs } from "node:util";
import { staleEntries } from "./staleEntries.ts";
import { type TargetEntry, targetEntries } from "./targetEntries.ts";

function options() {
  const { values, positionals } = parseArgs({
    allowPositionals: true,
    options: {
      days: { type: "string", default: "7" },
      "dry-run": { type: "boolean", default: false },
    },
  });
  const days = Number(values.days);
  if (!(days >= 0)) throw new Error(`--days takes a number of days, not "${values.days}"`);
  return { days, dryRun: values["dry-run"], target: positionals[0] ?? "engine/target" };
}

// A file a running program holds (Windows) can't go: say so, and go on.
function remove(entries: TargetEntry[]): TargetEntry[] {
  return entries.filter((entry) => {
    try {
      rmSync(entry.path, { recursive: true, force: true });
      return true;
    } catch (error) {
      console.warn(`warning: couldn't remove ${entry.path}: ${(error as Error).message}`);
      return false;
    }
  });
}

function main() {
  const { days, dryRun, target } = options();
  const stale = staleEntries(targetEntries(target), Date.now(), days);
  const gone = dryRun ? stale : remove(stale);
  const units = new Set(gone.map((e) => e.key)).size;
  const gb = (gone.reduce((sum, e) => sum + e.bytes, 0) / 1e9).toFixed(1);
  const verb = dryRun ? "Would prune" : "Pruned";
  console.log(
    `${verb} ${units} build units and caches (${gb} GB) unused for ${days} days from ${target}`,
  );
}

main();
