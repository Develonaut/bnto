"use client";

import { ChevronDownIcon, MenuTrigger, SlidersHorizontalIcon } from "@bnto/ui";

interface RecipeFilterTriggerProps {
  activeLabel: string;
}

export function RecipeFilterTrigger({ activeLabel }: RecipeFilterTriggerProps) {
  return (
    <MenuTrigger variant="outline" elevation="sm" data-testid="recipe-filter-trigger">
      <SlidersHorizontalIcon />
      {activeLabel}
      <ChevronDownIcon />
    </MenuTrigger>
  );
}
