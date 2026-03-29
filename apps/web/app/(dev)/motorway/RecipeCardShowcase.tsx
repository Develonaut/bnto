"use client";

import { useState } from "react";

import {
  Button,
  CardActions,
  EllipsisVerticalIcon,
  Grid,
  GridItem,
  Heading,
  RecipeCard,
  RecipeCardCategory,
  RecipeCardContent,
  RecipeCardHeader,
  RecipeCardIcon,
  RecipeCardTags,
  RecipeCardTitle,
  Row,
  ScaleIn,
  Skeleton,
  Stack,
  Stagger,
  Text,
} from "@bnto/ui";
import { getBntoIcon } from "@/lib/bntoIcons";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

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

      {/* CardActions — nested controls in clickable cards */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">
            CardActions + Dormant Ellipsis
          </Heading>
          <Text size="sm" color="muted">
            Clickable card with a dormant ellipsis button inside CardActions. Click the button —
            card navigation is blocked. Click elsewhere — card navigates.
          </Text>
        </div>
        <Grid cols={4} gap="md">
          <GridItem>
            <RecipeCard href="#card-action-demo">
              <RecipeCardHeader>
                <RecipeCardIcon />
                <CardActions>
                  <Button
                    icon={<EllipsisVerticalIcon />}
                    dormant
                    aria-label="Actions"
                    onClick={() => alert("Menu clicked — card did NOT navigate")}
                  />
                </CardActions>
              </RecipeCardHeader>
              <RecipeCardContent>
                <RecipeCardTitle>Card with nested controls</RecipeCardTitle>
                <Text size="xs" color="muted">
                  Hover to wake the ellipsis. Click it safely.
                </Text>
              </RecipeCardContent>
            </RecipeCard>
          </GridItem>
        </Grid>
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
