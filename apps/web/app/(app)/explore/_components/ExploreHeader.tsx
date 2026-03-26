/**
 * Explore page header — h1 and subtitle.
 *
 * Server component. Static text for SEO crawlers.
 */

import { Heading, Stack, Text } from "@bnto/ui";

export function ExploreHeader() {
  return (
    <Stack className="gap-3">
      <Heading level={1}>Explore Recipes</Heading>
      <Text size="lg" color="muted">
        Browse free browser-based recipes. Pick one, drop your files, get results.
      </Text>
    </Stack>
  );
}
