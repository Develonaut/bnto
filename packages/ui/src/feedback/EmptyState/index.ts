import { EmptyStateRoot } from "./EmptyStateRoot";
import { EmptyStateIcon } from "./EmptyStateIcon";
import { EmptyStateTitle } from "./EmptyStateTitle";
import { EmptyStateDescription } from "./EmptyStateDescription";
import { EmptyStateAction } from "./EmptyStateAction";
import { EmptyStateSkeleton } from "./EmptyStateSkeleton";

export const EmptyState = Object.assign(EmptyStateRoot, {
  Root: EmptyStateRoot,
  Icon: EmptyStateIcon,
  Title: EmptyStateTitle,
  Description: EmptyStateDescription,
  Action: EmptyStateAction,
  Skeleton: EmptyStateSkeleton,
});
