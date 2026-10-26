import type { CatalogEnvelope, RawNodeType } from "../types";

/** Node types with params but no processor; their schemas come from `nodeTypes[].params`. */
export function nonProcessorTypesWithParams(catalog: CatalogEnvelope): RawNodeType[] {
  const processorNodeTypes = new Set(catalog.processors.map((p) => p.nodeType));
  return catalog.nodeTypes.filter(
    (nt) => !processorNodeTypes.has(nt.name) && nt.params && nt.params.length > 0,
  );
}
