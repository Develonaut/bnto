"use client";

import { useCallback } from "react";
import type { Recipe } from "@bnto/core";
import { getAllRecipes, isRecipeBrowserCapable } from "@bnto/registry";
import { Divider } from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import { RecipeMasonryGrid, SectionHeader, SectionShell } from "@/components/sections";
import { ExploreRecipeCard } from "../../explore/_components/ExploreRecipeCard";

type CardSize = "sm" | "md" | "lg";

const SIZE_PATTERN: CardSize[] = ["lg", "sm", "md", "sm", "lg", "md", "md", "lg", "sm"];
const getCardSize = (index: number) => SIZE_PATTERN[index % SIZE_PATTERN.length];
const getRecipeKey = (r: Recipe) => r.id;

interface RelatedRecipesSectionProps {
  category: string;
  currentSlug: string;
}

export function RelatedRecipesSection({ category, currentSlug }: RelatedRecipesSectionProps) {
  const related = getAllRecipes().filter(
    (r) => r.category === category && r.slug !== currentSlug && isRecipeBrowserCapable(r),
  );

  const renderCard = useCallback(
    (recipe: Recipe, gridIndex: number) => (
      <ExploreRecipeCard
        recipe={recipe}
        icon={getBntoIcon(recipe.slug)}
        size={getCardSize(gridIndex)}
      />
    ),
    [],
  );

  if (related.length === 0) return null;

  return (
    <>
      <Divider label="Related tools" />
      <SectionShell muted>
        <SectionHeader title={`More ${category} tools`} />
        <RecipeMasonryGrid items={related} keyOf={getRecipeKey} renderCard={renderCard} />
      </SectionShell>
    </>
  );
}
