import appStrings from "./en.json";
import nodeStrings from "./generated/nodes.json";
import type { StringKey } from "./types";

const merged: Record<string, unknown> = { ...appStrings, ...nodeStrings };

/** Resolve a dot-path key to its string value. Returns the key itself on miss. */
export function t(key: StringKey): string {
  const parts = key.split(".");
  let current: unknown = merged;
  for (const part of parts) {
    if (current === null || typeof current !== "object") return key;
    current = (current as Record<string, unknown>)[part];
  }
  return typeof current === "string" ? current : key;
}
