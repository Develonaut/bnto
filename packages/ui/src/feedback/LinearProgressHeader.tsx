import type { ReactNode } from "react";

interface LinearProgressHeaderProps {
  icon?: ReactNode;
  label?: string;
  valueLabel: string;
}

/** Header row above the progress bar: [icon] Label ... value */
export function LinearProgressHeader({ icon, label, valueLabel }: LinearProgressHeaderProps) {
  return (
    <div className="flex items-center justify-between gap-2">
      <div className="flex min-w-0 items-center gap-2">
        {icon}
        {label && <span className="truncate text-sm text-muted-foreground">{label}</span>}
      </div>
      <span className="shrink-0 font-mono text-sm tabular-nums text-muted-foreground">
        {valueLabel}
      </span>
    </div>
  );
}
