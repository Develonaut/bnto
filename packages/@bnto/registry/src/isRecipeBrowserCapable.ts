/** Check if all processing nodes in a recipe can run in the browser. */

import type { Definition } from "@bnto/nodes";
import { NODE_TYPE_INFO } from "./catalog";
import { isIoNodeType, isContainerNodeType } from "./classification";
import type { Recipe } from "./recipe";
import type { NodeTypeName } from "./types";

export function isRecipeBrowserCapable(recipe: Recipe): boolean {
  return allRunInBrowser(recipe.definition.nodes ?? []);
}

/** Every step in `nodes`, containers' children included, runs in the browser. */
function allRunInBrowser(nodes: readonly Definition[]): boolean {
  return nodes.every((n) => {
    if (isIoNodeType(n.type)) return true;
    if (isContainerNodeType(n.type)) return allRunInBrowser(n.nodes ?? []);
    return NODE_TYPE_INFO[n.type as NodeTypeName]?.platforms.includes("browser") ?? false;
  });
}
