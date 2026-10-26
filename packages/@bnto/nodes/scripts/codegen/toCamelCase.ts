/** Convert kebab-case to camelCase. */
export function toCamelCase(s: string): string {
  return s.replace(/-([a-z])/g, (_, c) => c.toUpperCase());
}
