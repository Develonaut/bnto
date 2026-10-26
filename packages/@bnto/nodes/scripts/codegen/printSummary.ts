import { extractIterationModes } from "./catalog/extractIterationModes";
import type { Writer } from "./createWriter";
import { labeledNodeTypes } from "./files/labeledNodeTypes";
import { CONVEX_LABELS_OUTPUT, I18N_STRINGS_OUTPUT } from "./paths";
import { nonProcessorTypesWithParams } from "./schemas/nonProcessorTypesWithParams";
import type { CatalogEnvelope } from "./types";
import { typesWithParams } from "./typesWithParams";

function printSchemaCounts(catalog: CatalogEnvelope): void {
  const nonProcCount = nonProcessorTypesWithParams(catalog).length;
  const allSchemaCount = catalog.processors.length + nonProcCount;
  const allParamCount = typesWithParams(catalog).length;
  console.log(`  catalog/nodeTypes/ - ${catalog.nodeTypes.length} per-type files`);
  console.log(`  catalog/processors/ - ${catalog.processors.length} per-processor files`);
  console.log(
    `  schemas/ - ${allSchemaCount} Zod schemas (${catalog.processors.length} processor + ${nonProcCount} non-processor)`,
  );
  console.log(
    `  engineSchemaEntries.ts - ${allSchemaCount} schemas + ${allParamCount} field configs`,
  );
  console.log(`  paramFieldInfo.ts - ${allParamCount} node types with field info`);
}

function printFileList(catalog: CatalogEnvelope): void {
  const modes = extractIterationModes(catalog);
  console.log(`  enumConstants.ts - generated`);
  console.log(`  definitionSchema.ts - ${catalog.definitionSchema ? "included" : "NOT included"}`);
  console.log(`  formatVersion.ts - v${catalog.version}`);
  if (modes) console.log(`  iterationModes.ts - ${modes.join(", ")}`);
  console.log(`  ${CONVEX_LABELS_OUTPUT} - ${labeledNodeTypes(catalog).length} labels`);
  console.log(`  ${I18N_STRINGS_OUTPUT} - i18n strings`);
  if (catalog.recipes?.length) {
    console.log(`  recipes.ts - ${catalog.recipes.length} built-in recipes`);
  }
}

/** Log what codegen wrote. */
export function printSummary(out: Writer, catalog: CatalogEnvelope): void {
  const { written, unchanged } = out.counts;
  console.log(`\nCodegen complete (${written} written, ${unchanged} unchanged):`);
  printSchemaCounts(catalog);
  printFileList(catalog);
}
