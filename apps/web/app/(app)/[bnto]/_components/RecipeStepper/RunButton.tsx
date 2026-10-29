"use client";

import type { ReactNode } from "react";
import { cn, Button, LoaderIcon, PlayIcon, RotateCcwIcon } from "@bnto/ui";
import {
  useRecipeStepperStore,
  useRecipeStepperActions,
  useRecipeStepperDefn,
} from "../../_stores/recipeStepperContext";
import { missingRequiredParams } from "../../_utils/missingRequiredParams";

export type RunStep = "idle" | "running" | "completed" | "failed";

interface RunButtonProps {
  variant: "primary" | "outline";
  disabled: boolean;
  label: string;
  icon: ReactNode;
}

// eslint-disable-next-line no-restricted-syntax -- co-located prop resolver for RunRecipeButton
function resolveRunButtonProps(
  step: RunStep,
  hasFiles: boolean,
  missing: string | null,
): RunButtonProps {
  const blocked = missing ? `Set ${missing} to run` : null;
  switch (step) {
    case "completed":
      return {
        variant: "primary",
        disabled: blocked !== null,
        label: blocked ?? "Rerun",
        icon: <RotateCcwIcon className="size-4" />,
      };
    case "failed":
      return {
        variant: "outline",
        disabled: blocked !== null,
        label: blocked ?? "Try again",
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
        disabled: !hasFiles || blocked !== null,
        label: hasFiles ? (blocked ?? "Run") : "Select files to run",
        icon: <PlayIcon className="size-4" />,
      };
  }
}

/**
 * Primary run CTA — reads step, file count and the first required param
 * with no value from the stepper store. Disabled, naming that param, until
 * it has one (ISS-73).
 */
export function RunRecipeButton({ className }: { className?: string }) {
  const step = useRecipeStepperStore((s) => s.resolvedStep);
  const hasFiles = useRecipeStepperStore((s) => s.files.length > 0);
  const { definition } = useRecipeStepperDefn();
  const missing = useRecipeStepperStore((s) =>
    definition ? (missingRequiredParams(definition, s.config)[0]?.label ?? null) : null,
  );
  const actions = useRecipeStepperActions();
  const props = resolveRunButtonProps(step, hasFiles, missing);

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
