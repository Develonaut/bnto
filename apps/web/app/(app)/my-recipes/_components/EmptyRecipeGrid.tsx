"use client";

import {
  Button,
  EmptyState,
  EmptyStateIcon,
  EmptyStateTitle,
  EmptyStateDescription,
  FolderOpenIcon,
  PlusIcon,
} from "@bnto/ui";

interface EmptyRecipeGridProps {
  isFiltered: boolean;
}

/**
 * Empty state for the recipe grid.
 *
 * Shows a contextual message depending on whether the user
 * has an active category filter or no recipes at all.
 */
export function EmptyRecipeGrid({ isFiltered }: EmptyRecipeGridProps) {
  return (
    <div className="min-h-[240px]">
      <EmptyState>
        <EmptyStateIcon>
          <FolderOpenIcon />
        </EmptyStateIcon>
        <EmptyStateTitle>
          {isFiltered ? "No matching recipes" : "No saved recipes yet"}
        </EmptyStateTitle>
        <EmptyStateDescription>
          {isFiltered
            ? "Try a different filter or create a new recipe."
            : "Create a recipe in the editor — it will auto-save here."}
        </EmptyStateDescription>
        <Button href="/editor" variant="primary" elevation="sm">
          <PlusIcon />
          New Recipe
        </Button>
      </EmptyState>
    </div>
  );
}
