"use client";

import type { ReactNode } from "react";
import { Row } from "@bnto/ui";

interface RecipeFlowActionsProps {
  children: ReactNode;
  className?: string;
}

/** Thin Row wrapper for grouping action buttons. */
export function RecipeFlowActions({ children, className }: RecipeFlowActionsProps) {
  return (
    <Row gap="xs" className={className}>
      {children}
    </Row>
  );
}
