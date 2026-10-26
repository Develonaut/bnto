import { toCamelCase } from "./toCamelCase";

/** Convert kebab-case to PascalCase. */
export function toPascalCase(s: string): string {
  const camel = toCamelCase(s);
  return camel.charAt(0).toUpperCase() + camel.slice(1);
}
