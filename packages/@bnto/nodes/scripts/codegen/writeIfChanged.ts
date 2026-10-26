import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";

/** Write file only if content changed (avoids unnecessary git diffs). */
export function writeIfChanged(path: string, content: string): boolean {
  if (existsSync(path)) {
    const existing = readFileSync(path, "utf-8");
    if (existing === content) return false;
  }
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, content, "utf-8");
  return true;
}
