import { NODE_TYPE_INFO, isIoNodeType, isContainerNodeType } from "@bnto/nodes";
import type { NodeTypeName } from "@bnto/nodes";
import type { RecipeListItem, StoredRecipe } from "../types";

/** Extract unique processing node type labels (excludes I/O and container nodes). */
export function extractNodeTypeLabels(nodes: Array<{ type?: string }>): string[] {
  const seen = new Set<string>();
  const labels: string[] = [];
  for (const node of nodes) {
    if (!node.type || seen.has(node.type)) continue;
    if (isIoNodeType(node.type) || isContainerNodeType(node.type)) continue;
    seen.add(node.type);
    const info = NODE_TYPE_INFO[node.type as NodeTypeName];
    labels.push(info?.label ?? node.type);
  }
  return labels;
}

/** Convert a StoredRecipe to the RecipeListItem shape for list views. */
export function storedRecipeToListItem(recipe: StoredRecipe): RecipeListItem {
  const nodes = recipe.definition.nodes ?? [];
  return {
    id: recipe.metadata.id,
    name: recipe.metadata.name,
    nodeCount: nodes.length,
    nodeTypes: extractNodeTypeLabels(nodes),
    updatedAt: recipe.savedAt,
    syncedAt: recipe.syncedAt,
  };
}
