"use client";

import { useCallback } from "react";

import { FadeLine } from "./FadeLine";
import type { TerminalLine } from "./Terminal";
import { TypingLine } from "./TypingLine";

/** Renders a single terminal line with a stable onComplete callback. */
export function TerminalLineRenderer({
  line,
  index,
  onLineComplete,
}: {
  line: TerminalLine;
  index: number;
  onLineComplete: (index: number) => void;
}) {
  const handleComplete = useCallback(() => onLineComplete(index), [onLineComplete, index]);

  if (line.typing) {
    return (
      <TypingLine
        text={line.text}
        speed={line.speed}
        className={line.className}
        onComplete={handleComplete}
      />
    );
  }

  return <FadeLine text={line.text} className={line.className} onComplete={handleComplete} />;
}
