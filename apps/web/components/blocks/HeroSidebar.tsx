import {
  GithubIcon,
  GlobeIcon,
  LaptopIcon,
  ShieldCheckIcon,
  ZapIcon,
} from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Heading } from "@/components/ui/Heading";
import { IconBadge } from "@/components/ui/IconBadge";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";
import {
  GALLERY_HEADING,
  GALLERY_SUBHEADING,
  GITHUB_URL,
  TRUST_LINE,
} from "@/lib/copy";

/* ── Pitch points ────────────────────────────────────────────── */

const PITCH_POINTS = [
  { icon: ZapIcon, label: `${BNTO_REGISTRY.length} free tools, no limits, no signup` },
  { icon: ShieldCheckIcon, label: "Runs in your browser. Files stay on your device" },
  { icon: LaptopIcon, label: "Batch processing. Drop multiple files at once" },
  { icon: GlobeIcon, label: "Open source (MIT). Inspect every line" },
];

/* ── Hero sidebar ────────────────────────────────────────────── */

export function HeroSidebar({ showCta = true }: { showCta?: boolean } = {}) {
  return (
    <Stack className="gap-6">
      <Text size="xs" color="muted" mono className="uppercase tracking-wider">
        {TRUST_LINE}
      </Text>
      <Heading level={1} className="text-balance">
        {GALLERY_HEADING}
      </Heading>
      <Text color="muted" leading="snug">
        {GALLERY_SUBHEADING}
      </Text>

      {/* CTA */}
      {showCta && (
        <Row className="gap-3 pt-2">
          <Button variant="primary" href="#faq" elevation="sm">
            Learn more
          </Button>
          <Button
            variant="outline"
            href={GITHUB_URL}
            target="_blank"
            rel="noopener noreferrer"
            elevation="sm"
          >
            <GithubIcon />
            GitHub
          </Button>
        </Row>
      )}

      {/* Pitch points */}
      <Stack className="gap-3 pt-4">
        {PITCH_POINTS.map((point) => (
          <Row key={point.label} className="gap-3">
            <IconBadge variant="primary" size="md" shape="square">
              <point.icon className="size-4" />
            </IconBadge>
            <Text size="sm" color="muted">{point.label}</Text>
          </Row>
        ))}
      </Stack>
    </Stack>
  );
}
