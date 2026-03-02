"use client";

import { core } from "@bnto/core";

import { useDelayedLoading } from "../_hooks/useDelayedLoading";
import { Animate } from "@/components/ui/Animate";
import { EmptyState } from "@/components/ui/EmptyState";
import { Grid } from "@/components/ui/Grid";
import { RecipeCard } from "@/components/blocks/RecipeCard";
import { FolderOpenIcon } from "@/components/ui/icons";

/**
 * Saved workflows grid — self-fetching.
 * Shows RecipeCards for saved workflows, or an EmptyState when empty.
 *
 * The skeleton shows 3 cards matching the grid layout so the tab panel
 * height stays stable through loading → loaded transitions.
 */
export function WorkflowGrid() {
  const { data: workflows, isLoading } = core.workflows.useWorkflows();
  const showSkeleton = useDelayedLoading(isLoading);

  if (showSkeleton) {
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
  if (isLoading) return null;

  if (!workflows || workflows.length === 0) {
    return (
      <div className="min-h-[240px]">
        <EmptyState size="md">
          <EmptyState.Icon>
            <FolderOpenIcon />
          </EmptyState.Icon>
          <EmptyState.Title>No saved workflows yet</EmptyState.Title>
          <EmptyState.Description>
            Run a recipe and save it to see it here.
          </EmptyState.Description>
        </EmptyState>
      </div>
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
