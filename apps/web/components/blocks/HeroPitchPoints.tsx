import type { ComponentType } from "react";

import {
  BlocksIcon,
  GlobeIcon,
  TerminalIcon,
  ZapIcon,
  IconBadge,
  Row,
  Stack,
  Text,
} from "@bnto/ui";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

interface PitchPoint {
  icon: ComponentType<{ className?: string }>;
  label: string;
}

const PITCH_POINTS: PitchPoint[] = [
  {
    icon: BlocksIcon,
    label: "Pick your ingredients. Each node does one thing. Chain them into anything.",
  },
  { icon: TerminalIcon, label: `${BNTO_REGISTRY.length} house specials. One command: bnto run.` },
  { icon: ZapIcon, label: "Your kitchen, your rules. Everything runs locally." },
  { icon: GlobeIcon, label: "Open kitchen. MIT licensed. Fork it, read it, break it." },
];

export function HeroPitchPoints() {
  return (
    <Stack className="gap-3 pt-4">
      {PITCH_POINTS.map((point) => (
        <Row key={point.label} className="gap-3">
          <IconBadge variant="primary" size="md" shape="square">
            <point.icon className="size-4" />
          </IconBadge>
          <Text size="sm" color="muted">
            {point.label}
          </Text>
        </Row>
      ))}
    </Stack>
  );
}
