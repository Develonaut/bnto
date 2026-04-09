"use client";

import { useEffect, useRef } from "react";

import { cn, Surface } from "@bnto/ui";

import { Terminal } from "./Terminal";
import type { TerminalLine } from "./Terminal";

interface CodeEditorProps {
  /** Lines to render with typing/fade animation. */
  lines: TerminalLine[];
  /** Filename shown in the editor tab. */
  filename: string;
  /** Additional className for the outer wrapper. */
  className?: string;
}

/** Animated code editor — Terminal engine with editor chrome (filename tab). */
export function CodeEditor({ lines, filename, className }: CodeEditorProps) {
  const scrollRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);

  /* Scroll to keep the latest animated line visible as it appears. */
  useEffect(() => {
    const scroll = scrollRef.current;
    const content = contentRef.current;
    if (!scroll || !content) return;
    const observer = new MutationObserver(() => {
      const contentBottom = content.scrollHeight;
      const viewportHeight = scroll.clientHeight;
      if (contentBottom > viewportHeight) {
        scroll.scrollTop = contentBottom - viewportHeight;
      }
    });
    observer.observe(content, { childList: true, subtree: true });
    return () => observer.disconnect();
  }, []);

  return (
    <Surface elevation="lg" rounded="xl" className={cn("min-w-0", className)}>
      {/* Editor title bar — filename tab */}
      <div className="border-border flex items-center border-b px-4 py-2.5">
        <span className="bg-muted text-muted-foreground rounded-md px-3 py-1 font-mono text-xs">
          {filename}
        </span>
      </div>

      {/* Inner wrapper clips to border radius without affecting Surface 3D effect */}
      <div className="overflow-hidden rounded-b-[inherit]">
        <div ref={scrollRef} className="h-[24rem] overflow-auto">
          <div ref={contentRef}>
            <Terminal
              lines={lines}
              className="overflow-visible [&>div:first-child]:hidden [&>pre]:min-h-0 [&>pre]:overflow-visible"
            />
          </div>
        </div>
      </div>
    </Surface>
  );
}
