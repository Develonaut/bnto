"use client";

import { useCallback } from "react";
import { RECIPES } from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import {
  Card,
  RecipeCardIcon,
  RecipeCardTitle,
  RecipeCardCategory,
  RecipeCardTags,
  Button,
  Stack,
  Text,
  type LucideIcon,
} from "@bnto/ui";

/**
 * RecipePickerGrid — compact list of predefined recipes.
 *
 * Uses Card directly (not RecipeCard) because selecting a recipe
 * is an inline action, not a navigation. RecipeCard is for links.
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
        <Button
          key={recipe.slug}
          asChild
          className="flex-1 text-left"
          onClick={handleSelect(recipe.definition)}
        >
          <Card className="flex flex-1 flex-row items-center gap-3 p-3">
            <RecipeCardIcon icon={getIcon?.(recipe.slug)} />
            <Stack className="min-w-0 flex-1 gap-1">
              <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
              <RecipeCardTags tags={recipe.features} limit={3} />
            </Stack>
            <RecipeCardCategory>{recipe.category}</RecipeCardCategory>
          </Card>
        </Button>
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
