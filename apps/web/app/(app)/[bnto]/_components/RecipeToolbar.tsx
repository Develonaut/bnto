"use client";

import type { ReactNode } from "react";
import type { RunPhase } from "./RunButton";
import { ToolbarActions } from "./ToolbarActions";
import { ToolbarBackSection } from "./ToolbarBackSection";

interface RecipeToolbarProps {
  activePhase: 2 | 3;
  resolvedPhase: RunPhase;
  fileCount: number;
  onRun: () => void;
  onDownloadAll: () => void;
  centerContent?: ReactNode;
}

/**
 * Toolbar for recipe Phases 2-3.
 *
 * Left: file count. Right: action buttons (clear, download).
 */
export function RecipeToolbar(props: RecipeToolbarProps) {
  const { activePhase, resolvedPhase, fileCount, centerContent, onRun, onDownloadAll } = props;

  return (
    <div role="toolbar" aria-label="Recipe actions" className="flex items-center gap-4">
      <ToolbarBackSection
        activePhase={activePhase}
        resolvedPhase={resolvedPhase}
        fileCount={fileCount}
        onRun={onRun}
        onDownloadAll={onDownloadAll}
      />
      {centerContent && (
        <div className="min-w-0 flex-1 border-border md:mx-4 md:border-l md:border-r md:px-4">
          {centerContent}
        </div>
      )}
    </div>
  );
}
