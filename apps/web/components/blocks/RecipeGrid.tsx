"use client";

import { BouncyStagger, Stack, Text } from "@bnto/ui";
import {
  RecipeCard,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardContent,
  RecipeCardTitle,
  RecipeCardTags,
} from "@/components/blocks/RecipeCard";
import { getBntoIcon } from "@/lib/bntoIcons";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

/* ── Recipe grid ─────────────────────────────────────────────── */

export function RecipeGrid() {
  return (
    <Stack className="gap-3">
      <BouncyStagger className="grid grid-cols-2 gap-4" from={0.85}>
        {BNTO_REGISTRY.map((entry) => (
          <RecipeCard key={entry.slug} href={`/${entry.slug}`}>
            <RecipeCardHeader>
              <RecipeCardIcon icon={getBntoIcon(entry.slug)} />
              <RecipeCardCategory>{entry.features[0]}</RecipeCardCategory>
            </RecipeCardHeader>
            <RecipeCardContent>
              <RecipeCardTitle>{entry.h1.replace(/ Online Free$/, "")}</RecipeCardTitle>
              <RecipeCardTags tags={entry.features} limit={3} />
            </RecipeCardContent>
          </RecipeCard>
        ))}
      </BouncyStagger>
      <Text size="xs" color="muted" className="text-center">
        Pick a tool to get started. No signup needed.
      </Text>
    </Stack>
  );
}
