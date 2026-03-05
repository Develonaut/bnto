"use client";

import { useState } from "react";
import type { RecipeListItem } from "@bnto/core";

import {
  Stagger,
  ScaleIn,
  Button,
  Grid,
  GridItem,
  Heading,
  Row,
  Skeleton,
  Stack,
  Text,
} from "@bnto/ui";
import {
  RecipeCard,
  RecipeCardHeader,
  RecipeCardContent,
  RecipeCardIcon,
  RecipeCardCategory,
  RecipeCardTitle,
  RecipeCardTags,
} from "@/components/blocks/RecipeCard";
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
        <Button variant={loading ? "secondary" : "outline"} onClick={() => setLoading((l) => !l)}>
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
          <Heading level={3} size="xs">
            SavedRecipeCard
          </Heading>
          <Text size="sm" color="muted">
            Domain wrapper that composes RecipeCard sub-components with saved recipe data.
          </Text>
        </div>
        <Stagger asChild>
          <Grid cols={4} gap="md" animated>
            {MOCK_RECIPES.map((item, i) => (
              <GridItem key={item.recipe.id}>
                <ScaleIn index={i} from={0.85} easing="spring-bouncy" className="h-full">
                  <SavedRecipeCard
                    recipe={item.recipe}
                    lastStatus={item.lastStatus}
                    loading={loading}
                  />
                </ScaleIn>
              </GridItem>
            ))}
          </Grid>
        </Stagger>
      </Stack>

      {/* RecipeCard — direct composition pattern */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">
            RecipeCard
          </Heading>
          <Text size="sm" color="muted">
            Direct composition with sub-components. Caller controls layout and content.
          </Text>
        </div>
        <Stagger asChild>
          <Grid cols={4} gap="md" animated>
            {registrySlice.map((entry, i) => (
              <GridItem key={entry.slug}>
                <ScaleIn index={i} from={0.85} easing="spring-bouncy" className="h-full">
                  <RecipeCard href={`/${entry.slug}`} loading={loading}>
                    {loading ? (
                      <>
                        <RecipeCardHeader>
                          <Skeleton className="size-10 rounded-lg" />
                          <Skeleton className="h-3 w-8" />
                        </RecipeCardHeader>
                        <RecipeCardContent>
                          <Skeleton className="h-5 w-3/4" />
                          <Row wrap className="gap-1.5 pt-1">
                            <Skeleton className="h-5 w-12 rounded-full" />
                            <Skeleton className="h-5 w-28 rounded-full" />
                            <Skeleton className="h-5 w-20 rounded-full" />
                          </Row>
                        </RecipeCardContent>
                      </>
                    ) : (
                      <>
                        <RecipeCardHeader>
                          <RecipeCardIcon icon={getBntoIcon(entry.slug)} />
                          <RecipeCardCategory>{entry.features[0]}</RecipeCardCategory>
                        </RecipeCardHeader>
                        <RecipeCardContent>
                          <RecipeCardTitle>{entry.h1.replace(/ Online Free$/, "")}</RecipeCardTitle>
                          <RecipeCardTags tags={entry.features} limit={3} />
                        </RecipeCardContent>
                      </>
                    )}
                  </RecipeCard>
                </ScaleIn>
              </GridItem>
            ))}
          </Grid>
        </Stagger>
      </Stack>
    </Stack>
  );
}
