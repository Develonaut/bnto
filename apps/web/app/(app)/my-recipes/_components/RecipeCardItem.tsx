"use client";

import {
  CardActions,
  GridItem,
  RecipeCard,
  RecipeCardContent,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardTags,
  RecipeCardTitle,
  ScaleIn,
} from "@bnto/ui";
import type { LucideIcon } from "@bnto/ui";
import { editorUrl } from "@/lib/routes";
import { RecipeCardMenu } from "./RecipeCardMenu";
import { RecipeCardMeta } from "./RecipeCardMeta";

interface RecipeCardItemProps {
  recipe: {
    id: string;
    name: string;
    nodeCount: number;
    nodeTypes: string[];
    updatedAt: number;
    syncedAt?: number | null;
  };
  index: number;
  categoryIcon?: LucideIcon;
}

function RecipeCardBody({
  recipe,
  categoryIcon,
}: Pick<RecipeCardItemProps, "recipe" | "categoryIcon">) {
  return (
    <RecipeCard href={editorUrl(recipe.id)}>
      <RecipeCardHeader>
        <RecipeCardIcon icon={categoryIcon} />
        <CardActions>
          <RecipeCardMenu recipeId={recipe.id} recipeName={recipe.name} />
        </CardActions>
      </RecipeCardHeader>
      <RecipeCardContent>
        <RecipeCardTitle>{recipe.name}</RecipeCardTitle>
        <RecipeCardMeta
          nodeCount={recipe.nodeCount}
          updatedAt={recipe.updatedAt}
          syncedAt={recipe.syncedAt}
        />
        {recipe.nodeTypes.length > 0 && <RecipeCardTags tags={recipe.nodeTypes} limit={3} />}
      </RecipeCardContent>
    </RecipeCard>
  );
}

/** Individual recipe card in the grid. */
export function RecipeCardItem({ recipe, index, categoryIcon }: RecipeCardItemProps) {
  return (
    <GridItem>
      <ScaleIn
        index={index}
        from={0.85}
        easing="spring-bouncy"
        className="h-full"
        data-testid="recipe-card"
      >
        <RecipeCardBody recipe={recipe} categoryIcon={categoryIcon} />
      </ScaleIn>
    </GridItem>
  );
}
