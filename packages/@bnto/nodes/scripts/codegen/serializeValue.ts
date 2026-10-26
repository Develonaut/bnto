/** A param value as a TypeScript literal (`undefined` for null or missing). */
export function serializeValue(v: unknown): string {
  if (v === null || v === undefined) return "undefined";
  if (typeof v === "string") return JSON.stringify(v);
  if (typeof v === "number" || typeof v === "boolean") return String(v);
  return JSON.stringify(v);
}
