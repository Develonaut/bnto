import type { RawNodeType, RawProcessor } from "../types";

/** Each param's default, or a placeholder for a required one without a default. */
function exampleParams(processor: RawProcessor): Record<string, unknown> {
  const params: Record<string, unknown> = {};
  for (const param of processor.parameters) {
    if (param.default !== undefined) {
      params[param.name] = param.default;
    } else if (param.constraints?.required) {
      params[param.name] = param.paramType.options?.[0]?.value ?? `<${param.paramType.type}>`;
    }
  }
  return params;
}

/** A node README's "Configuration Example" section. */
export function configExample(nodeType: RawNodeType, processor: RawProcessor): string[] {
  const params = JSON.stringify(exampleParams(processor), null, 4)
    .split("\n")
    .map((l, i) => (i === 0 ? l : "  " + l))
    .join("\n");
  return [
    "## Configuration Example",
    "",
    "```json",
    `{`,
    `  "type": ${JSON.stringify(nodeType.name)},`,
    `  "parameters": ${params}`,
    `}`,
    "```",
    "",
  ];
}
