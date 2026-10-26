import type { Writer } from "../createWriter";
import { generatedHeader } from "../generatedHeader";
import { SCHEMAS_DIR } from "../paths";
import type { CatalogEnvelope } from "../types";
import { writePerItemFolder } from "../writePerItemFolder";
import { nonProcessorTypesWithParams } from "./nonProcessorTypesWithParams";
import { schemaFile } from "./schemaFile";
import { schemasBarrel } from "./schemasBarrel";

/** Everything under `src/generated/schemas/` (processor-backed + non-processor types). */
export function writeSchemaFiles(out: Writer, catalog: CatalogEnvelope): void {
  const header = generatedHeader(catalog.version);
  const sources = [
    ...catalog.processors.map((p) => ({ name: p.nodeType, params: p.parameters })),
    ...nonProcessorTypesWithParams(catalog).map((nt) => ({
      name: nt.name,
      params: nt.params ?? [],
    })),
  ];
  const files = sources.map(({ name, params }) => ({
    name,
    content: schemaFile(name, params, header),
  }));
  writePerItemFolder(out, SCHEMAS_DIR, files, schemasBarrel(catalog));
}
