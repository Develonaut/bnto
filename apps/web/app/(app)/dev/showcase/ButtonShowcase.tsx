"use client";

import { useState } from "react";
import { ArrowRight, Download, Zap } from "lucide-react";

import type { SpringMode } from "@/components/ui/button";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";
import { Row } from "@/components/ui/Row";

/* ── Option types ────────────────────────────────────────── */

type Variant = "default" | "secondary" | "outline" | "muted" | "destructive" | "success" | "warning";
type Size = "sm" | "default" | "lg";

const VARIANTS: { value: Variant; label: string }[] = [
  { value: "default", label: "Default" },
  { value: "secondary", label: "Secondary" },
  { value: "outline", label: "Outline" },
  { value: "muted", label: "Muted" },
  { value: "destructive", label: "Destructive" },
  { value: "success", label: "Success" },
  { value: "warning", label: "Warning" },
];

const SIZES: { value: Size; label: string }[] = [
  { value: "sm", label: "sm" },
  { value: "default", label: "default" },
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
            variant={opt.value === value ? "default" : "outline"}
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

/* ── Root ────────────────────────────────────────────────── */

export function ButtonShowcase() {
  const [variant, setVariant] = useState<Variant>("default");
  const [size, setSize] = useState<Size>("default");
  const [spring, setSpring] = useState<SpringMode>("lg");

  return (
    <Stack gap="lg">
      {/* Controls */}
      <Stack gap="sm">
        <ToggleGroup label="variant" options={VARIANTS} value={variant} onChange={setVariant} />
        <ToggleGroup label="size" options={SIZES} value={size} onChange={setSize} />
        <ToggleGroup label="spring" options={SPRINGS} value={spring} onChange={setSpring} />
      </Stack>

      {/* Specimens */}
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
