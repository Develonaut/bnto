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
  { icon: BlocksIcon, label: "Composable nodes. Build any workflow by chaining nodes together" },
  { icon: TerminalIcon, label: `${BNTO_REGISTRY.length} recipes included. One command: bnto run` },
  { icon: ZapIcon, label: "Runs on your machine. Files never leave your device" },
  { icon: GlobeIcon, label: "Open source (MIT). Extend it, fork it, contribute" },
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
