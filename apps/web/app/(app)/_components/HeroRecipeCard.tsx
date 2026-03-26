import {
  RecipeCard,
  RecipeCardContent,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardTitle,
  RecipeCardTags,
} from "@bnto/ui";

import { getBntoIcon } from "@/lib/bntoIcons";
import type { BntoEntry } from "@/lib/bntoRegistry";

export function HeroRecipeCard({ entry }: { entry: BntoEntry }) {
  return (
    <RecipeCard href={`/${entry.slug}`}>
      <RecipeCardHeader>
        <RecipeCardIcon icon={getBntoIcon(entry.slug)} />
        <RecipeCardCategory>{entry.features[0]}</RecipeCardCategory>
      </RecipeCardHeader>
      <RecipeCardContent>
        <RecipeCardTitle>{entry.h1.replace(/ Online Free$/, "")}</RecipeCardTitle>
        <RecipeCardTags tags={entry.features} limit={3} />
      </RecipeCardContent>
    </RecipeCard>
  );
}
