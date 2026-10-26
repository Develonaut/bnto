import { generatedHeader } from "../generatedHeader";
import { nonProcessorTypesWithParams } from "../schemas/nonProcessorTypesWithParams";
import { toCamelCase } from "../toCamelCase";
import type { CatalogEnvelope } from "../types";
import { typesWithParams } from "../typesWithParams";
import { nodeParamFieldsLiteral } from "./nodeParamFieldsLiteral";

function schemasConst(names: string[]): string {
  const entries = names.map((name) => `  ${JSON.stringify(name)}: ${toCamelCase(name)}NodeSchema`);
  return `/**
 * Schema entries for ALL node types — auto-assembled from the engine catalog.
 * The engine is the single source of truth. No hand-written overrides needed.
 */
export const ENGINE_NODE_SCHEMAS: Record<string, NodeSchema> = {
${entries.join(",\n")},
};`;
}

function paramFieldsConst(catalog: CatalogEnvelope): string {
  const types = typesWithParams(catalog).sort((a, b) => a.name.localeCompare(b.name));
  const entries = types.map(
    (nt) => `  ${JSON.stringify(nt.name)}: ${nodeParamFieldsLiteral(nt.params ?? [])}`,
  );
  return `/**
 * UI field presentation metadata for all node types.
 * Generated from the engine catalog's nodeTypes[].params presentation fields.
 */
export const ENGINE_NODE_PARAM_FIELDS: Record<string, NodeParamFields> = {
${entries.join(",\n")},
};`;
}

/** `engineSchemaEntries.ts`: every schema (processor-backed + non-processor) and field config. */
export function engineSchemaEntriesFile(catalog: CatalogEnvelope): string {
  const names = [
    ...catalog.processors.map((p) => p.nodeType),
    ...nonProcessorTypesWithParams(catalog).map((nt) => nt.name),
  ].sort();
  const imports = names.map(
    (name) => `import { ${toCamelCase(name)}NodeSchema } from "./schemas/${toCamelCase(name)}";`,
  );

  return `${generatedHeader(catalog.version)}

import type { NodeSchema } from "../schemas/types";
import type { NodeParamFields } from "../schemas/types";

${imports.join("\n")}

${schemasConst(names)}

${paramFieldsConst(catalog)}
`;
}
