import { generatedHeader } from "../generatedHeader";
import type { CatalogEnvelope, RawNodeType, RawNodeTypeParam } from "../types";
import { typesWithParams } from "../typesWithParams";
import { computeFieldInfo } from "./computeFieldInfo";

const LOOKUP = `/** Look up pre-computed field info for a specific node type and parameter. */
export function getParamFieldInfo(
  nodeType: string,
  paramName: string,
): NodeParamFieldInfo | undefined {
  return NODE_PARAM_FIELD_INFO[nodeType]?.[paramName];
}`;

function paramEntry(p: RawNodeTypeParam): string {
  const info = computeFieldInfo(p);
  const parts: string[] = [];
  parts.push(`type: ${JSON.stringify(info.type)}`);
  parts.push(`control: ${JSON.stringify(info.control)}`);
  parts.push(`required: ${info.required}`);
  if (info.enumValues) parts.push(`enumValues: ${JSON.stringify(info.enumValues)} as const`);
  if (info.min !== undefined) parts.push(`min: ${info.min}`);
  if (info.max !== undefined) parts.push(`max: ${info.max}`);
  return `    ${JSON.stringify(p.name)}: { ${parts.join(", ")} }`;
}

function typeEntry(nt: RawNodeType): string {
  const params = (nt.params ?? []).map(paramEntry);
  return `  ${JSON.stringify(nt.name)}: {\n${params.join(",\n")},\n  }`;
}

/** `paramFieldInfo.ts`: the static field info map (replaces runtime Zod introspection). */
export function paramFieldInfoFile(catalog: CatalogEnvelope): string {
  const types = typesWithParams(catalog).sort((a, b) => a.name.localeCompare(b.name));

  return `${generatedHeader(catalog.version)}

import type { NodeParamFieldInfo } from "../schemas/types";

/** Pre-computed field info for all node parameters — replaces runtime Zod introspection. */
export const NODE_PARAM_FIELD_INFO: Record<string, Record<string, NodeParamFieldInfo>> = {
${types.map(typeEntry).join(",\n")},
};

${LOOKUP}
`;
}
