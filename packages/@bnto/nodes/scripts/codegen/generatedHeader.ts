/** The doc comment that opens most generated TypeScript files. */
export function generatedHeader(version: string): string {
  return `/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run \`task nodes:generate\` to regenerate after engine changes.
 * Engine catalog v${version}
 */`;
}
