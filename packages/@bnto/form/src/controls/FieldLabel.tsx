"use client";

import type { ReactNode } from "react";
import { Label } from "@bnto/ui";

interface FieldLabelProps {
  htmlFor: string;
  title?: string;
  required?: boolean;
  children: ReactNode;
}

/** Renders a label with optional required indicator. Shared by SliderControl and SchemaField. */
function FieldLabel({ htmlFor, title, required, children }: FieldLabelProps) {
  return (
    <Label htmlFor={htmlFor} title={title}>
      {children}
      {required && <span className="ml-0.5 text-destructive">*</span>}
    </Label>
  );
}

export { FieldLabel };
export type { FieldLabelProps };
