import { Heading, Stack, Text } from "@bnto/ui";
import { t } from "@bnto/i18n";

import { HeroCTA } from "./HeroCTA";
import { HeroPitchPoints } from "./HeroPitchPoints";

export function HeroSidebar({ showCta = true }: { showCta?: boolean } = {}) {
  return (
    <Stack className="gap-6">
      <Text size="xs" color="muted" mono className="uppercase tracking-wider">
        {t("site.trustLine")}
      </Text>
      <Heading level={1} className="text-balance" data-testid="gallery-heading">
        {t("hero.heading")}
      </Heading>
      <Text color="muted" leading="snug">
        {t("hero.subheading")}
      </Text>
      {showCta && <HeroCTA />}
      <HeroPitchPoints />
    </Stack>
  );
}
