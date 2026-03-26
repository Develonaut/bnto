"use client";

import { LoaderIcon } from "@bnto/ui";

/** Loading skeleton shown while the execution document is being fetched. */
export function ExecutionProgressLoading() {
  return (
    <div
      className="rounded-lg border border-border bg-card p-4"
      data-testid="execution-progress"
      data-status="loading"
    >
      <div className="flex items-center gap-3">
        <LoaderIcon className="size-5 shrink-0 text-primary motion-safe:animate-spin" />
        <p className="text-sm text-muted-foreground">Starting execution&hellip;</p>
      </div>
    </div>
  );
}
