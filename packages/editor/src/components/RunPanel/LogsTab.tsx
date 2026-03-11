"use client";

import { useEffect, useMemo, useRef } from "react";
import { Text, CopyButton, Divider } from "@bnto/ui";
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

  const logsText = useMemo(
    () =>
      logs
        .map((entry) => {
          const { time, icon, message } = formatLogEntry(entry);
          return `${time} ${icon} ${message}`;
        })
        .join("\n"),
    [logs],
  );

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
    <div className="flex h-full flex-col">
      <div className="flex shrink-0 justify-end px-2 pt-1">
        <CopyButton value={logsText} label="Copy logs" />
      </div>
      <div ref={scrollRef} className="min-h-0 flex-1 overflow-y-auto p-2 pt-0">
        <pre className="whitespace-pre-wrap break-words font-mono text-xs leading-relaxed text-foreground">
          {logs.map((entry, i) => (
            <LogLine key={i} entry={entry} isLast={i === logs.length - 1} />
          ))}
        </pre>
      </div>
    </div>
  );
}

function LogLine({ entry, isLast }: { entry: RunLogEntry; isLast: boolean }) {
  const { icon, message } = formatLogEntry(entry);
  return (
    <>
      <div className="flex gap-2 py-1.5 hover:bg-muted/50">
        <span className="shrink-0">{icon}</span>
        <span>{message}</span>
      </div>
      {!isLast && <Divider />}
    </>
  );
}

export { LogsTab };
