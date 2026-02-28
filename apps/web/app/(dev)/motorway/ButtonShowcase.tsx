"use client";

import { useState } from "react";

import {
  ArrowRightIcon,
  BoldIcon,
  CheckIcon,
  DownloadIcon,
  FileTextIcon,
  HeartIcon,
  ItalicIcon,
  PinIcon,
  RotateCcwIcon,
  StarIcon,
  UnderlineIcon,
  ZapIcon,
} from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Heading } from "@/components/ui/Heading";
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

function ToggleShowcase() {
  const [bold, setBold] = useState(false);
  const [italic, setItalic] = useState(false);
  const [underline, setUnderline] = useState(true);
  const [pinned, setPinned] = useState(false);
  const [starred, setStarred] = useState(true);

  return (
    <Stack className="gap-6">
      <Stack className="gap-1">
        <Heading level={4}>Toggle Buttons</Heading>
        <Text size="sm" color="muted">
          Push-button metaphor — toggled ON stays depressed at hover depth. Click to pop back up.
        </Text>
      </Stack>

      {/* Text formatting toolbar */}
      <Stack className="gap-2">
        <Text size="xs" color="muted" mono as="span">Formatting toolbar</Text>
        <Row className="gap-2">
          <Button variant="outline" size="icon" toggle pressed={bold} onClick={() => setBold(!bold)}>
            <BoldIcon />
          </Button>
          <Button variant="outline" size="icon" toggle pressed={italic} onClick={() => setItalic(!italic)}>
            <ItalicIcon />
          </Button>
          <Button variant="outline" size="icon" toggle pressed={underline} onClick={() => setUnderline(!underline)}>
            <UnderlineIcon />
          </Button>
        </Row>
      </Stack>

      {/* Action toggles across variants */}
      <Stack className="gap-2">
        <Text size="xs" color="muted" mono as="span">Action toggles</Text>
        <Row className="gap-2">
          <Button variant="primary" size="icon" toggle pressed={pinned} onClick={() => setPinned(!pinned)}>
            <PinIcon />
          </Button>
          <Button variant="warning" size="icon" toggle pressed={starred} onClick={() => setStarred(!starred)}>
            <StarIcon />
          </Button>
          <Button variant="success" size="md" toggle pressed={pinned} onClick={() => setPinned(!pinned)}>
            <PinIcon /> {pinned ? "Pinned" : "Pin"}
          </Button>
          <Button variant="secondary" size="md" toggle pressed={starred} onClick={() => setStarred(!starred)}>
            <StarIcon /> {starred ? "Starred" : "Star"}
          </Button>
        </Row>
      </Stack>
    </Stack>
  );
}

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
            size="md"
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
              <Button variant={value} size="md" hovered={hovered} pressed={pressed} disabled={disabled}>Label</Button>
              <Button variant={value} size="icon" hovered={hovered} pressed={pressed} disabled={disabled}><ZapIcon /></Button>
              {iconEntry && (
                <Button variant={value} size="md" hovered={hovered} pressed={pressed} disabled={disabled}>
                  {iconEntry.trailing ? <>{iconLabel} {iconEntry.icon}</> : <>{iconEntry.icon} {iconLabel}</>}
                </Button>
              )}
            </Row>
          );
        })}
      </Stack>

      {/* Toggle demo */}
      <ToggleShowcase />
    </Stack>
  );
}
