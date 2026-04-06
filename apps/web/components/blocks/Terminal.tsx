"use client";

import { useCallback, useEffect, useState } from "react";

import { cn } from "@bnto/ui";

import { TerminalLineRenderer } from "./TerminalLineRenderer";

/* ── Types ─────────────────────────────────────────────────────── */

export interface TerminalLine {
  /** Text to display. */
  text: string;
  /** Delay (ms) before this line appears after the previous one finishes. */
  delay?: number;
  /** If true, text is typed character by character. Otherwise it fades in. */
  typing?: boolean;
  /** Typing speed in ms per character. Default: 40. */
  speed?: number;
  /** Optional className for styling (e.g. text-muted-foreground). */
  className?: string;
}

interface TerminalProps {
  /** Lines to render sequentially. */
  lines: TerminalLine[];
  /** Additional className for the outer shell. */
  className?: string;
}

/* ── Terminal ──────────────────────────────────────────────────── */

export function Terminal({ lines, className }: TerminalProps) {
  const [visibleCount, setVisibleCount] = useState(0);

  const handleLineComplete = useCallback(
    (index: number) => {
      const nextLine = lines[index + 1];
      if (!nextLine) return;
      const delay = nextLine.delay ?? 300;
      setTimeout(() => setVisibleCount(index + 2), delay);
    },
    [lines],
  );

  /* Start the first line after initial delay. */
  useEffect(() => {
    if (lines.length > 0) {
      const delay = lines[0].delay ?? 600;
      const timer = setTimeout(() => setVisibleCount(1), delay);
      return () => clearTimeout(timer);
    }
  }, [lines]);

  return (
    <div className={cn("w-full overflow-hidden", className)}>
      {/* macOS title bar */}
      <div className="border-border flex items-center gap-2 border-b px-4 py-3">
        <span className="size-2.5 rounded-full bg-red-400" />
        <span className="size-2.5 rounded-full bg-yellow-400" />
        <span className="size-2.5 rounded-full bg-green-400" />
      </div>

      {/* Terminal body — min-h reserves space so content below doesn't shift */}
      <pre className="min-h-[14rem] overflow-auto p-4">
        <code className="grid gap-y-1 font-mono text-sm leading-relaxed">
          {lines.slice(0, visibleCount).map((line, i) => (
            <TerminalLineRenderer
              key={i}
              line={line}
              index={i}
              onLineComplete={handleLineComplete}
            />
          ))}
        </code>
      </pre>
    </div>
  );
}
