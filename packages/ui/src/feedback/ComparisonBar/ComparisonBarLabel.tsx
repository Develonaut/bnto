import { cn } from "../../utils/cn";

interface ComparisonBarLabelProps {
  label: string;
  value: number;
  primary: boolean;
  subtitle?: string;
  formatValue: (value: number) => string;
}

export function ComparisonBarLabel({
  label,
  value,
  primary,
  subtitle,
  formatValue,
}: ComparisonBarLabelProps) {
  return (
    <div className="flex items-baseline justify-between">
      <div className="flex flex-col">
        <span
          className={cn(
            "text-[10px] font-medium",
            primary ? "text-primary" : "text-muted-foreground",
          )}
        >
          {label}
        </span>
        {subtitle && <span className="text-muted-foreground/50 text-[9px]">{subtitle}</span>}
      </div>
      <span className="text-muted-foreground text-[10px]">
        {value > 0 ? formatValue(value) : ""}
      </span>
    </div>
  );
}
