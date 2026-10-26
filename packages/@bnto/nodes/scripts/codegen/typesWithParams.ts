import type { CatalogEnvelope, RawNodeType } from "./types";

/** All node types with params (both processor-backed and non-processor). */
export function typesWithParams(catalog: CatalogEnvelope): RawNodeType[] {
  return catalog.nodeTypes.filter((nt) => nt.params && nt.params.length > 0);
}
