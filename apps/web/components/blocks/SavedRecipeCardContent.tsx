import type { Execution, RecipeListItem } from "@bnto/core";

import {
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardIcon,
  RecipeCardTitle,
  Row,
  Text,
  formatTimeAgo,
} from "@bnto/ui";
import { StatusBadge } from "@/components/blocks/StatusBadge";

interface SavedRecipeCardContentProps {
  recipe: RecipeListItem;
  lastStatus?: Execution["status"];
}

export function SavedRecipeCardContent({ recipe, lastStatus }: SavedRecipeCardContentProps) {
  return (
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
  );
}
