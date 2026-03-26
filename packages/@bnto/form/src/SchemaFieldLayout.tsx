import type { ReactNode } from "react";
import type { NodeParamControl } from "@bnto/core";
import { getFieldLayout } from "./fieldLayout";

interface SchemaFieldLayoutProps {
  name: string;
  control: NodeParamControl;
  label: ReactNode;
  controlEl: ReactNode;
}

/** Wraps a field control with layout determined by control type. */
function SchemaFieldLayout({ name, control, label, controlEl }: SchemaFieldLayoutProps) {
  const layout = getFieldLayout(control);
  const testId = `schema-field-${name}`;

  if (layout === "self-labeled") {
    return <div data-testid={testId}>{controlEl}</div>;
  }

  if (layout === "inline") {
    return (
      <div className="flex items-center justify-between gap-2" data-testid={testId}>
        {label}
        {controlEl}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-1.5" data-testid={testId}>
      {label}
      {controlEl}
    </div>
  );
}

export { SchemaFieldLayout };
export type { SchemaFieldLayoutProps };
