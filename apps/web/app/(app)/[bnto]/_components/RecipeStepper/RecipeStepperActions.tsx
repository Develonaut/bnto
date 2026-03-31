"use client";

import type { ReactNode } from "react";
import { Row } from "@bnto/ui";

interface RecipeStepperActionsProps {
  children: ReactNode;
  className?: string;
}

/** Thin Row wrapper for grouping action buttons. */
export function RecipeStepperActions({ children, className }: RecipeStepperActionsProps) {
  return (
    <Row gap="xs" className={className}>
      {children}
    </Row>
  );
}
