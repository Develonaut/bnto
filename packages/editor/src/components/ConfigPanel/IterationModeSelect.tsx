"use client";

import { ITERATION_MODES, t, type StringKey } from "@bnto/core";
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

const DESCRIPTION_KEYS: Record<string, StringKey> = {
  auto: "editor.iterationAutoDescription",
  explicit: "editor.iterationExplicitDescription",
};

/** File iteration mode selector with description text. */
function IterationModeSelect({ value, onChange }: IterationModeSelectProps) {
  const descriptionKey = DESCRIPTION_KEYS[value];
  const description = descriptionKey ? t(descriptionKey) : "";

  return (
    <div className="flex flex-col gap-1.5">
      <div className="flex items-center justify-between gap-2">
        <Label>{t("editor.iterationLabel")}</Label>
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
