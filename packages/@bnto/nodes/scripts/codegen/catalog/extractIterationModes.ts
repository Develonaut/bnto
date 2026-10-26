import type { CatalogEnvelope } from "../types";

/** The `PipelineSettings.iteration` enum from the definition schema, if it has one. */
export function extractIterationModes(catalog: CatalogEnvelope): string[] | null {
  if (!catalog.definitionSchema) return null;
  const defs = (catalog.definitionSchema as Record<string, unknown>).$defs as
    | Record<string, unknown>
    | undefined;
  if (!defs) return null;
  const ps = defs.PipelineSettings as
    | { properties?: { iteration?: { enum?: string[] } } }
    | undefined;
  return ps?.properties?.iteration?.enum ?? null;
}
