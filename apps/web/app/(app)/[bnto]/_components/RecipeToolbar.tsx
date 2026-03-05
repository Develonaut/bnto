"use client";

import type { ReactNode } from "react";
import { Button, FileUpload, Row } from "@bnto/ui";
import {
  ArrowLeftIcon,
  DownloadIcon,
  TrashIcon,
} from "@bnto/ui";
import { RunButton } from "./RunButton";
import type { RunPhase } from "./RunButton";

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
 * Responsive toolbar for recipe Phases 2–3.
 *
 * Left: back + file count. Center: config or progress slot.
 * Right: action buttons. Stacks vertically on mobile, inline on desktop.
 */
export function RecipeToolbar({
  activePhase,
  resolvedPhase,
  isProcessing,
  fileCount,
  onBack,
  onRun,
  onDownloadAll,
  centerContent,
}: RecipeToolbarProps) {
  return (
    <div
      role="toolbar"
      aria-label="Recipe actions"
      className="flex min-h-10 flex-col gap-3 md:flex-row md:items-center md:gap-4"
    >
      {/* Left: back button + file count + mobile actions */}
      <Row gap="xs">
        <Button
          variant="ghost"
          size="icon"
          elevation="sm"
          disabled={isProcessing}
          onClick={onBack}
          aria-label={
            activePhase === 3 ? "Back to configure" : "Back to file selection"
          }
        >
          <ArrowLeftIcon className="size-4" />
        </Button>
        <p className="shrink-0 text-sm font-medium text-foreground">
          {fileCount} {fileCount === 1 ? "file" : "files"} selected
        </p>
        <Row gap="xs" className="ml-auto shrink-0 md:hidden">
          {activePhase === 3 && (
            <Button
              variant="outline"
              size="icon"
              elevation="sm"
              disabled={resolvedPhase !== "completed"}
              onClick={onDownloadAll}
              aria-label="Download all"
            >
              <DownloadIcon className="size-4" />
            </Button>
          )}
          {activePhase === 2 && (
            <FileUpload.Clear asChild>
              <Button
                variant="outline"
                size="icon"
                elevation="md"
                aria-label="Clear all files"
              >
                <TrashIcon className="size-4" />
              </Button>
            </FileUpload.Clear>
          )}
          <RunButton
            phase={resolvedPhase}
            hasFiles={fileCount > 0}
            onRun={onRun}
          />
        </Row>
      </Row>

      {/* Center: config (Phase 2) or progress (Phase 3) */}
      {centerContent && (
        <div className="min-w-0 flex-1 border-border md:mx-4 md:border-l md:border-r md:px-4">
          {centerContent}
        </div>
      )}

      {/* Right: desktop actions (hidden on mobile) */}
      <Row gap="xs" className="ml-auto hidden shrink-0 md:flex">
        {activePhase === 3 && (
          <Button
            variant="outline"
            size="icon"
            elevation="sm"
            disabled={resolvedPhase !== "completed"}
            onClick={onDownloadAll}
            aria-label="Download all"
          >
            <DownloadIcon className="size-4" />
          </Button>
        )}
        {activePhase === 2 && (
          <FileUpload.Clear asChild>
            <Button
              variant="outline"
              size="icon"
              elevation="md"
              aria-label="Clear all files"
            >
              <TrashIcon className="size-4" />
            </Button>
          </FileUpload.Clear>
        )}
        <RunButton
          phase={resolvedPhase}
          hasFiles={fileCount > 0}
          onRun={onRun}
        />
      </Row>
    </div>
  );
}
