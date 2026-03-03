"use client";

import { useState } from "react";
import type { RecipeListItem } from "@bnto/core";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Grid } from "@/components/ui/Grid";
import { Heading } from "@/components/ui/Heading";
import { Row } from "@/components/ui/Row";
import { Skeleton } from "@/components/ui/Skeleton";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { RecipeCard } from "@/components/blocks/RecipeCard";
import { SavedRecipeCard } from "@/components/blocks/SavedRecipeCard";
import { getBntoIcon } from "@/lib/bntoIcons";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

/* ── Mock recipe data ──────────────────────────────────────── */

const NOW = Date.now();
const HOUR = 3_600_000;
const DAY = 86_400_000;

const MOCK_RECIPES: Array<{
  recipe: RecipeListItem;
  lastStatus?: "pending" | "running" | "completed" | "failed";
}> = [
  {
    recipe: { id: "1", name: "Compress & Resize", nodeCount: 3, updatedAt: NOW - 2 * HOUR },
    lastStatus: "completed",
  },
  {
    recipe: { id: "2", name: "Clean CSV Pipeline", nodeCount: 5, updatedAt: NOW - 3 * DAY },
    lastStatus: "failed",
  },
  {
    recipe: { id: "3", name: "Batch Rename", nodeCount: 2, updatedAt: NOW - 15 * DAY },
    lastStatus: "running",
  },
  {
    recipe: { id: "4", name: "Image Format Converter", nodeCount: 4, updatedAt: NOW - 45 * DAY },
  },
];

/* ── Showcase ────────────────────────────────────────────────── */

export function RecipeCardShowcase() {
  const [loading, setLoading] = useState(false);
  const registrySlice = BNTO_REGISTRY.slice(0, 3);

  return (
    <Stack gap="lg">
      <Row gap="sm" className="items-center">
        <Button
          variant={loading ? "secondary" : "outline"}
          onClick={() => setLoading((l) => !l)}
        >
          {loading ? "Load Content" : "Show Loading"}
        </Button>
        <Text size="sm" color="muted">
          {loading
            ? "Skeletons are grounded — cards spring up when content arrives."
            : "Cards are loaded with real content."}
        </Text>
      </Row>

      {/* SavedRecipeCard — domain wrapper pattern */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">SavedRecipeCard</Heading>
          <Text size="sm" color="muted">
            Domain wrapper that composes RecipeCard sub-components with saved recipe data.
          </Text>
        </div>
        <Animate.Stagger asChild>
          <Grid cols={4} gap="md" animated>
            {MOCK_RECIPES.map((item, i) => (
              <Grid.Item key={item.recipe.id}>
                <Animate.ScaleIn index={i} from={0.85} easing="spring-bouncy" className="h-full">
                  <SavedRecipeCard
                    recipe={item.recipe}
                    lastStatus={item.lastStatus}
                    loading={loading}
                  />
                </Animate.ScaleIn>
              </Grid.Item>
            ))}
          </Grid>
        </Animate.Stagger>
      </Stack>

      {/* RecipeCard — direct composition pattern */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">RecipeCard</Heading>
          <Text size="sm" color="muted">
            Direct composition with sub-components. Caller controls layout and content.
          </Text>
        </div>
        <Animate.Stagger asChild>
          <Grid cols={4} gap="md" animated>
            {registrySlice.map((entry, i) => (
              <Grid.Item key={entry.slug}>
                <Animate.ScaleIn index={i} from={0.85} easing="spring-bouncy" className="h-full">
                  <RecipeCard href={`/${entry.slug}`} loading={loading}>
                    {loading ? (
                      <>
                        <RecipeCard.Header>
                          <Skeleton className="size-10 rounded-lg" />
                          <Skeleton className="h-3 w-8" />
                        </RecipeCard.Header>
                        <RecipeCard.Content>
                          <Skeleton className="h-5 w-3/4" />
                          <Row wrap className="gap-1.5 pt-1">
                            <Skeleton className="h-5 w-12 rounded-full" />
                            <Skeleton className="h-5 w-28 rounded-full" />
                            <Skeleton className="h-5 w-20 rounded-full" />
                          </Row>
                        </RecipeCard.Content>
                      </>
                    ) : (
                      <>
                        <RecipeCard.Header>
                          <RecipeCard.Icon icon={getBntoIcon(entry.slug)} />
                          <RecipeCard.Category>{entry.features[0]}</RecipeCard.Category>
                        </RecipeCard.Header>
                        <RecipeCard.Content>
                          <RecipeCard.Title>{entry.h1.replace(/ Online Free$/, "")}</RecipeCard.Title>
                          <RecipeCard.Tags tags={entry.features} limit={3} />
                        </RecipeCard.Content>
                      </>
                    )}
                  </RecipeCard>
                </Animate.ScaleIn>
              </Grid.Item>
            ))}
          </Grid>
        </Animate.Stagger>
      </Stack>
    </Stack>
  );
}
