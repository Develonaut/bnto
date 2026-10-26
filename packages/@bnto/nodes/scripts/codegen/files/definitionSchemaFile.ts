import type { CatalogEnvelope } from "../types";

const UNAVAILABLE = `/**
 * Definition JSON Schema - NOT AVAILABLE (engine did not include it).
 */
export const DEFINITION_JSON_SCHEMA = null;
`;

/** `definitionSchema.ts`: the JSON Schema for `.bnto.json` files. */
export function definitionSchemaFile(catalog: CatalogEnvelope): string {
  if (!catalog.definitionSchema) return UNAVAILABLE;

  return `/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 *
 * JSON Schema (Draft 2020-12) for \`.bnto.json\` definition files.
 * Use this to validate definition files in any language or tool
 * that supports JSON Schema (VS Code, ajv, jsonschema, etc.).
 *
 * Run \`task nodes:generate\` to regenerate after engine changes.
 *
 * Engine catalog v${catalog.version}
 */

/** JSON Schema for .bnto.json definition files. */
export const DEFINITION_JSON_SCHEMA = ${JSON.stringify(catalog.definitionSchema, null, 2)} as const;
`;
}
