import type { RawNodeTypeParam } from "../types";

/** A param's `NodeParamFieldInfo`, worked out at codegen time. */
export interface FieldInfo {
  type: string;
  control: string;
  required: boolean;
  enumValues?: string[];
  min?: number;
  max?: number;
}

type InfoFor = (param: RawNodeTypeParam, required: boolean) => FieldInfo;

const enumInfo: InfoFor = (param, required) => ({
  type: "enum",
  control: param.control ?? "select",
  required,
  enumValues: (param.paramType.options ?? []).map((o) => o.value),
});

/** A number with both bounds gets a slider unless the engine names a control. */
const numberInfo: InfoFor = (param, required) => {
  const min = param.constraints?.min;
  const max = param.constraints?.max;
  const isBounded = min !== undefined && max !== undefined;
  return {
    type: "number",
    control: param.control ?? (isBounded ? "slider" : "number"),
    required,
    min,
    max,
  };
};

const stringInfo: InfoFor = (param, required) => ({
  type: "string",
  control: param.control ?? "text",
  required,
});

const INFO_BY_TYPE = new Map<string, InfoFor>([
  ["enum", enumInfo],
  ["boolean", (p, required) => ({ type: "boolean", control: p.control ?? "switch", required })],
  ["number", numberInfo],
  ["file", (p, required) => ({ type: "string", control: p.control ?? "file", required })],
  ["object", (p, required) => ({ type: "record", control: p.control ?? "keyValue", required })],
]);

/** Compute NodeParamFieldInfo for a parameter at codegen time. */
export function computeFieldInfo(param: RawNodeTypeParam): FieldInfo {
  const required = param.constraints?.required === true && param.default === undefined;
  if (param.control === "tagPicker") return { type: "array", control: "tagPicker", required };
  if (param.control === "keyValue") return { type: "record", control: "keyValue", required };
  return (INFO_BY_TYPE.get(param.paramType.type) ?? stringInfo)(param, required);
}
