"use client";

import { core } from "@bnto/core";

import { Animate } from "@/components/ui/Animate";
import { EmptyState } from "@/components/ui/EmptyState";
import { Grid } from "@/components/ui/Grid";
import { RecipeCard } from "@/components/blocks/RecipeCard";
import { FolderOpenIcon } from "@/components/ui/icons";

/**
 * Saved workflows grid — self-fetching.
 * Shows RecipeCards for saved workflows, or an EmptyState when empty.
 */
export function WorkflowGrid() {
  const { data: workflows, isLoading } = core.workflows.useWorkflows();

  if (isLoading) {
    return (
      <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md">
        {Array.from({ length: 3 }).map((_, i) => (
          <Grid.Item key={i}>
            <RecipeCard.Skeleton />
          </Grid.Item>
        ))}
      </Grid>
    );
  }

  if (!workflows || workflows.length === 0) {
    return (
      <EmptyState size="md">
        <EmptyState.Icon>
          <FolderOpenIcon />
        </EmptyState.Icon>
        <EmptyState.Title>No saved workflows yet</EmptyState.Title>
        <EmptyState.Description>
          Run a recipe and save it to see it here. Your workflows are stored in
          the cloud so you can access them from any device.
        </EmptyState.Description>
      </EmptyState>
    );
  }

  return (
    <Animate.Stagger>
      <Grid cols={{ mobile: 1, tablet: 2, desktop: 3 }} gap="md">
        {workflows.map((workflow, i) => (
          <Animate.ScaleIn key={workflow.id} index={i} from={0.95}>
            <Grid.Item>
              <RecipeCard>
                <RecipeCard.Header>
                  <RecipeCard.Icon />
                </RecipeCard.Header>
                <RecipeCard.Content>
                  <RecipeCard.Title>{workflow.name}</RecipeCard.Title>
                  <RecipeCard.Meta
                    nodeCount={workflow.nodeCount}
                    updatedAt={workflow.updatedAt}
                  />
                </RecipeCard.Content>
              </RecipeCard>
            </Grid.Item>
          </Animate.ScaleIn>
        ))}
      </Grid>
    </Animate.Stagger>
  );
}
