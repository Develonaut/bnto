import type { Execution, WorkflowListItem } from "@bnto/core";

import { RecipeCard } from "@/components/blocks/RecipeCard";
import { Skeleton } from "@/components/ui/Skeleton";

interface WorkflowCardProps {
  workflow: WorkflowListItem;
  lastStatus?: Execution["status"];
  onClick?: () => void;
  /** Grounded loading state — card springs up when loading clears. */
  loading?: boolean;
}

export function WorkflowCard({ workflow, lastStatus, onClick, loading }: WorkflowCardProps) {
  return (
    <RecipeCard onClick={onClick} loading={loading}>
      {loading ? (
        <>
          <RecipeCard.Header>
            <Skeleton className="size-10 rounded-lg" />
            <Skeleton className="h-5 w-16 rounded-full" />
          </RecipeCard.Header>
          <RecipeCard.Content>
            <Skeleton className="h-5 w-3/4" />
            <Skeleton className="h-4 w-1/2" />
          </RecipeCard.Content>
        </>
      ) : (
        <>
          <RecipeCard.Header>
            <RecipeCard.Icon />
            {lastStatus && <RecipeCard.Status status={lastStatus} />}
          </RecipeCard.Header>
          <RecipeCard.Content>
            <RecipeCard.Title>{workflow.name}</RecipeCard.Title>
            <RecipeCard.Meta nodeCount={workflow.nodeCount} updatedAt={workflow.updatedAt} />
          </RecipeCard.Content>
        </>
      )}
    </RecipeCard>
  );
}
