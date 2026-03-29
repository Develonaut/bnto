"use client";

import { cn } from "@bnto/ui";

const STATUS_CLASSES: Record<string, string> = {
  running: "bg-primary motion-safe:animate-pulse",
  completed: "bg-secondary",
  failed: "bg-destructive",
};

/** Colored dot indicating node execution status. */
export function NodeStatusIndicator({ status }: { status: string }) {
  return <span className={cn("size-2 rounded-full", STATUS_CLASSES[status] ?? "bg-border")} />;
}
