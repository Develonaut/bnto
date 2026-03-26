/**
 * Recipe card for the explore grid.
 *
 * Uses the RecipeCard compound component with a colored hero icon zone.
 */

import type { Recipe } from "@bnto/core";
import type { LucideIcon } from "@bnto/ui";
import {
  RecipeCard,
  RecipeCardCategory,
  RecipeCardContent,
  RecipeCardDescription,
  RecipeCardTags,
  RecipeCardTitle,
} from "@bnto/ui";
import { getCategoryInfo } from "@bnto/registry";

export function ExploreRecipeCard({ recipe, icon: Icon }: { recipe: Recipe; icon: LucideIcon }) {
  const categoryLabel = getCategoryInfo(recipe.category)?.label ?? recipe.category;

  return (
    <RecipeCard href={`/${recipe.slug}`}>
      <div className="flex items-center justify-center rounded-t-lg bg-muted py-6">
        <Icon className="size-10 text-primary" />
      </div>
      <RecipeCardContent>
        <RecipeCardCategory>{categoryLabel}</RecipeCardCategory>
        <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
        <RecipeCardDescription>{recipe.description}</RecipeCardDescription>
        <RecipeCardTags tags={recipe.features} limit={3} />
      </RecipeCardContent>
    </RecipeCard>
  );
}
