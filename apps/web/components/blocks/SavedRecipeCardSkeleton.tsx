import { RecipeCardHeader, RecipeCardContent, Skeleton } from "@bnto/ui";

export function SavedRecipeCardSkeleton() {
  return (
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
  );
}
