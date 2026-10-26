import type { RawParameter } from "../types";

/** One param's entry in a `NodeSchema`'s `params` (label, description, …). */
export function nodeParamEntry(param: RawParameter): string {
  const lines: string[] = [];
  lines.push(`    ${param.name}: {`);
  lines.push(`      label: ${JSON.stringify(param.label)},`);
  lines.push(`      description: ${JSON.stringify(param.description)},`);
  if (param.placeholder !== undefined) {
    lines.push(`      placeholder: ${JSON.stringify(param.placeholder)},`);
  }
  if (param.surfaceable === false) {
    lines.push(`      surfaceable: false,`);
  }
  lines.push(`    },`);
  return lines.join("\n");
}
