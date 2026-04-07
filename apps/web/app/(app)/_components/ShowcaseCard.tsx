import {
  RecipeCard,
  RecipeCardCategory,
  RecipeCardContent,
  RecipeCardDescription,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardTags,
  RecipeCardTitle,
} from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import type { BntoEntry } from "@/lib/bntoRegistry";

export function ShowcaseCard({ entry }: { entry: BntoEntry }) {
  return (
    <RecipeCard href={`/${entry.slug}`} className="w-80">
      <RecipeCardHeader>
        <RecipeCardIcon icon={getBntoIcon(entry.slug)} />
        <RecipeCardCategory>{entry.features[0]}</RecipeCardCategory>
      </RecipeCardHeader>
      <RecipeCardContent>
        <RecipeCardTitle>{entry.h1.replace(/ Online Free$/, "")}</RecipeCardTitle>
        <RecipeCardDescription>
          <span className="line-clamp-2">{entry.description}</span>
        </RecipeCardDescription>
        <RecipeCardTags tags={entry.features} limit={3} />
      </RecipeCardContent>
    </RecipeCard>
  );
}
