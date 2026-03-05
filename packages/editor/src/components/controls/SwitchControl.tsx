"use client";

import { Switch } from "@bnto/ui";
import type { ControlProps } from "./types";

function SwitchControl({ id, value, onChange }: ControlProps) {
  return (
    <Switch
      id={id}
      checked={Boolean(value ?? false)}
      onCheckedChange={onChange}
      data-testid={`control-switch-${id}`}
    />
  );
}

export { SwitchControl };
