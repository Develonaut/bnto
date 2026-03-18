import type { ComponentProps } from "react";
import type { Execution, RecipeListItem } from "@bnto/core";

import {
  RecipeCard,
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardIcon,
  RecipeCardTitle,
  Row,
  Text,
  Skeleton,
} from "@bnto/ui";
import { StatusBadge } from "@/components/blocks/StatusBadge";
import { formatTimeAgo } from "@/lib/formatTimeAgo";

interface SavedRecipeCardProps extends Pick<ComponentProps<typeof RecipeCard>, "loading"> {
  recipe: RecipeListItem;
  lastStatus?: Execution["status"];
  /** Destination link — required for saved recipe navigation. */
  href: string;
}

export function SavedRecipeCard({ recipe, lastStatus, href, loading }: SavedRecipeCardProps) {
  return (
    <RecipeCard href={href} loading={loading}>
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
            {lastStatus && <StatusBadge status={lastStatus} />}
          </RecipeCardHeader>
          <RecipeCardContent>
            <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
            <Row className="gap-2">
              <Text as="span" size="xs" color="muted">
                {recipe.nodeCount === 1 ? "1 node" : `${recipe.nodeCount} nodes`}
              </Text>
              <Text as="span" size="xs" color="muted">
                &middot;
              </Text>
              <Text as="span" size="xs" color="muted">
                {formatTimeAgo(recipe.updatedAt)}
              </Text>
            </Row>
          </RecipeCardContent>
        </>
      )}
    </RecipeCard>
  );
}
