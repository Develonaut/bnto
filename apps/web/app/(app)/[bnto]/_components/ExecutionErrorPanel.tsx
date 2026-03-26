"use client";

import { friendlyExecutionError } from "./friendlyExecutionError";

interface ExecutionErrorPanelProps {
  error: string;
}

/** Error details shown when an execution fails. */
export function ExecutionErrorPanel({ error }: ExecutionErrorPanelProps) {
  const friendly = friendlyExecutionError(error);

  return (
    <div
      className="rounded-md border border-destructive/30 bg-destructive/5 p-3"
      data-testid="execution-error"
    >
      <p className="text-sm text-destructive">{friendly}</p>
      {error !== friendly && <p className="mt-1 text-xs text-muted-foreground">{error}</p>}
    </div>
  );
}
