"use client";

import {
  createContext,
  useContext,
  useMemo,
  type ReactNode,
  type HTMLAttributes,
  Children,
  isValidElement,
} from "react";
import { cn } from "../utils/cn";
import { CheckIcon } from "../icons";

/* ── Types ──────────────────────────────────────────────────── */

interface StepDef {
  value: string;
  label: string;
}

interface StepperContextValue {
  activeStep: string;
  steps: StepDef[];
  setActiveStep: (value: string) => void;
}

/* ── Context ────────────────────────────────────────────────── */

const StepperContext = createContext<StepperContextValue | null>(null);

function useStepper() {
  const ctx = useContext(StepperContext);
  if (!ctx) throw new Error("Stepper compound components must be used within <Stepper>");
  return ctx;
}

/* ── Stepper (Root) ─────────────────────────────────────────── */

interface StepperProps extends HTMLAttributes<HTMLDivElement> {
  /** The active step value (controlled). */
  value: string;
  /** Called when the active step should change. */
  onValueChange: (value: string) => void;
  children: ReactNode;
}

export function Stepper({ value, onValueChange, children, ...divProps }: StepperProps) {
  /** Collect StepDef[] from StepperStep children nested inside StepperList. */
  const steps = useMemo(() => collectSteps(children), [children]);

  const ctx = useMemo<StepperContextValue>(
    () => ({ activeStep: value, steps, setActiveStep: onValueChange }),
    [value, steps, onValueChange],
  );

  return (
    <StepperContext.Provider value={ctx}>
      <div {...divProps}>{children}</div>
    </StepperContext.Provider>
  );
}

/* ── StepperList (registry, renders nothing) ────────────────── */

interface StepperListProps {
  children: ReactNode;
}

/**
 * Collects StepperStep declarations. Renders nothing —
 * steps are read from children in the Stepper root via collectSteps().
 */
export function StepperList(_props: StepperListProps) {
  return null;
}
StepperList.displayName = "StepperList";

/* ── StepperStep (declaration, renders nothing) ─────────────── */

interface StepperStepProps {
  /** Unique step identifier — matches StepperContent value. */
  value: string;
  /** Human-readable label shown in StepperIndicator. */
  label: string;
}

/** Declares a step. Renders nothing — used for registration only. */
export function StepperStep(_props: StepperStepProps) {
  return null;
}
StepperStep.displayName = "StepperStep";

/* ── StepperContent ─────────────────────────────────────────── */

interface StepperContentProps {
  /** Which step this content belongs to. */
  value: string;
  children: ReactNode;
}

/** Renders children only when the active step matches `value`. */
export function StepperContent({ value, children }: StepperContentProps) {
  const { activeStep } = useStepper();
  if (activeStep !== value) return null;
  return <>{children}</>;
}

/* ── StepperIndicator ───────────────────────────────────────── */

interface StepperIndicatorProps {
  className?: string;
}

/**
 * Visual stepper — horizontal circles with labels and connectors.
 * Reads steps + activeStep from context. Labels hidden on mobile.
 */
export function StepperIndicator({ className }: StepperIndicatorProps) {
  const { steps, activeStep } = useStepper();
  const activeIndex = steps.findIndex((s) => s.value === activeStep);

  return (
    <nav aria-label="Progress" className={cn("flex items-center justify-center gap-0", className)}>
      {steps.map((step, i) => {
        const isCompleted = i < activeIndex;
        const isActive = step.value === activeStep;

        return (
          <div key={step.value} className="flex items-center">
            {i > 0 && <StepConnector filled={isCompleted || isActive} />}
            <div className="flex w-16 flex-col items-center gap-1 sm:w-20">
              <StepBadge
                displayNumber={i + 1}
                label={step.label}
                isCompleted={isCompleted}
                isActive={isActive}
              />
              <StepLabel label={step.label} isActive={isActive} />
            </div>
          </div>
        );
      })}
    </nav>
  );
}

/* ── Internal sub-components ────────────────────────────────── */

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

function stepBadgeCn(isCompleted: boolean, isActive: boolean) {
  return cn(
    "surface pressable pointer-events-none flex size-7 items-center justify-center rounded-full text-xs font-medium transition-colors duration-fast",
    (isCompleted || isActive) && "surface-primary elevation-sm bg-primary text-primary-foreground",
    !isCompleted &&
      !isActive &&
      "surface-card elevation-none border border-border text-muted-foreground",
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
      className={stepBadgeCn(isCompleted, isActive)}
    >
      {isCompleted ? (
        <CheckIcon aria-hidden="true" className="size-3.5" />
      ) : (
        <span aria-hidden="true">{displayNumber}</span>
      )}
    </div>
  );
}

/* ── Utility: collect StepDef from JSX tree ─────────────────── */

function collectSteps(children: ReactNode): StepDef[] {
  const steps: StepDef[] = [];
  Children.forEach(children, (child) => {
    if (!isValidElement(child)) return;
    if (child.type === StepperList) {
      const listProps = child.props as StepperListProps;
      Children.forEach(listProps.children, (stepChild) => {
        if (isValidElement(stepChild) && stepChild.type === StepperStep) {
          const { value, label } = stepChild.props as StepperStepProps;
          steps.push({ value, label });
        }
      });
    }
  });
  return steps;
}
