import type { ComponentType } from "react";

import {
  AnimatedCounter,
  Card,
  CheckIcon,
  ComparisonBar,
  GlobeIcon,
  IconBadge,
  RepeatIcon,
  Row,
  ScaleIn,
  ShieldCheckIcon,
  SlideUp,
  Stack,
  Stagger,
  Text,
  ZapIcon,
} from "@bnto/ui";

/* ── Data ────────────────────────────────────────────────────── */

interface Capability {
  icon: ComponentType<{ className?: string }>;
  variant: "primary" | "secondary" | "accent" | "success";
  label: string;
}

const CAPABILITIES: Capability[] = [
  {
    icon: ShieldCheckIcon,
    variant: "secondary",
    label: "Runs locally — files never leave your device",
  },
  { icon: RepeatIcon, variant: "accent", label: "No daily limits — process unlimited files" },
  { icon: GlobeIcon, variant: "success", label: "No signup required — just drop files and go" },
  {
    icon: CheckIcon,
    variant: "primary",
    label: "Free forever — browser tools cost us nothing to run",
  },
];

/* ── Brag layout ─────────────────────────────────────────────── */

export function BragLayout() {
  const speedCard = (
    <Card className="p-5">
      <Stack className="gap-3">
        <Row className="gap-3">
          <IconBadge variant="primary" size="md" shape="square">
            <ZapIcon className="size-4" />
          </IconBadge>
          <Row className="gap-1.5" align="baseline">
            <AnimatedCounter
              value={50}
              active
              className="font-display text-2xl font-bold tracking-tight"
            />
            <span className="text-muted-foreground text-sm font-medium">ms</span>
          </Row>
        </Row>
        <ComparisonBar
          active
          height="h-2.5"
          items={[
            { label: "bnto", value: 50, subtitle: "Local WASM" },
            { label: "Cloud upload", value: 8000, subtitle: "Upload, process, download" },
          ]}
        />
        <Text size="xs" color="muted">
          Avg processing time — local vs cloud round-trip
        </Text>
      </Stack>
    </Card>
  );

  const capabilityCard = (
    <Card className="p-5">
      <Stack className="gap-3">
        {CAPABILITIES.map((cap, i) => (
          <SlideUp key={cap.label} index={i} distance={8} easing="spring-bouncy">
            <Row className="gap-3">
              <IconBadge variant={cap.variant} size="sm">
                <cap.icon className="size-3.5" />
              </IconBadge>
              <span className="text-sm">{cap.label}</span>
            </Row>
          </SlideUp>
        ))}
      </Stack>
    </Card>
  );

  return (
    <Stagger asChild>
      <Stack className="gap-3">
        <ScaleIn index={0} from={0.9} easing="spring-bouncy">
          {speedCard}
        </ScaleIn>
        <ScaleIn index={1} from={0.9} easing="spring-bouncy">
          {capabilityCard}
        </ScaleIn>
      </Stack>
    </Stagger>
  );
}
