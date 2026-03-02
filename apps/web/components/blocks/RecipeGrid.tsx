"use client";

import { Animate } from "@/components/ui/Animate";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { RecipeCard } from "@/components/blocks/RecipeCard";
import { getBntoIcon } from "@/lib/bntoIcons";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

/* ── Recipe grid ─────────────────────────────────────────────── */

export function RecipeGrid() {
  return (
    <Stack className="gap-3">
      <Animate.BouncyStagger className="grid grid-cols-2 gap-4" from={0.85}>
        {BNTO_REGISTRY.map((entry) => (
          <RecipeCard key={entry.slug} href={`/${entry.slug}`}>
            <RecipeCard.Header>
              <RecipeCard.Icon icon={getBntoIcon(entry.slug)} />
              <RecipeCard.Category>{entry.features[0]}</RecipeCard.Category>
            </RecipeCard.Header>
            <RecipeCard.Content>
              <RecipeCard.Title>{entry.h1.replace(/ Online Free$/, "")}</RecipeCard.Title>
              <RecipeCard.Tags tags={entry.features} limit={3} />
            </RecipeCard.Content>
          </RecipeCard>
        ))}
      </Animate.BouncyStagger>
      <Text size="xs" color="muted" className="text-center">
        Pick a tool to get started. No signup needed.
      </Text>
    </Stack>
  );
}
