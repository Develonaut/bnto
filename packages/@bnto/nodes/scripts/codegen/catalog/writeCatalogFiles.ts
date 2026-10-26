import { resolve } from "node:path";
import type { Writer } from "../createWriter";
import { generatedHeader } from "../generatedHeader";
import { CATALOG_DIR, NODE_TYPES_DIR, PROCESSORS_DIR } from "../paths";
import type { CatalogEnvelope } from "../types";
import { writePerItemFolder } from "../writePerItemFolder";
import { catalogBarrel } from "./catalogBarrel";
import { catalogTypesFile } from "./catalogTypesFile";
import { extractIterationModes } from "./extractIterationModes";
import { iterationModesFile } from "./iterationModesFile";
import { nodeTypeFile } from "./nodeTypeFile";
import { nodeTypesBarrel } from "./nodeTypesBarrel";
import { processorFile } from "./processorFile";
import { processorLookupsFile } from "./processorLookupsFile";
import { processorsBarrel } from "./processorsBarrel";

/** `catalog/types.ts`, `processorLookups.ts`, `iterationModes.ts` and `index.ts`. */
function writeCatalogRoot(out: Writer, catalog: CatalogEnvelope): void {
  const { version } = catalog;
  const modes = extractIterationModes(catalog);
  out.write(resolve(CATALOG_DIR, "types.ts"), catalogTypesFile(catalog));
  out.write(resolve(CATALOG_DIR, "processorLookups.ts"), processorLookupsFile(version));
  if (modes) {
    out.write(resolve(CATALOG_DIR, "iterationModes.ts"), iterationModesFile(version, modes));
  }
  out.write(resolve(CATALOG_DIR, "index.ts"), catalogBarrel(version, modes !== null));
}

/** Everything under `src/generated/catalog/`. */
export function writeCatalogFiles(out: Writer, catalog: CatalogEnvelope): void {
  const header = generatedHeader(catalog.version);
  const nodeTypes = catalog.nodeTypes.map((t) => ({
    name: t.name,
    content: nodeTypeFile(t, header),
  }));
  writePerItemFolder(out, NODE_TYPES_DIR, nodeTypes, nodeTypesBarrel(catalog));
  const processors = catalog.processors.map((p) => ({
    name: p.nodeType,
    content: processorFile(p, header),
  }));
  writePerItemFolder(out, PROCESSORS_DIR, processors, processorsBarrel(catalog));
  writeCatalogRoot(out, catalog);
}
