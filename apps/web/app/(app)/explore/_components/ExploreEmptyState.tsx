/**
 * Empty state shown when no recipes match the search/filter.
 */

import { Text } from "@bnto/ui";

export function ExploreEmptyState() {
  return (
    <div className="flex flex-col items-center gap-3 py-20 text-center">
      <Text size="lg" color="muted">
        No recipes match your search.
      </Text>
      <Text size="sm" color="muted">
        Try a different query or clear the filters.
      </Text>
    </div>
  );
}
