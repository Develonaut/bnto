/**
 * findDefinitionById — recursively search for a definition by ID in a tree.
 *
 * Used by the export adapter and useEditorNode hook to locate
 * container/leaf definitions within the nested definition tree.
 */

import type { Definition } from "@bnto/nodes";

function findDefinitionById(def: Definition, id: string): Definition | null {
  if (def.id === id) return def;
  for (const child of def.nodes ?? []) {
    const found = findDefinitionById(child, id);
    if (found) return found;
  }
  return null;
}

export { findDefinitionById };
