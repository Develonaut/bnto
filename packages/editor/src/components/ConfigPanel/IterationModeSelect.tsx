"use client";

import {
  Label,
  Text,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@bnto/ui";

interface IterationModeSelectProps {
  value: string;
  onChange: (value: string) => void;
}

/** File iteration mode selector with description text. */
function IterationModeSelect({ value, onChange }: IterationModeSelectProps) {
  const description =
    value === "auto"
      ? "Automatically processes each file through the pipeline."
      : "Requires explicit loop nodes for per-file iteration.";

  return (
    <div className="flex flex-col gap-1.5">
      <Label>File Iteration</Label>
      <Select value={value} onValueChange={onChange}>
        <SelectTrigger>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="auto">Auto</SelectItem>
          <SelectItem value="explicit">Explicit</SelectItem>
        </SelectContent>
      </Select>
      <Text size="xs" color="muted">
        {description}
      </Text>
    </div>
  );
}

export { IterationModeSelect };
