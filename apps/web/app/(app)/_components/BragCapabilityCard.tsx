import type { ComponentType } from "react";

import {
  Card,
  CheckIcon,
  GlobeIcon,
  IconBadge,
  RepeatIcon,
  Row,
  ShieldCheckIcon,
  SlideUp,
  Stack,
} from "@bnto/ui";

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
    label: "Free forever — browser recipes cost us nothing to run",
  },
];

export function BragCapabilityCard() {
  return (
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
}
