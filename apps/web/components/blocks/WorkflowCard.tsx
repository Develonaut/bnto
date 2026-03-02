import type { Execution, WorkflowListItem } from "@bnto/core";

import { RecipeCard } from "@/components/blocks/RecipeCard";

interface WorkflowCardProps {
  workflow: WorkflowListItem;
  lastStatus?: Execution["status"];
  onClick?: () => void;
}

export function WorkflowCard({ workflow, lastStatus, onClick }: WorkflowCardProps) {
  return (
    <RecipeCard onClick={onClick}>
      <RecipeCard.Header>
        <RecipeCard.Icon />
        {lastStatus && <RecipeCard.Status status={lastStatus} />}
      </RecipeCard.Header>
      <RecipeCard.Content>
        <RecipeCard.Title>{workflow.name}</RecipeCard.Title>
        <RecipeCard.Meta nodeCount={workflow.nodeCount} updatedAt={workflow.updatedAt} />
      </RecipeCard.Content>
    </RecipeCard>
  );
}
