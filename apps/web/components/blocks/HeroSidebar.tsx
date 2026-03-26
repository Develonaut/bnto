import { Heading, Stack, Text } from "@bnto/ui";
import { GALLERY_HEADING, GALLERY_SUBHEADING, TRUST_LINE } from "@/lib/copy";

import { HeroCTA } from "./HeroCTA";
import { HeroPitchPoints } from "./HeroPitchPoints";

export function HeroSidebar({ showCta = true }: { showCta?: boolean } = {}) {
  return (
    <Stack className="gap-6">
      <Text size="xs" color="muted" mono className="uppercase tracking-wider">
        {TRUST_LINE}
      </Text>
      <Heading level={1} className="text-balance" data-testid="gallery-heading">
        {GALLERY_HEADING}
      </Heading>
      <Text color="muted" leading="snug">
        {GALLERY_SUBHEADING}
      </Text>
      {showCta && <HeroCTA />}
      <HeroPitchPoints />
    </Stack>
  );
}
