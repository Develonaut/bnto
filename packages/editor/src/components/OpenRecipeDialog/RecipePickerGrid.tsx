"use client";

import { useCallback } from "react";
import { RECIPES } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import {
  RecipeCard,
  RecipeCardIcon,
  RecipeCardTitle,
  RecipeCardCategory,
  RecipeCardTags,
  Stack,
  Text,
  type LucideIcon,
} from "@bnto/ui";

/**
 * RecipePickerGrid — compact list of predefined recipes.
 *
 * Uses RecipeCard compact mode for horizontal row layout:
 * icon | title + tags | category. Scrolls independently within the dialog.
 */

interface RecipePickerGridProps {
  onSelect: (definition: Definition) => void;
  getIcon?: (slug: string) => LucideIcon;
}

function RecipePickerGrid({ onSelect, getIcon }: RecipePickerGridProps) {
  const handleSelect = useCallback(
    (definition: Definition) => () => onSelect(definition),
    [onSelect],
  );

  return (
    <div className="flex min-h-0 flex-1 flex-col gap-2 overflow-y-auto p-1">
      {RECIPES.map((recipe) => (
        <RecipeCard key={recipe.slug} compact onClick={handleSelect(recipe.definition)}>
          <RecipeCardIcon icon={getIcon?.(recipe.slug)} />
          <Stack className="min-w-0 flex-1 gap-1">
            <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
            <RecipeCardTags tags={recipe.features} limit={3} />
          </Stack>
          <RecipeCardCategory>{recipe.category}</RecipeCardCategory>
        </RecipeCard>
      ))}
      {RECIPES.length === 0 && (
        <Text size="sm" className="py-4 text-center text-muted-foreground">
          No recipes available
        </Text>
      )}
    </div>
  );
}

export { RecipePickerGrid };
