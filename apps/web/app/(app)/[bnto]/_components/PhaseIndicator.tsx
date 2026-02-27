import { cn } from "@/lib/cn";
import { CheckIcon } from "@/components/ui/icons";

type Phase = 1 | 2 | 3;

interface PhaseIndicatorProps {
  /** Currently active phase: 1 = Files, 2 = Configure, 3 = Results */
  activePhase: Phase;
  /** When false, phase 2 ("Configure") is hidden. Default true. */
  hasConfig?: boolean;
}

interface StepDef {
  phase: Phase;
  label: string;
}

const ALL_STEPS: StepDef[] = [
  { phase: 1, label: "Files" },
  { phase: 2, label: "Configure" },
  { phase: 3, label: "Results" },
];

/**
 * Horizontal stepper showing recipe page progress.
 *
 * Three phases: Files → Configure → Results.
 * When `hasConfig` is false, the Configure step is omitted
 * and display numbers adjust to stay sequential (1, 2 instead of 1, 3).
 * Circles + labels on desktop, circles only on mobile.
 */
export function PhaseIndicator({
  activePhase,
  hasConfig = true,
}: PhaseIndicatorProps) {
  const steps = hasConfig
    ? ALL_STEPS
    : ALL_STEPS.filter((s) => s.phase !== 2);

  return (
    <nav aria-label="Recipe progress" className="flex items-center justify-center gap-0">
      {steps.map((step, i) => {
        const isCompleted = step.phase < activePhase;
        const isActive = step.phase === activePhase;
        const displayNumber = i + 1;

        return (
          <div key={step.phase} className="flex items-center">
            {/* Connecting line before (not for the first step) */}
            {i > 0 && (
              <div
                className={cn(
                  "h-0.5 w-6 sm:w-10",
                  isCompleted || isActive ? "bg-primary" : "bg-border",
                )}
              />
            )}

            {/* Step circle + label */}
            <div className="flex flex-col items-center gap-1">
              <div
                aria-current={isActive ? "step" : undefined}
                className={cn(
                  "flex size-7 items-center justify-center rounded-full text-xs font-medium transition-colors duration-fast",
                  isCompleted && "bg-primary text-primary-foreground",
                  isActive && "bg-primary text-primary-foreground",
                  !isCompleted && !isActive && "border border-border text-muted-foreground",
                )}
              >
                {isCompleted ? (
                  <CheckIcon className="size-3.5" />
                ) : (
                  displayNumber
                )}
              </div>
              <span
                className={cn(
                  "hidden text-xs sm:block",
                  isActive ? "font-medium text-foreground" : "text-muted-foreground",
                )}
              >
                {step.label}
              </span>
            </div>
          </div>
        );
      })}
    </nav>
  );
}
