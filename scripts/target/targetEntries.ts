/**
 * Every file or folder a cargo unit owns in a target folder (ISS-120): the
 * top-level entries of each profile's `deps`, `.fingerprint`, `build` and
 * `examples` folders, keyed by unit hash, and each incremental cache, keyed
 * by its own path. Copied binaries and anything without a unit hash are left
 * out, so nothing prunes them.
 */

import { existsSync, lstatSync, readdirSync, type Stats } from "node:fs";
import { basename, join } from "node:path";
import { unitKey } from "./unitKey.ts";

export interface TargetEntry {
  path: string;
  key: string;
  /** Newest access or modification time (ms) of the entry or anything in it. */
  lastUsed: number;
  bytes: number;
}

const UNIT_FOLDERS = ["deps", ".fingerprint", "build", "examples"];

function children(dir: string): string[] {
  return existsSync(dir) ? readdirSync(dir).map((name) => join(dir, name)) : [];
}

// `debug`, `release`, … and `<triple>/<profile>` for a cross-compile: any
// folder up to two deep that holds a `.fingerprint` folder.
function profileFolders(target: string): string[] {
  const level1 = children(target).filter((dir) => lstatSync(dir).isDirectory());
  const level2 = level1.flatMap((dir) => children(dir).filter((d) => lstatSync(d).isDirectory()));
  return [...level1, ...level2].filter((dir) => existsSync(join(dir, ".fingerprint")));
}

// Atime counts where the filesystem moves it on a read (Linux's relatime);
// macOS and Windows mostly don't, so markUsed.ts marks a build's units.
// Only files count: listing a folder, as this walk does, updates its atime.
// A file with several names (rustc hard-links incremental work products into
// deps/) counts its bytes once, for the first entry walked: deps/ comes first.
function usage(path: string, seen: Set<string>): Pick<TargetEntry, "lastUsed" | "bytes"> {
  const stat = lstatSync(path);
  if (stat.isDirectory()) {
    return children(path)
      .map((child) => usage(child, seen))
      .reduce(
        (sum, c) => ({ lastUsed: Math.max(sum.lastUsed, c.lastUsed), bytes: sum.bytes + c.bytes }),
        {
          lastUsed: 0,
          bytes: 0,
        },
      );
  }
  return {
    lastUsed: Math.max(stat.atimeMs, stat.mtimeMs),
    bytes: firstName(path, stat, seen) ? stat.size : 0,
  };
}

function firstName(path: string, stat: Stats, seen: Set<string>): boolean {
  if (stat.nlink < 2) return true;
  const { dev, ino } = lstatSync(path, { bigint: true });
  const id = `${dev}:${ino}`;
  if (seen.has(id)) return false;
  seen.add(id);
  return true;
}

function profileEntries(profile: string, seen: Set<string>): TargetEntry[] {
  const owned = UNIT_FOLDERS.flatMap((folder) => children(join(profile, folder)))
    .map((path) => ({ path, key: unitKey(basename(path)) }))
    .filter((e): e is { path: string; key: string } => e.key !== null);
  const caches = children(join(profile, "incremental")).map((path) => ({ path, key: path }));
  return [...owned, ...caches].map((e) => ({ ...e, ...usage(e.path, seen) }));
}

/** The unit entries of every profile in `target`; none when it doesn't exist. */
export function targetEntries(target: string): TargetEntry[] {
  if (!existsSync(target)) return [];
  const seen = new Set<string>();
  return profileFolders(target).flatMap((profile) => profileEntries(profile, seen));
}
