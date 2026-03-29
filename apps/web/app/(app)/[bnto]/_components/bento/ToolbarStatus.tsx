"use client";

import { Text } from "@bnto/ui";
import type { BentoFlowPhase } from "../../_hooks/useBentoRecipeFlow";

/** Phase-dependent status text for the toolbar. */
export function ToolbarStatus({ phase }: { phase: BentoFlowPhase }) {
  if (phase === "running")
    return (
      <Text size="xs" color="muted">
        Processing files...
      </Text>
    );
  if (phase === "failed")
    return (
      <Text size="xs" className="text-destructive">
        Processing failed. Try again.
      </Text>
    );
  return null;
}
