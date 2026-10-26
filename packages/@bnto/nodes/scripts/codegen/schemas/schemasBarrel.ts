import { generatedHeader } from "../generatedHeader";
import { toCamelCase } from "../toCamelCase";
import { toPascalCase } from "../toPascalCase";
import type { CatalogEnvelope } from "../types";
import { nonProcessorTypesWithParams } from "./nonProcessorTypesWithParams";

function reexport(nodeType: string): string {
  const camelName = toCamelCase(nodeType);
  return `export { ${camelName}ParamsSchema, ${camelName}NodeSchema } from "./${camelName}";
export type { ${toPascalCase(nodeType)}Params } from "./${camelName}";`;
}

/** `schemas/index.ts`: processor-backed schemas, then the non-processor ones, each sorted. */
export function schemasBarrel(catalog: CatalogEnvelope): string {
  const processorTypes = catalog.processors
    .map((p) => p.nodeType)
    .sort((a, b) => a.localeCompare(b));
  const otherTypes = nonProcessorTypesWithParams(catalog)
    .map((nt) => nt.name)
    .sort((a, b) => a.localeCompare(b));
  const reexports = [...processorTypes, ...otherTypes].map(reexport);

  return `${generatedHeader(catalog.version)}

${reexports.join("\n")}
`;
}
