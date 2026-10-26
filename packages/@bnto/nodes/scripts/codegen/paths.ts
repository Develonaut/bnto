/** Where node codegen reads the catalog and writes each output. */

import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../..");

export const CATALOG_PATH = resolve(ROOT, "engine/catalog.snapshot.json");
export const GENERATED_DIR = resolve(ROOT, "packages/@bnto/nodes/src/generated");
export const CATALOG_DIR = resolve(GENERATED_DIR, "catalog");
export const NODE_TYPES_DIR = resolve(CATALOG_DIR, "nodeTypes");
export const PROCESSORS_DIR = resolve(CATALOG_DIR, "processors");
export const SCHEMAS_DIR = resolve(GENERATED_DIR, "schemas");
export const DEF_SCHEMA_OUTPUT = resolve(GENERATED_DIR, "definitionSchema.ts");
export const FORMAT_VERSION_OUTPUT = resolve(GENERATED_DIR, "formatVersion.ts");
export const ENGINE_SCHEMA_ENTRIES_OUTPUT = resolve(GENERATED_DIR, "engineSchemaEntries.ts");
export const DOCS_DIR = resolve(ROOT, "packages/@bnto/nodes/docs");
export const CONVEX_LABELS_OUTPUT = resolve(
  ROOT,
  "packages/@bnto/backend/convex/_helpers/nodeTypeLabels.ts",
);
export const I18N_STRINGS_OUTPUT = resolve(ROOT, "packages/@bnto/i18n/src/generated/nodes.json");
export const RECIPES_OUTPUT = resolve(GENERATED_DIR, "recipes.ts");
export const PARAM_FIELD_INFO_OUTPUT = resolve(GENERATED_DIR, "paramFieldInfo.ts");
export const ENUM_CONSTANTS_OUTPUT = resolve(GENERATED_DIR, "enumConstants.ts");

/** Monolithic files from before the per-file layout, removed on every run. */
export const LEGACY_FILES = [
  resolve(GENERATED_DIR, "catalog.ts"),
  resolve(GENERATED_DIR, "schemas.ts"),
  resolve(GENERATED_DIR, "nodeTypes.ts"),
];
