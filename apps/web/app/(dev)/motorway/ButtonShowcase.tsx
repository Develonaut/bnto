"use client";

import { useState, type ReactNode } from "react";

import {
  ArrowRightIcon,
  BoldIcon,
  CheckIcon,
  DownloadIcon,
  FileTextIcon,
  HeartIcon,
  ItalicIcon,
  PenLineIcon,
  PinIcon,
  RotateCcwIcon,
  StarIcon,
  TrashIcon,
  UnderlineIcon,
  ZapIcon,
} from "@bnto/ui";

import { Button, Card, Heading, Row, Stack, Text, cn } from "@bnto/ui";

type Variant =
  | "primary"
  | "secondary"
  | "outline"
  | "ghost"
  | "muted"
  | "destructive"
  | "success"
  | "warning";
type ButtonState = "resting" | "hover" | "pressed" | "disabled" | "dormant";

const VARIANTS: { value: Variant; label: string; iconLabel?: string }[] = [
  { value: "primary", label: "Primary", iconLabel: "Download" },
  { value: "secondary", label: "Secondary", iconLabel: "Favorite" },
  { value: "outline", label: "Outline", iconLabel: "Next" },
  { value: "ghost", label: "Ghost", iconLabel: "Edit" },
  { value: "muted", label: "Muted", iconLabel: "Draft" },
  { value: "destructive", label: "Destructive", iconLabel: "Remove" },
  { value: "success", label: "Success", iconLabel: "Done" },
  { value: "warning", label: "Warning", iconLabel: "Retry" },
];

const ICON_LABELS: Record<string, { icon: ReactNode; trailing?: boolean }> = {
  Download: { icon: <DownloadIcon /> },
  Favorite: { icon: <StarIcon /> },
  Next: { icon: <ArrowRightIcon />, trailing: true },
  Edit: { icon: <PenLineIcon /> },
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
  { value: "dormant", label: "Dormant" },
];

/* ── Variant grid ──────────────────────────────────────────── */

function VariantGrid({ state }: { state: ButtonState }) {
  const hovered = state === "hover";
  const pressed = state === "pressed";
  const disabled = state === "disabled";
  const dormant = state === "dormant";

  const shared = { hovered, pressed, disabled, dormant } as const;

  return (
    <Stack gap="md">
      {VARIANTS.map(({ value, label, iconLabel }) => {
        const iconEntry = iconLabel ? ICON_LABELS[iconLabel] : null;
        return (
          <Row key={value} className={cn("items-center gap-3", dormant && "group")}>
            <Text
              size="xs"
              color="muted"
              className="w-20 shrink-0 font-mono uppercase tracking-wider"
            >
              {label}
            </Text>

            {/* sm */}
            <Button variant={value} size="sm" {...shared}>
              Label
            </Button>
            <Button variant={value} size="sm" icon={<ZapIcon />} {...shared} />

            {/* md (default) */}
            <Button variant={value} {...shared}>
              Label
            </Button>
            <Button variant={value} size="icon" {...shared}>
              <ZapIcon />
            </Button>
            {iconEntry && (
              <Button variant={value} {...shared}>
                {iconEntry.trailing ? (
                  <>
                    {iconLabel} {iconEntry.icon}
                  </>
                ) : (
                  <>
                    {iconEntry.icon} {iconLabel}
                  </>
                )}
              </Button>
            )}
          </Row>
        );
      })}
    </Stack>
  );
}

/* ── Toggle demo ───────────────────────────────────────────── */

function ToggleDemo() {
  const [bold, setBold] = useState(false);
  const [italic, setItalic] = useState(false);
  const [underline, setUnderline] = useState(true);
  const [pinned, setPinned] = useState(false);
  const [starred, setStarred] = useState(true);

  return (
    <Stack className="gap-4">
      <Row className="gap-2">
        <Button variant="outline" size="icon" toggle pressed={bold} onClick={() => setBold(!bold)}>
          <BoldIcon />
        </Button>
        <Button
          variant="outline"
          size="icon"
          toggle
          pressed={italic}
          onClick={() => setItalic(!italic)}
        >
          <ItalicIcon />
        </Button>
        <Button
          variant="outline"
          size="icon"
          toggle
          pressed={underline}
          onClick={() => setUnderline(!underline)}
        >
          <UnderlineIcon />
        </Button>
      </Row>
      <Row className="gap-2">
        <Button
          variant="primary"
          size="icon"
          toggle
          pressed={pinned}
          onClick={() => setPinned(!pinned)}
        >
          <PinIcon />
        </Button>
        <Button
          variant="warning"
          size="icon"
          toggle
          pressed={starred}
          onClick={() => setStarred(!starred)}
        >
          <StarIcon />
        </Button>
        <Button variant="success" toggle pressed={pinned} onClick={() => setPinned(!pinned)}>
          <PinIcon /> {pinned ? "Pinned" : "Pin"}
        </Button>
      </Row>
    </Stack>
  );
}

/* ── Dormant demo ──────────────────────────────────────────── */

function DormantDemo() {
  return (
    <Card elevation="sm" className="flex items-center gap-4 px-5 py-4">
      <Stack className="min-w-0 flex-1 gap-0.5">
        <Text weight="medium">Recipe name</Text>
        <Text size="xs" color="muted">
          3 nodes &middot; 2 hours ago
        </Text>
      </Stack>
      <Row className="gap-1">
        <Button icon={<PenLineIcon />} variant="primary" dormant />
        <Button icon={<TrashIcon />} variant="destructive" dormant />
      </Row>
    </Card>
  );
}

/* ── Main showcase ──────────────────────────────────────────── */

export function ButtonShowcase() {
  const [state, setState] = useState<ButtonState>("resting");

  return (
    <Stack className="gap-10">
      {/* State toggle bar */}
      <Row className="gap-2">
        {STATES.map(({ value, label }) => (
          <Button
            key={value}
            variant={state === value ? "secondary" : "outline"}
            onClick={() => setState(value)}
          >
            {label}
          </Button>
        ))}
      </Row>

      {/* All variants */}
      <VariantGrid state={state} />

      {/* Toggle */}
      <Stack className="gap-4">
        <Stack className="gap-1">
          <Heading level={4}>Toggle Buttons</Heading>
          <Text size="sm" color="muted">
            Push-button metaphor — toggled ON stays depressed at hover depth. Click to pop back up.
          </Text>
        </Stack>
        <ToggleDemo />
      </Stack>

      {/* Dormant */}
      <Stack className="gap-4">
        <Stack className="gap-1">
          <Heading level={4}>Dormant Buttons</Heading>
          <Text size="sm" color="muted">
            Grounded and muted by default. Self-managed group wrapper — each button wakes on its own
            hover. CSS-first, no JS state.
          </Text>
        </Stack>
        <DormantDemo />
      </Stack>
    </Stack>
  );
}
