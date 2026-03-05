import { Heading, Stack, Text } from "@bnto/ui";

export function TypographyShowcase() {
  return (
    <Stack gap="md">
      <Heading level={1} size="xl">Display Heading (Geist)</Heading>
      <Heading level={2}>Page Heading (Geist)</Heading>
      <Heading level={3}>Section Heading (Geist)</Heading>
      <Text>
        Body text in Inter. Clean and highly legible at all sizes. The warm
        terracotta palette carries through every element.
      </Text>
      <Text size="sm" color="muted">
        Secondary text in Inter for descriptions, labels, and metadata.
      </Text>
      <Text size="xs" color="muted" mono className="uppercase tracking-wider">
        Geist Mono (technical output)
      </Text>
    </Stack>
  );
}
