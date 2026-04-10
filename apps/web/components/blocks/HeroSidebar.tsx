import { Heading, Stack, Text } from "@bnto/ui";
import { t } from "@bnto/i18n";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

import { HeroPitchPoints } from "./HeroPitchPoints";

const recipeCount = BNTO_REGISTRY.length;

export function HeroSidebar() {
  return (
    <Stack className="gap-6">
      <Text size="xs" color="muted" mono className="uppercase tracking-wider">
        {t("site.trustLine")}
      </Text>
      <Heading level={1} className="text-balance" data-testid="gallery-heading">
        {t("hero.heading")}
      </Heading>
      <Text color="muted" leading="snug">
        {t("hero.subheading", { count: recipeCount })}
      </Text>
      <HeroPitchPoints />
    </Stack>
  );
}
