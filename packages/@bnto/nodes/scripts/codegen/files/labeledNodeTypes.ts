import type { CatalogEnvelope, RawNodeType } from "../types";

/** Processing node types: no I/O nodes, no containers. */
export function labeledNodeTypes(catalog: CatalogEnvelope): RawNodeType[] {
  return catalog.nodeTypes.filter((t) => !t.isContainer && t.category !== "io");
}
