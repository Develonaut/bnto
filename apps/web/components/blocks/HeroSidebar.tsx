import {
  GithubIcon,
  GlobeIcon,
  LaptopIcon,
  ShieldCheckIcon,
  ZapIcon,
} from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Heading } from "@/components/ui/Heading";
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
  { icon: ShieldCheckIcon, label: "100% in your browser. Files never leave your device" },
  { icon: LaptopIcon, label: "Batch processing. Drop multiple files at once" },
  { icon: GlobeIcon, label: "Open source (MIT). Inspect every line" },
];

/* ── Hero sidebar ────────────────────────────────────────────── */

export function HeroSidebar({ showCta = true }: { showCta?: boolean } = {}) {
  return (
    <div className="flex flex-col gap-6">
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
        <div className="flex items-center gap-3 pt-2">
          <Button variant="primary" href="#faq" depth="sm">
            Learn more
          </Button>
          <Button
            variant="outline"
            href={GITHUB_URL}
            target="_blank"
            rel="noopener noreferrer"
            depth="sm"
          >
            <GithubIcon />
            GitHub
          </Button>
        </div>
      )}

      {/* Pitch points */}
      <div className="flex flex-col gap-3 pt-4">
        {PITCH_POINTS.map((point) => (
          <div key={point.label} className="flex items-center gap-3">
            <div className="text-primary flex size-8 items-center justify-center rounded-lg bg-primary/10">
              <point.icon className="size-4" />
            </div>
            <Text size="sm" color="muted">{point.label}</Text>
          </div>
        ))}
      </div>
    </div>
  );
}
