import type { CSSProperties } from "react";
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

import { hashDelay } from "./hashDelay";

interface ShowcaseCardProps {
  entry: BntoEntry;
  dormant?: boolean;
  /** Additional class names. Defaults to `w-80` for marquee use. Pass `w-full` for grid layouts. */
  className?: string;
}

export function ShowcaseCard({ entry, dormant, className }: ShowcaseCardProps) {
  return (
    <RecipeCard
      href={`/${entry.slug}`}
      className={className ?? "w-80"}
      elevation="lg"
      dormant={dormant}
      style={{ "--spring-delay": `${hashDelay(entry.slug)}ms` } as CSSProperties}
    >
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
