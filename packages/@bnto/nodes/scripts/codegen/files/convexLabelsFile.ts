import type { CatalogEnvelope } from "../types";
import { labeledNodeTypes } from "./labeledNodeTypes";

/** Convex's `nodeTypeLabels.ts`: display labels for processing node types. */
export function convexLabelsFile(catalog: CatalogEnvelope): string {
  const entries = labeledNodeTypes(catalog)
    .map((t) => `  ${JSON.stringify(t.name)}: ${JSON.stringify(t.label)},`)
    .join("\n");

  return `/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 *
 * Convex can't import from @bnto/nodes (bundling constraint), so this
 * file is generated alongside the main catalog. Run \`task nodes:generate\`
 * to regenerate after engine changes.
 *
 * Engine catalog v${catalog.version}
 */

/** Node type name → display label for processing nodes (excludes I/O and containers). */
export const NODE_TYPE_LABELS: Record<string, string> = {
${entries}
};
`;
}
