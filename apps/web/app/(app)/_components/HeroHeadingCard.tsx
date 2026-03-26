import { Card, Heading, Text } from "@bnto/ui";
import { GALLERY_HEADING, GALLERY_SUBHEADING, TRUST_LINE } from "@/lib/copy";

export function HeroHeadingCard() {
  return (
    <Card className="flex h-full flex-col justify-center gap-4 p-8">
      <Text size="xs" color="muted" mono className="uppercase tracking-wider">
        {TRUST_LINE}
      </Text>
      <Heading level={1} className="text-balance" data-testid="gallery-heading">
        {GALLERY_HEADING}
      </Heading>
      <Text color="muted" leading="snug">
        {GALLERY_SUBHEADING}
      </Text>
    </Card>
  );
}
