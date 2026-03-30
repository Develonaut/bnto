"use client";

import type { ReactNode } from "react";
import { cn, Button, LoaderIcon, PlayIcon, RotateCcwIcon } from "@bnto/ui";
import { useRecipeStepperStore, useRecipeStepperActions } from "../../_stores/recipeStepperContext";

export type RunStep = "idle" | "running" | "completed" | "failed";

interface RunButtonProps {
  variant: "primary" | "outline";
  disabled: boolean;
  label: string;
  icon: ReactNode;
}

// eslint-disable-next-line no-restricted-syntax -- co-located prop resolver for RunRecipeButton
function resolveRunButtonProps(step: RunStep, hasFiles: boolean): RunButtonProps {
  switch (step) {
    case "completed":
      return {
        variant: "primary",
        disabled: false,
        label: "Rerun",
        icon: <RotateCcwIcon className="size-4" />,
      };
    case "failed":
      return {
        variant: "outline",
        disabled: false,
        label: "Try again",
        icon: <RotateCcwIcon className="size-4" />,
      };
    case "running":
      return {
        variant: "primary",
        disabled: true,
        label: "Processing",
        icon: <LoaderIcon className="size-4 motion-safe:animate-spin" />,
      };
    default:
      return {
        variant: "primary",
        disabled: !hasFiles,
        label: hasFiles ? "Run" : "Select files to run",
        icon: <PlayIcon className="size-4" />,
      };
  }
}

/** Primary run CTA — reads step and file count from the stepper store. */
export function RunRecipeButton({ className }: { className?: string }) {
  const step = useRecipeStepperStore((s) => s.resolvedStep);
  const hasFiles = useRecipeStepperStore((s) => s.files.length > 0);
  const actions = useRecipeStepperActions();
  const props = resolveRunButtonProps(step, hasFiles);

  return (
    <Button
      variant={props.variant}
      size="icon"
      onClick={actions.run}
      disabled={props.disabled}
      className={cn(className)}
      data-testid="run-button"
      data-step={step}
      aria-label={props.label}
    >
      {props.icon}
    </Button>
  );
}
