"use client";

import { ReactFlowProvider } from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import { cn } from "@bnto/ui";
import type { CanvasProps } from "./canvasTypes";
import { CanvasInner } from "./CanvasInner";

/* ── Public canvas wrapper ──────────────────────────────────── */

export function Canvas(props: CanvasProps) {
  const { height, standalone = false, className, children, ...innerProps } = props;

  const inner = <CanvasInner {...innerProps}>{children}</CanvasInner>;
  const wrappedInner = standalone ? inner : <ReactFlowProvider>{inner}</ReactFlowProvider>;

  return (
    <div
      className={cn(
        "w-full overflow-hidden rounded-xl border border-border bg-background",
        className,
      )}
      style={height ? { height } : undefined}
    >
      {wrappedInner}
    </div>
  );
}
