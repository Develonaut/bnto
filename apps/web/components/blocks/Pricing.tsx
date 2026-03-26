import { Heading, Stack, Text } from "@bnto/ui";

import { PricingTiers } from "./PricingTiers";

function PricingHeader() {
  return (
    <Stack gap="md">
      <Heading level={2}>Simple pricing.</Heading>
      <Text color="muted" leading="snug" balance className="mx-auto max-w-xl">
        Every browser recipe is free, unlimited, forever. Pro adds persistence, collaboration, and
        premium compute.
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
          Browser recipes will always be free and unlimited. Pro is for users who want to save their
          work, collaborate with a team, or use server-powered features like AI and video
          processing.
        </Text>
      </Stack>
    </div>
  );
}
