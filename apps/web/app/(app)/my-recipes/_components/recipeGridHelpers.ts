import { NODE_TYPE_INFO } from "@bnto/core";
import type { LucideIcon } from "@bnto/ui";
import { FileIcon, GlobeIcon, ImageIcon, SheetIcon } from "@bnto/ui";
import type { RecipeCategory, RecipeSortOrder } from "./RecipeFilterMenu";

/** Reverse map: node type label -> category. */
const LABEL_TO_CATEGORY: Record<string, string> = {};
for (const info of Object.values(NODE_TYPE_INFO)) {
  LABEL_TO_CATEGORY[info.label] = info.category;
}

/** Map category -> card icon. */
const CATEGORY_ICON: Record<string, LucideIcon> = {
  image: ImageIcon,
  file: FileIcon,
  spreadsheet: SheetIcon,
  network: GlobeIcon,
};

/** Find the dominant domain category icon from a recipe's node type labels. */
export function getCategoryIcon(nodeTypes: string[]): LucideIcon | undefined {
  for (const label of nodeTypes) {
    const icon = CATEGORY_ICON[LABEL_TO_CATEGORY[label]];
    if (icon) return icon;
  }
  return undefined;
}

/** Filter recipes by category and apply sort order. */
export function filterAndSortRecipes<T extends { nodeTypes: string[]; updatedAt: number }>(
  recipes: T[],
  category: RecipeCategory,
  sort: RecipeSortOrder,
): T[] {
  let result = recipes;
  if (category !== "all") {
    result = result.filter((r) =>
      r.nodeTypes.some((label) => LABEL_TO_CATEGORY[label] === category),
    );
  }
  if (sort === "oldest") {
    result = [...result].sort((a, b) => a.updatedAt - b.updatedAt);
  }
  return result;
}
