"use client";

import { useState } from "react";
import { ArrowRight, Download, Zap } from "lucide-react";

import type { SpringMode } from "@/components/ui/button";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";
import { Row } from "@/components/ui/Row";
import { Tabs } from "@/components/ui/tabs";

/* ── Option types ────────────────────────────────────────── */

type Variant = "primary" | "secondary" | "outline" | "muted" | "destructive" | "success" | "warning";
type Size = "sm" | "md" | "lg";

const VARIANTS: { value: Variant; label: string }[] = [
  { value: "primary", label: "Primary" },
  { value: "secondary", label: "Secondary" },
  { value: "outline", label: "Outline" },
  { value: "muted", label: "Muted" },
  { value: "destructive", label: "Destructive" },
  { value: "success", label: "Success" },
  { value: "warning", label: "Warning" },
];

const SIZES: { value: Size; label: string }[] = [
  { value: "sm", label: "sm" },
  { value: "md", label: "md" },
  { value: "lg", label: "lg" },
];

const SPRINGS: { value: SpringMode; label: string }[] = [
  { value: "sm", label: "sm (firm)" },
  { value: "md", label: "md (bounce)" },
  { value: "lg", label: "lg (elastic)" },
];

/* ── Toggle group ────────────────────────────────────────── */

function ToggleGroup<T extends string>({
  label,
  options,
  value,
  onChange,
}: {
  label: string;
  options: { value: T; label: string }[];
  value: T;
  onChange: (v: T) => void;
}) {
  return (
    <Row className="items-center gap-3 flex-wrap">
      <Text size="xs" color="muted" mono className="w-16 shrink-0">
        {label}
      </Text>
      <Row className="gap-1.5 flex-wrap">
        {options.map((opt) => (
          <Button
            key={opt.value}
            variant={opt.value === value ? "primary" : "outline"}
            size="sm"
            spring="sm"
            depth={opt.value === value}
            onClick={() => onChange(opt.value)}
          >
            {opt.label}
          </Button>
        ))}
      </Row>
    </Row>
  );
}

/* ── Playground tab ──────────────────────────────────────── */

function Playground() {
  const [variant, setVariant] = useState<Variant>("primary");
  const [size, setSize] = useState<Size>("md");
  const [spring, setSpring] = useState<SpringMode>("lg");

  return (
    <Stack gap="lg">
      <Stack gap="sm">
        <ToggleGroup label="variant" options={VARIANTS} value={variant} onChange={setVariant} />
        <ToggleGroup label="size" options={SIZES} value={size} onChange={setSize} />
        <ToggleGroup label="spring" options={SPRINGS} value={spring} onChange={setSpring} />
      </Stack>

      <Row className="justify-center gap-6 items-end pt-4">
        <Stack gap="xs" className="items-center">
          <Button
            variant={variant}
            size={size === "sm" ? "icon-sm" : size === "lg" ? "icon-lg" : "icon"}
            spring={spring}
            data-testid="showcase-btn-icon"
          >
            <Zap />
          </Button>
          <Text size="xs" color="muted" mono>icon</Text>
        </Stack>

        <Stack gap="xs" className="items-center">
          <Button
            variant={variant}
            size={size}
            spring={spring}
            data-testid="showcase-btn-icon-text"
          >
            <Download />
            Download
          </Button>
          <Text size="xs" color="muted" mono>icon + text</Text>
        </Stack>

        <Stack gap="xs" className="items-center">
          <Button
            variant={variant}
            size={size}
            spring={spring}
            data-testid="showcase-btn-text"
          >
            Learn More
          </Button>
          <Text size="xs" color="muted" mono>text</Text>
        </Stack>
      </Row>
    </Stack>
  );
}

/* ── All variants tab ────────────────────────────────────── */

function AllVariants() {
  return (
    <Stack gap="lg">
      {VARIANTS.map(({ value, label }) => (
        <Row key={value} className="gap-4 items-center">
          <Text size="xs" color="muted" mono as="span" className="w-24 shrink-0">
            {label}
          </Text>
          <Button variant={value} size="icon">
            <ArrowRight />
          </Button>
          <Button variant={value} size="sm">
            Learn More
          </Button>
          <Button variant={value} size="md">
            Learn More
          </Button>
          <Button variant={value} size="lg" data-testid={`pressable-${label.toLowerCase()}-lg`}>
            Learn More
          </Button>
        </Row>
      ))}
    </Stack>
  );
}

/* ── Root ────────────────────────────────────────────────── */

export function ButtonShowcase() {
  return (
    <Tabs defaultValue="playground">
      <Tabs.List>
        <Tabs.Trigger value="playground">Playground</Tabs.Trigger>
        <Tabs.Trigger value="all">All Variants</Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content value="playground" className="pt-4">
        <Playground />
      </Tabs.Content>

      <Tabs.Content value="all" className="pt-4">
        <AllVariants />
      </Tabs.Content>
    </Tabs>
  );
}
