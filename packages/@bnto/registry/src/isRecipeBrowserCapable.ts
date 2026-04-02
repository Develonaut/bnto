/** Check if all processing nodes in a recipe can run in the browser. */

import { NODE_TYPE_INFO } from "./catalog";
import { isIoNodeType, isContainerNodeType } from "./classification";
import type { Recipe } from "./recipe";
import type { NodeTypeName } from "./types";

export function isRecipeBrowserCapable(recipe: Recipe): boolean {
  const nodes = recipe.definition.nodes ?? [];
  return nodes
    .filter((n) => !isIoNodeType(n.type) && !isContainerNodeType(n.type))
    .every((n) => NODE_TYPE_INFO[n.type as NodeTypeName]?.platforms.includes("browser") ?? false);
}
