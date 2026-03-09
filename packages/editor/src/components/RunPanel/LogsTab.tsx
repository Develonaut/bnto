"use client";

import { useEffect, useRef } from "react";
import { Text } from "@bnto/ui";
import type { RunLogEntry } from "../../store/types";
import { formatLogEntry } from "./formatLogEntry";
import { useEditorStore } from "../../hooks/useEditorStore";

/**
 * LogsTab — consumes logs directly from the editor store.
 *
 * Auto-scrolls to the bottom as new entries arrive.
 * Each entry is formatted as a timestamped line with event details.
 */
function LogsTab() {
  const logs = useEditorStore((s) => s.executionLogs);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight });
  }, [logs.length]);

  if (logs.length === 0) {
    return (
      <div className="flex h-full items-center justify-center p-4">
        <Text size="xs" color="muted">
          Run a recipe to see execution logs.
        </Text>
      </div>
    );
  }

  return (
    <div ref={scrollRef} className="h-full overflow-y-auto p-2">
      <pre className="font-mono text-xs leading-relaxed text-foreground">
        {logs.map((entry, i) => (
          <LogLine key={i} entry={entry} />
        ))}
      </pre>
    </div>
  );
}

function LogLine({ entry }: { entry: RunLogEntry }) {
  const { time, icon, message } = formatLogEntry(entry);
  return (
    <div className="flex gap-2 py-px hover:bg-muted/50">
      <span className="shrink-0 text-muted-foreground">{time}</span>
      <span className="shrink-0">{icon}</span>
      <span>{message}</span>
    </div>
  );
}

export { LogsTab };
