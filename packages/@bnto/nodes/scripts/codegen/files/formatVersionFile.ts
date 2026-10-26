/** `formatVersion.ts`: the catalog format version. */
export function formatVersionFile(version: string): string {
  return `/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 *
 * The catalog format version, extracted from the engine catalog.
 * Run \`task nodes:generate\` to regenerate after engine changes.
 *
 * Engine catalog v${version}
 */

/** The format version from the engine catalog. */
export const CATALOG_FORMAT_VERSION = ${JSON.stringify(version)};
`;
}
