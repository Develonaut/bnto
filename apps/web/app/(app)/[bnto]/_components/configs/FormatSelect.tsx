import { Label, Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@bnto/ui";

const FORMAT_OPTIONS = [
  { value: "webp", label: "WebP" },
  { value: "jpeg", label: "JPEG" },
  { value: "png", label: "PNG" },
] as const;

interface FormatSelectProps {
  id: string;
  value: string;
  onChange: (format: string) => void;
  helpText?: string;
}

/** Shared format select dropdown used across image config panels. */
export function FormatSelect({ id, value, onChange, helpText }: FormatSelectProps) {
  const labelId = `${id}-label`;

  return (
    <div className="flex shrink-0 flex-col gap-1">
      <Label id={labelId} className="text-muted-foreground text-xs">
        Format
      </Label>
      <Select value={value} onValueChange={onChange}>
        <SelectTrigger className="w-24" aria-labelledby={labelId} data-testid="format-select">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {FORMAT_OPTIONS.map((opt) => (
            <SelectItem
              key={opt.value}
              value={opt.value}
              data-testid={`format-option-${opt.value}`}
            >
              {opt.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      {helpText && <p className="text-muted-foreground text-xs">{helpText}</p>}
    </div>
  );
}
