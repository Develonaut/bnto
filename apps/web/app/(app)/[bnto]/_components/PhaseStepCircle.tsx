import { cn, CheckIcon } from "@bnto/ui";

type Phase = 1 | 2 | 3;

interface PhaseStepCircleProps {
  phase: Phase;
  label: string;
  displayNumber: number;
  isCompleted: boolean;
  isActive: boolean;
  showConnector: boolean;
}

/** A single step in the phase indicator: connector line + numbered circle + label. */
export function PhaseStepCircle({
  phase,
  label,
  displayNumber,
  isCompleted,
  isActive,
  showConnector,
}: PhaseStepCircleProps) {
  return (
    <div key={phase} className="flex items-center">
      {showConnector && <StepConnector filled={isCompleted || isActive} />}
      <div className="flex w-16 flex-col items-center gap-1 sm:w-20">
        <StepBadge
          displayNumber={displayNumber}
          label={label}
          isCompleted={isCompleted}
          isActive={isActive}
        />
        <StepLabel label={label} isActive={isActive} />
      </div>
    </div>
  );
}

function StepConnector({ filled }: { filled: boolean }) {
  return (
    <div
      aria-hidden="true"
      className={cn("h-0.5 w-6 sm:w-10", filled ? "bg-primary" : "bg-border")}
    />
  );
}

function StepLabel({ label, isActive }: { label: string; isActive: boolean }) {
  return (
    <span
      aria-hidden="true"
      className={cn(
        "hidden text-xs sm:block",
        isActive ? "font-medium text-foreground" : "text-muted-foreground",
      )}
    >
      {label}
    </span>
  );
}

function StepBadge({
  displayNumber,
  label,
  isCompleted,
  isActive,
}: {
  displayNumber: number;
  label: string;
  isCompleted: boolean;
  isActive: boolean;
}) {
  return (
    <div
      role="img"
      aria-current={isActive ? "step" : undefined}
      aria-label={`Step ${displayNumber}: ${label}${isCompleted ? " (completed)" : ""}`}
      className={cn(
        "surface pressable pointer-events-none flex size-7 items-center justify-center rounded-full text-xs font-medium transition-colors duration-fast",
        (isCompleted || isActive) &&
          "surface-primary elevation-sm bg-primary text-primary-foreground",
        !isCompleted &&
          !isActive &&
          "surface-card elevation-none border border-border text-muted-foreground",
      )}
    >
      {isCompleted ? (
        <CheckIcon aria-hidden="true" className="size-3.5" />
      ) : (
        <span aria-hidden="true">{displayNumber}</span>
      )}
    </div>
  );
}
