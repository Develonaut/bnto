import { PhaseStepCircle } from "./PhaseStepCircle";

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
 * Three phases: Files -> Configure -> Results.
 * When `hasConfig` is false, the Configure step is omitted
 * and display numbers adjust to stay sequential (1, 2 instead of 1, 3).
 * Circles + labels on desktop, circles only on mobile.
 */
export function PhaseIndicator({ activePhase, hasConfig = true }: PhaseIndicatorProps) {
  const steps = hasConfig ? ALL_STEPS : ALL_STEPS.filter((s) => s.phase !== 2);

  return (
    <nav aria-label="Recipe progress" className="flex items-center justify-center gap-0">
      {steps.map((step, i) => (
        <PhaseStepCircle
          key={step.phase}
          phase={step.phase}
          label={step.label}
          displayNumber={i + 1}
          isCompleted={step.phase < activePhase}
          isActive={step.phase === activePhase}
          showConnector={i > 0}
        />
      ))}
    </nav>
  );
}
