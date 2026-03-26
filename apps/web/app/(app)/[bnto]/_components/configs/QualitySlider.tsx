import { Label, Slider } from "@bnto/ui";

interface QualitySliderProps {
  value: number;
  sliderValue: number[];
  onChange: (value: number[]) => void;
}

/** Quality slider with label, percentage display, and help text. */
export function QualitySlider({ value, sliderValue, onChange }: QualitySliderProps) {
  return (
    <div className="flex min-w-0 flex-1 flex-col gap-1">
      <Label id="convert-quality-label" className="text-muted-foreground text-xs">
        Quality
      </Label>
      <div className="flex items-center gap-3">
        <Slider
          className="w-full"
          aria-labelledby="convert-quality-label"
          aria-describedby="convert-quality-help"
          aria-valuetext={`${value} percent`}
          value={sliderValue}
          onValueChange={onChange}
          min={1}
          max={100}
          step={1}
        />
        <span className="text-muted-foreground shrink-0 font-mono text-sm tabular-nums">
          {value}%
        </span>
      </div>
      <p id="convert-quality-help" className="text-muted-foreground text-xs">
        Lower values reduce file size
      </p>
    </div>
  );
}
