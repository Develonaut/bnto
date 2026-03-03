import type { Execution, RecipeListItem } from "@bnto/core";

import { RecipeCard } from "@/components/blocks/RecipeCard";
import { Skeleton } from "@/components/ui/Skeleton";

interface SavedRecipeCardProps {
  recipe: RecipeListItem;
  lastStatus?: Execution["status"];
  onClick?: () => void;
  /** Grounded loading state — card springs up when loading clears. */
  loading?: boolean;
}

export function SavedRecipeCard({ recipe, lastStatus, onClick, loading }: SavedRecipeCardProps) {
  return (
    <RecipeCard onClick={onClick} loading={loading}>
      {loading ? (
        <>
          <RecipeCard.Header>
            <Skeleton className="size-10 rounded-lg" />
            <Skeleton className="h-5 w-16 rounded-full" />
          </RecipeCard.Header>
          <RecipeCard.Content>
            <Skeleton className="h-5 w-3/4" />
            <Skeleton className="h-4 w-1/2" />
          </RecipeCard.Content>
        </>
      ) : (
        <>
          <RecipeCard.Header>
            <RecipeCard.Icon />
            {lastStatus && <RecipeCard.Status status={lastStatus} />}
          </RecipeCard.Header>
          <RecipeCard.Content>
            <RecipeCard.Title>{recipe.name}</RecipeCard.Title>
            <RecipeCard.Meta nodeCount={recipe.nodeCount} updatedAt={recipe.updatedAt} />
          </RecipeCard.Content>
        </>
      )}
    </RecipeCard>
  );
}
