import { serializeValue } from "../serializeValue";
import type { RawNodeTypeParam, RawParameter } from "../types";

// shell-run args support nested arrays as conditional groups
// (flattened by the engine's flatten_conditional_args()) and
// `{ each: [...] }` groups, repeated once per incoming file.
const SHELL_ARGS_ZOD =
  "z.array(z.union([z.string(), z.array(z.string()), z.object({ each: z.array(z.string()) }).strict()]))";

// File params are base64-encoded strings.
const PLAIN_TYPE_ZOD = new Map([
  ["string", "z.string()"],
  ["boolean", "z.boolean()"],
  ["object", "z.record(z.unknown())"],
  ["file", "z.string()"],
]);

/** Control-based overrides: tagPicker → array, keyValue → record. */
function controlZod(control: string | undefined, param: RawParameter): string | undefined {
  if (control === "tagPicker" && param.name === "args") return SHELL_ARGS_ZOD;
  if (control === "tagPicker") return "z.array(z.string())";
  if (control === "keyValue" && param.paramType.type === "object") return "z.record(z.unknown())";
  if (control === "keyValue") return "z.record(z.string())";
  return undefined;
}

function typeZod(param: RawParameter): string {
  const { type, options } = param.paramType;
  if (type === "number") {
    const min = param.constraints?.min;
    const max = param.constraints?.max;
    return `z.number()${min !== undefined ? `.min(${min})` : ""}${max !== undefined ? `.max(${max})` : ""}`;
  }
  if (type === "enum") {
    const values = (options ?? []).map((o) => o.value);
    return `z.enum(${JSON.stringify(values)} as const)`;
  }
  return PLAIN_TYPE_ZOD.get(type) ?? "z.unknown()";
}

/** Generate a single Zod field from a RawParameter (or extended RawNodeTypeParam). */
export function zodField(param: RawParameter | RawNodeTypeParam): string {
  const control = "control" in param ? (param as RawNodeTypeParam).control : undefined;
  const chain = controlZod(control, param) ?? typeZod(param);
  const optional = param.constraints?.required ? "" : ".optional()";
  const fallback = param.default !== undefined ? `.default(${serializeValue(param.default)})` : "";
  return `  ${param.name}: ${chain}${optional}${fallback}`;
}
