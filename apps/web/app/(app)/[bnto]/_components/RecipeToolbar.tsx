"use client";

import type { ReactNode } from "react";
import type { RunPhase } from "./RunButton";
import { ToolbarActions } from "./ToolbarActions";
import { ToolbarBackSection } from "./ToolbarBackSection";

interface RecipeToolbarProps {
  activePhase: 2 | 3;
  resolvedPhase: RunPhase;
  isProcessing: boolean;
  fileCount: number;
  onBack: () => void;
  onRun: () => void;
  onDownloadAll: () => void;
  centerContent?: ReactNode;
}

/**
 * Responsive toolbar for recipe Phases 2-3.
 *
 * Left: back + file count. Center: config or progress slot.
 * Right: action buttons. Stacks vertically on mobile, inline on desktop.
 */
export function RecipeToolbar(props: RecipeToolbarProps) {
  const { activePhase, resolvedPhase, fileCount, centerContent, onRun, onDownloadAll } = props;

  return (
    <div
      role="toolbar"
      aria-label="Recipe actions"
      className="flex min-h-[4.5rem] flex-col gap-3 md:flex-row md:items-center md:gap-4"
    >
      <ToolbarBackSection {...props} />
      {centerContent && (
        <div className="min-w-0 flex-1 border-border md:mx-4 md:border-l md:border-r md:px-4">
          {centerContent}
        </div>
      )}
      <ToolbarActions
        activePhase={activePhase}
        resolvedPhase={resolvedPhase}
        fileCount={fileCount}
        onRun={onRun}
        onDownloadAll={onDownloadAll}
        className="ml-auto hidden shrink-0 md:flex"
      />
    </div>
  );
}
