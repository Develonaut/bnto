import type { RawNodeTypeParam } from "../types";
import { paramFieldLiteral } from "./paramFieldLiteral";

/** Generate NODE_PARAM_FIELDS entries for a node type. Returns empty object literal if no fields. */
export function nodeParamFieldsLiteral(params: RawNodeTypeParam[]): string {
  const fieldEntries = params.map((p) => paramFieldLiteral(p)).filter((s) => s.length > 0);

  if (fieldEntries.length === 0) return "{}";
  return `{\n${fieldEntries.join(",\n")},\n  }`;
}
