/**
 * Recipe card for the explore grid.
 *
 * Uses the RecipeCard compound component with a colored hero icon zone.
 * Accepts a `size` prop to vary the hero zone height for masonry layout.
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

type CardSize = "sm" | "md" | "lg";

const HERO_HEIGHT: Record<CardSize, string> = {
  sm: "py-5",
  md: "py-10",
  lg: "py-16",
};

interface ExploreRecipeCardProps {
  recipe: Recipe;
  icon: LucideIcon;
  /** Hero zone height variant for masonry stagger. Default `"md"`. */
  size?: CardSize;
}

export function ExploreRecipeCard({ recipe, icon: Icon, size = "md" }: ExploreRecipeCardProps) {
  const categoryLabel = getCategoryInfo(recipe.category)?.label ?? recipe.category;

  return (
    <RecipeCard href={`/${recipe.slug}`} className="h-auto">
      <div
        data-testid={`explore-recipe-${recipe.slug}`}
        className={`flex items-center justify-center rounded-t-lg bg-muted ${HERO_HEIGHT[size]}`}
      >
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
