"use client";

import { Grid, GridItem } from "@bnto/ui";

import { StepCard } from "./StepCard";

export interface StepFlowStep {
  step: number;
  title: string;
  description: string;
  mascot: string;
  mascotHeight?: number;
  variant: "primary" | "secondary" | "accent";
}

interface StepFlowLayoutProps {
  steps: StepFlowStep[];
  baseDelay?: number;
}

export function StepFlowLayout({ steps, baseDelay = 200 }: StepFlowLayoutProps) {
  return (
    <div
      className="rounded-lg border bg-muted p-3 lg:p-4"
      style={{ borderColor: "var(--surface-muted-wall)" }}
    >
      <Grid cols={{ mobile: 1, desktop: 3 }} gap="md">
        {steps.map((step, i) => (
          <GridItem key={step.step} colSpan={1}>
            <StepCard {...step} delay={baseDelay + i * 100} className="h-full" />
          </GridItem>
        ))}
      </Grid>
    </div>
  );
}
