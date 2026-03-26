"use client";

import { ClockIcon } from "@bnto/ui";
import type { Execution } from "@bnto/core";
import { formatElapsed } from "../_hooks/useElapsedTime";
import { ExecutionStatusIcon, getStatusLabel } from "./ExecutionStatusIcon";

interface ExecutionProgressHeaderProps {
  status: Execution["status"];
  elapsed: number;
}

/** Header row with status icon, label, and elapsed time. */
export function ExecutionProgressHeader({ status, elapsed }: ExecutionProgressHeaderProps) {
  return (
    <div className="flex items-center justify-between">
      <div className="flex items-center gap-2">
        <ExecutionStatusIcon status={status} />
        <p className="text-sm font-medium text-foreground">{getStatusLabel(status)}</p>
      </div>
      <div className="flex items-center gap-1.5 text-xs text-muted-foreground">
        <ClockIcon className="size-3.5" />
        <span>{formatElapsed(elapsed)}</span>
      </div>
    </div>
  );
}
