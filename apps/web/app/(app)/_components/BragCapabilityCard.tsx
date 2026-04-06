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
    label: "Your kitchen, your rules. Files never leave your machine.",
  },
  {
    icon: RepeatIcon,
    variant: "accent",
    label: "Portable .bnto.json recipes. Pack once, run anywhere.",
  },
  { icon: GlobeIcon, variant: "success", label: "One node, one job. Chain them into anything." },
  {
    icon: CheckIcon,
    variant: "primary",
    label: "Open kitchen. MIT licensed. Always free.",
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
