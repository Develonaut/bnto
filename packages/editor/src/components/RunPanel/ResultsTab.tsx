"use client";

import { Text } from "@bnto/ui";
import { useEditor } from "../../context";
import { ExecutionBanner } from "./ExecutionBanner";
import { ResultsList } from "./ResultsList";

/**
 * ResultsTab — consumes execution state via domain hook.
 *
 * A persistent StatusBanner stays mounted from execution start.
 * Props update across phases — no layout shift.
 */
function ResultsTab() {
  const editor = useEditor();
  const { phase } = editor.execution.useExecution();

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
