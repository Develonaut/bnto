import { serializeValue } from "../serializeValue";
import type { RawParameter } from "../types";

function typeLines(p: RawParameter): string[] {
  if (p.paramType.options) {
    const values = p.paramType.options.map((o) => o.value);
    return [`  type: "enum" as const,`, `  options: ${JSON.stringify(values)} as const,`];
  }
  return [`  type: ${JSON.stringify(p.paramType.type)} as const,`];
}

function constraintsLine(c: NonNullable<RawParameter["constraints"]>): string {
  const parts: string[] = [];
  if (c.min !== undefined) parts.push(`min: ${c.min}`);
  if (c.max !== undefined) parts.push(`max: ${c.max}`);
  if (c.required !== undefined) parts.push(`required: ${c.required}`);
  return `  constraints: { ${parts.join(", ")} },`;
}

function optionalLines(p: RawParameter): string[] {
  const lines: string[] = [];
  if (p.paramType.accept) lines.push(`  accept: ${JSON.stringify(p.paramType.accept)},`);
  if (p.default !== undefined) lines.push(`  default: ${serializeValue(p.default)},`);
  if (p.constraints) lines.push(constraintsLine(p.constraints));
  if (p.placeholder !== undefined) lines.push(`  placeholder: ${JSON.stringify(p.placeholder)},`);
  if (p.surfaceable === false) lines.push(`  surfaceable: false,`);
  return lines;
}

/** One `ProcessorParam` object literal, unindented. */
export function paramLiteral(p: RawParameter): string {
  return [
    "{",
    `  name: ${JSON.stringify(p.name)},`,
    `  label: ${JSON.stringify(p.label)},`,
    `  description: ${JSON.stringify(p.description)},`,
    ...typeLines(p),
    ...optionalLines(p),
    "}",
  ].join("\n");
}
