import appStrings from "./en.json";
import nodeStrings from "./generated/nodes.json";
import type { StringKey } from "./types";

const merged: Record<string, unknown> = { ...appStrings, ...nodeStrings };

/**
 * Resolve a dot-path key to its string value. Returns the key itself on miss.
 * Supports `{{var}}` interpolation — pass a params object to replace placeholders.
 */
export function t(key: StringKey, params?: Record<string, string | number>): string {
  const parts = key.split(".");
  let current: unknown = merged;
  for (const part of parts) {
    if (current === null || typeof current !== "object") return key;
    current = (current as Record<string, unknown>)[part];
  }
  const raw = typeof current === "string" ? current : key;
  if (!params) return raw;
  return raw.replace(/\{\{(\w+)\}\}/g, (match, name: string) =>
    name in params ? String(params[name]) : match,
  );
}
