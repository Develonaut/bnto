/**
 * Repo paths outside `engine/` that the engine's Rust code reads through a
 * `../` string literal (test fixtures, mostly). CI must run the Rust jobs when
 * one changes (ISS-38).
 */

import { posix } from "node:path";

// A `../` path in a string literal, optionally after a leading `/` (`concat!`).
const RELATIVE_LITERAL = /"\/?((?:\.\.\/)+[^"]*)"/g;

// include_bytes! resolves from the file's folder, CARGO_MANIFEST_DIR joins
// from the crate's: try both.
function bases(file: string): string[] {
  return [posix.dirname(file), file.split("/").slice(0, 3).join("/")];
}

// The fixed part of a templated path (`…/images/{name}` → `…/images`).
function fixedPart(literal: string): string {
  const brace = literal.indexOf("{");
  return brace === -1 ? literal : posix.dirname(literal.slice(0, brace) + "x");
}

// `engine` itself and the repo root (`.`, which release-verify's tests run
// against) name no one path a filter could list.
function isOutsideEngine(path: string): boolean {
  return path !== "engine" && path !== "." && !path.startsWith("engine/") && !path.startsWith("..");
}

function readsIn(file: string, source: string, exists: (path: string) => boolean): string[] {
  // ts-rs `export_to` paths are where generated types are written, not reads.
  const lines = source.split("\n").filter((line) => !line.includes("export_to"));
  const literals = lines.flatMap((line) => [...line.matchAll(RELATIVE_LITERAL)].map((m) => m[1]));
  return literals.flatMap((literal) =>
    bases(file)
      .map((base) => posix.normalize(posix.join(base, fixedPart(literal))).replace(/\/$/, ""))
      .filter((path) => isOutsideEngine(path) && exists(path)),
  );
}

/** Root paths the given engine `.rs` files (repo-relative) read, deduplicated and sorted. */
export function engineRootReads(
  files: string[],
  read: (file: string) => string,
  exists: (path: string) => boolean,
): string[] {
  const paths = files.flatMap((file) => readsIn(file, read(file), exists));
  return [...new Set(paths)].sort();
}
