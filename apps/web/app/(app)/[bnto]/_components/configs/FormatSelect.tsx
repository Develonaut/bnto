import type { ReactNode } from "react";
import {
  FormControl,
  FormLabel,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@bnto/ui";
import { FORMAT_OPTIONS } from "./formatOptions";

interface FormatSelectProps {
  format: string;
  labelId: string;
  onFormatChange: (f: string) => void;
  describedBy?: string;
  children?: ReactNode;
}

const FormatOptions = FORMAT_OPTIONS.map((opt) => (
  <SelectItem key={opt.value} value={opt.value} data-testid={`format-option-${opt.value}`}>
    {opt.label}
  </SelectItem>
));

export function FormatSelect({
  format,
  labelId,
  onFormatChange,
  describedBy,
  children,
}: FormatSelectProps) {
  return (
    <FormControl className="shrink-0">
      <FormLabel id={labelId}>Format</FormLabel>
      <Select value={format} onValueChange={onFormatChange}>
        <SelectTrigger
          className="w-24"
          aria-labelledby={labelId}
          aria-describedby={describedBy}
          data-testid="format-select"
        >
          <SelectValue />
        </SelectTrigger>
        <SelectContent>{FormatOptions}</SelectContent>
      </Select>
      {children}
    </FormControl>
  );
}
