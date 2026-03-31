import type { ComponentType } from "react";

import {
  BlocksIcon,
  GlobeIcon,
  ShieldCheckIcon,
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
  { icon: ZapIcon, label: `${BNTO_REGISTRY.length} free recipes, no limits, no signup` },
  { icon: ShieldCheckIcon, label: "Runs in your browser. Files never leave your device" },
  { icon: BlocksIcon, label: "Build custom recipes with the visual editor" },
  { icon: GlobeIcon, label: "Open source (MIT). Inspect every line" },
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
