import { Heading } from "#components/ui/Heading";
import { Text } from "#components/ui/Text";
import { Stack } from "#components/ui/Stack";

export function TypographyShowcase() {
  return (
    <Stack gap="md">
      <Heading level={1} size="xl">Display Heading</Heading>
      <Heading level={2}>Page Heading</Heading>
      <Heading level={3}>Section Heading</Heading>
      <Text>
        Body text in Inter. Clean and highly legible at all sizes. The warm
        terracotta palette carries through every element.
      </Text>
      <Text size="sm" color="muted">
        Secondary text for descriptions, labels, and metadata.
      </Text>
      <Text size="xs" color="muted" mono className="uppercase tracking-wider">
        Monospace technical output
      </Text>
    </Stack>
  );
}
