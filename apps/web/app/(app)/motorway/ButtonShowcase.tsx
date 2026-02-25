"use client";

import { useState } from "react";

import {
  ArrowRightIcon,
  CheckIcon,
  DownloadIcon,
  FileTextIcon,
  HeartIcon,
  RotateCcwIcon,
  StarIcon,
  ZapIcon,
} from "@/components/ui/icons";

import { Button } from "@/components/ui/button";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

type Variant = "primary" | "secondary" | "outline" | "muted" | "destructive" | "success" | "warning";
type ButtonState = "resting" | "hover" | "pressed" | "disabled";

const VARIANTS: { value: Variant; label: string; iconLabel?: string }[] = [
  { value: "primary", label: "Primary", iconLabel: "Download" },
  { value: "secondary", label: "Secondary", iconLabel: "Favorite" },
  { value: "outline", label: "Outline", iconLabel: "Next" },
  { value: "muted", label: "Muted", iconLabel: "Draft" },
  { value: "destructive", label: "Destructive", iconLabel: "Remove" },
  { value: "success", label: "Success", iconLabel: "Done" },
  { value: "warning", label: "Warning", iconLabel: "Retry" },
];

const ICON_LABELS: Record<string, { icon: React.ReactNode; trailing?: boolean }> = {
  Download: { icon: <DownloadIcon /> },
  Favorite: { icon: <StarIcon /> },
  Next: { icon: <ArrowRightIcon />, trailing: true },
  Draft: { icon: <FileTextIcon /> },
  Remove: { icon: <HeartIcon /> },
  Done: { icon: <CheckIcon /> },
  Retry: { icon: <RotateCcwIcon /> },
};

const STATES: { value: ButtonState; label: string }[] = [
  { value: "resting", label: "Resting" },
  { value: "hover", label: "Hover" },
  { value: "pressed", label: "Pressed" },
  { value: "disabled", label: "Disabled" },
];

export function ButtonShowcase() {
  const [state, setState] = useState<ButtonState>("resting");

  const hovered = state === "hover";
  const pressed = state === "pressed";
  const disabled = state === "disabled";

  return (
    <Stack className="gap-10">
      {/* State toggle */}
      <Row className="gap-2">
        {STATES.map(({ value, label }) => (
          <Button
            key={value}
            variant={state === value ? "secondary" : "outline"}
            size="sm"
            onClick={() => setState(value)}
          >
            {label}
          </Button>
        ))}
      </Row>

      {/* All variants × sizes */}
      <Stack gap="md">
        {VARIANTS.map(({ value, label, iconLabel }) => {
          const iconEntry = iconLabel ? ICON_LABELS[iconLabel] : null;
          return (
            <Row key={value} className="items-center gap-3">
              <Text size="xs" color="muted" mono as="span" className="w-24 shrink-0">
                {label}
              </Text>
              <Button variant={value} size="sm" hovered={hovered} pressed={pressed} disabled={disabled}>Label</Button>
              <Button variant={value} size="md" hovered={hovered} pressed={pressed} disabled={disabled}>Label</Button>
              <Button variant={value} size="lg" hovered={hovered} pressed={pressed} disabled={disabled}>Label</Button>
              <Button variant={value} size="icon-sm" hovered={hovered} pressed={pressed} disabled={disabled}><ZapIcon /></Button>
              <Button variant={value} size="icon" hovered={hovered} pressed={pressed} disabled={disabled}><ZapIcon /></Button>
              <Button variant={value} size="icon-lg" hovered={hovered} pressed={pressed} disabled={disabled}><ZapIcon /></Button>
              {iconEntry && (
                <Button variant={value} size="md" hovered={hovered} pressed={pressed} disabled={disabled}>
                  {iconEntry.trailing ? <>{iconLabel} {iconEntry.icon}</> : <>{iconEntry.icon} {iconLabel}</>}
                </Button>
              )}
            </Row>
          );
        })}
      </Stack>
    </Stack>
  );
}
