import type { WorkflowListItem } from "@bnto/core";

import { Animate } from "@/components/ui/Animate";
import { Grid } from "@/components/ui/Grid";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { RecipeCard } from "@/components/blocks/RecipeCard";
import { WorkflowCard } from "@/components/blocks/WorkflowCard";
import { getBntoIcon } from "@/lib/bntoIcons";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

/* ── Mock workflow data ──────────────────────────────────────── */

const NOW = Date.now();
const HOUR = 3_600_000;
const DAY = 86_400_000;

const MOCK_WORKFLOWS: Array<{
  workflow: WorkflowListItem;
  lastStatus?: "pending" | "running" | "completed" | "failed";
}> = [
  {
    workflow: { id: "1", name: "Compress & Resize", nodeCount: 3, updatedAt: NOW - 2 * HOUR },
    lastStatus: "completed",
  },
  {
    workflow: { id: "2", name: "Clean CSV Pipeline", nodeCount: 5, updatedAt: NOW - 3 * DAY },
    lastStatus: "failed",
  },
  {
    workflow: { id: "3", name: "Batch Rename", nodeCount: 2, updatedAt: NOW - 15 * DAY },
    lastStatus: "running",
  },
  {
    workflow: { id: "4", name: "Image Format Converter", nodeCount: 4, updatedAt: NOW - 45 * DAY },
  },
];

/* ── Showcase ────────────────────────────────────────────────── */

export function RecipeCardShowcase() {
  const registrySlice = BNTO_REGISTRY.slice(0, 3);

  return (
    <Stack gap="lg">
      {/* WorkflowCard — domain wrapper pattern */}
      <Stack gap="sm">
        <div>
          <Heading level={3} size="xs">WorkflowCard</Heading>
          <Text size="sm" color="muted">
            Domain wrapper that composes RecipeCard sub-components with workflow data.
          </Text>
        </div>
        <Animate.Stagger asChild>
          <Grid cols={4} gap="md" animated>
            {MOCK_WORKFLOWS.map((item, i) => (
              <Grid.Item key={item.workflow.id}>
                <Animate.ScaleIn index={i} from={0.85} easing="spring-bouncy" className="h-full">
                  <WorkflowCard workflow={item.workflow} lastStatus={item.lastStatus} />
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
                  <RecipeCard href={`/${entry.slug}`}>
                    <RecipeCard.Header>
                      <RecipeCard.Icon icon={getBntoIcon(entry.slug)} />
                      <RecipeCard.Category>{entry.features[0]}</RecipeCard.Category>
                    </RecipeCard.Header>
                    <RecipeCard.Content>
                      <RecipeCard.Title>{entry.h1.replace(/ Online Free$/, "")}</RecipeCard.Title>
                      <RecipeCard.Tags tags={entry.features} limit={3} />
                    </RecipeCard.Content>
                  </RecipeCard>
                </Animate.ScaleIn>
              </Grid.Item>
            ))}
            <Grid.Item>
              <RecipeCard.Skeleton />
            </Grid.Item>
          </Grid>
        </Animate.Stagger>
      </Stack>
    </Stack>
  );
}
