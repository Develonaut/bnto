import type { Execution, RecipeListItem } from "@bnto/core";

import {
  RecipeCard,
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardIcon,
  RecipeCardStatus,
  RecipeCardTitle,
  RecipeCardMeta,
} from "@/components/blocks/RecipeCard";
import { Skeleton } from "@bnto/ui";

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
          <RecipeCardHeader>
            <Skeleton className="size-10 rounded-lg" />
            <Skeleton className="h-5 w-16 rounded-full" />
          </RecipeCardHeader>
          <RecipeCardContent>
            <Skeleton className="h-5 w-3/4" />
            <Skeleton className="h-4 w-1/2" />
          </RecipeCardContent>
        </>
      ) : (
        <>
          <RecipeCardHeader>
            <RecipeCardIcon />
            {lastStatus && <RecipeCardStatus status={lastStatus} />}
          </RecipeCardHeader>
          <RecipeCardContent>
            <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
            <RecipeCardMeta nodeCount={recipe.nodeCount} updatedAt={recipe.updatedAt} />
          </RecipeCardContent>
        </>
      )}
    </RecipeCard>
  );
}
