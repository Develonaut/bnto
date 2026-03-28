"use client";

import { ITERATION_MODES } from "@bnto/core";
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

const DESCRIPTIONS: Record<string, string> = {
  auto: "Automatically processes each file through the pipeline.",
  explicit: "Requires explicit loop nodes for per-file iteration.",
};

/** File iteration mode selector with description text. */
function IterationModeSelect({ value, onChange }: IterationModeSelectProps) {
  const description = DESCRIPTIONS[value] ?? "";

  return (
    <div className="flex flex-col gap-1.5">
      <div className="flex items-center justify-between gap-2">
        <Label>File Iteration</Label>
        <Select value={value} onValueChange={onChange}>
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {ITERATION_MODES.map((mode) => (
              <SelectItem key={mode} value={mode}>
                {mode.charAt(0).toUpperCase() + mode.slice(1)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>
      <Text size="xs" color="muted">
        {description}
      </Text>
    </div>
  );
}

export { IterationModeSelect };
