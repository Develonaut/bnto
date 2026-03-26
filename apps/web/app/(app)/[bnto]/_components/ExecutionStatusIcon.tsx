"use client";

import { CheckCircle2Icon, LoaderIcon, XCircleIcon } from "@bnto/ui";
import type { Execution } from "@bnto/core";

/** Status icon for the execution header row. */
export function ExecutionStatusIcon({ status }: { status: Execution["status"] }) {
  switch (status) {
    case "pending":
    case "running":
      return <LoaderIcon className="size-5 shrink-0 text-primary motion-safe:animate-spin" />;
    case "completed":
      return <CheckCircle2Icon className="size-5 shrink-0 text-success" />;
    case "failed":
      return <XCircleIcon className="size-5 shrink-0 text-destructive" />;
  }
}

/** Human-readable label for the execution status. */
export function getStatusLabel(status: Execution["status"]): string {
  switch (status) {
    case "pending":
      return "Queued...";
    case "running":
      return "Processing...";
    case "completed":
      return "Completed";
    case "failed":
      return "Failed";
  }
}
