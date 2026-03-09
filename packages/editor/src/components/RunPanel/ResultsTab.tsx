"use client";

import { Text } from "@bnto/ui";
import { useEditorStore } from "../../hooks/useEditorStore";
import { ExecutionBanner } from "./ExecutionBanner";
import { ResultsList } from "./ResultsList";

/**
 * ResultsTab — consumes execution state directly from the editor store.
 *
 * A persistent StatusBanner stays mounted from execution start.
 * Props update across phases — no layout shift.
 */
function ResultsTab() {
  const phase = useEditorStore((s) => s.executionPhase);

  if (phase === "idle") {
    return (
      <div className="flex h-full items-center justify-center p-4">
        <Text size="xs" color="muted">
          Run a recipe to see results.
        </Text>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col overflow-y-auto">
      <div className="p-2 pb-0">
        <ExecutionBanner />
      </div>
      <ResultsList />
    </div>
  );
}

export { ResultsTab };
