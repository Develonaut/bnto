import { ComparisonBarTrack } from "./ComparisonBarTrack";
import { ComparisonBarLabel } from "./ComparisonBarLabel";

export interface ComparisonBarItemProps {
  label: string;
  value: number;
  max: number;
  mounted: boolean;
  barClassName?: string;
  primary: boolean;
  subtitle?: string;
  height: string;
  delay: number;
  formatValue: (value: number) => string;
}

export function ComparisonBarItem(props: ComparisonBarItemProps) {
  const widthPercent = Math.max((props.value / props.max) * 100, 2);

  return (
    <div className="flex flex-col gap-1">
      <ComparisonBarTrack
        widthPercent={widthPercent}
        mounted={props.mounted}
        barClassName={props.barClassName}
        primary={props.primary}
        height={props.height}
        delay={props.delay}
      />
      <ComparisonBarLabel
        label={props.label}
        value={props.value}
        primary={props.primary}
        subtitle={props.subtitle}
        formatValue={props.formatValue}
      />
    </div>
  );
}
