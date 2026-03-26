/** Convert a UserRecipe to the RecipeListItem shape for list views. */
import { NODE_TYPE_INFO, isIoNodeType, isContainerNodeType } from "@bnto/registry";
import type { NodeTypeName } from "@bnto/registry";
import type { UserRecipe, RecipeListItem } from "../types";

/** Extract unique processing node type labels (excludes I/O and container nodes). */
function extractNodeTypeLabels(nodes: Array<{ type?: string }>): string[] {
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

export function recipeToListItem(recipe: UserRecipe): RecipeListItem {
  const nodes = recipe.definition.nodes ?? [];
  return {
    id: recipe.id,
    name: recipe.name,
    nodeCount: nodes.length,
    nodeTypes: extractNodeTypeLabels(nodes),
    updatedAt: recipe.savedAt ?? 0,
    syncedAt: recipe.syncedAt,
  };
}
