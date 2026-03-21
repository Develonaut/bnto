import { NODE_TYPE_INFO, isIoNodeType, isContainerNodeType } from "@bnto/registry";
import type { NodeTypeName } from "@bnto/registry";
import type { RawRecipeDoc, RawRecipeListProjection } from "../types/raw";
import type { UserRecipe, RecipeListItem } from "../types";

/** Cloud recipe detail — internal shape for Convex detail queries. */
export interface CloudRecipeDetail {
  id: string;
  userId: string;
  name: string;
  definition: RawRecipeDoc["definition"];
  version: number;
  formatVersion?: string;
  isPublic: boolean;
  createdAt: number;
  updatedAt: number;
}

export function toRecipe(doc: RawRecipeDoc): CloudRecipeDetail {
  return {
    id: String(doc._id),
    userId: String(doc.userId),
    name: doc.name,
    definition: doc.definition,
    version: doc.version,
    formatVersion: doc.formatVersion,
    isPublic: doc.isPublic,
    createdAt: doc.createdAt,
    updatedAt: doc.updatedAt,
  };
}

export function toRecipeListItem(doc: RawRecipeListProjection): RecipeListItem {
  return {
    id: String(doc._id),
    name: doc.name,
    nodeCount: doc.nodeCount,
    nodeTypes: doc.nodeTypes ?? [],
    updatedAt: doc.updatedAt,
  };
}

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

/** Convert a UserRecipe to the RecipeListItem shape for list views. */
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
