import { Heading, Stack, Text } from "@bnto/ui";

import { PricingTiers } from "./PricingTiers";

function PricingHeader() {
  return (
    <Stack gap="md">
      <Heading level={2}>Simple pricing.</Heading>
      <Text color="muted" leading="snug" balance className="mx-auto max-w-xl">
        Every browser recipe is free, unlimited, forever. Pro adds server-powered processing for
        what browsers can&apos;t do.
      </Text>
    </Stack>
  );
}

export function Pricing() {
  return (
    <div className="text-center">
      <PricingHeader />
      <PricingTiers />
      <Stack gap="sm" className="mx-auto mt-12 max-w-xl">
        <Text size="sm" color="muted">
          Browser recipes will always be free and unlimited. Pro is for recipes that need
          server-side compute — AI inference, video processing, and shell commands.
        </Text>
      </Stack>
    </div>
  );
}
