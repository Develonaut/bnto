import type { ComponentProps } from "react";
import type { Execution, RecipeListItem } from "@bnto/core";

import { RecipeCard } from "@bnto/ui";

import { SavedRecipeCardSkeleton } from "./SavedRecipeCardSkeleton";
import { SavedRecipeCardContent } from "./SavedRecipeCardContent";

interface SavedRecipeCardProps extends Pick<ComponentProps<typeof RecipeCard>, "loading"> {
  recipe: RecipeListItem;
  lastStatus?: Execution["status"];
  /** Destination link — required for saved recipe navigation. */
  href: string;
}

export function SavedRecipeCard({ recipe, lastStatus, href, loading }: SavedRecipeCardProps) {
  return (
    <RecipeCard href={href} loading={loading}>
      {loading ? (
        <SavedRecipeCardSkeleton />
      ) : (
        <SavedRecipeCardContent recipe={recipe} lastStatus={lastStatus} />
      )}
    </RecipeCard>
  );
}
