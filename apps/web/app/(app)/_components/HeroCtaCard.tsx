import { Card, Heading, IconBadge, PlusIcon, Stack, Text } from "@bnto/ui";
import Link from "next/link";

export function HeroCtaCard() {
  return (
    <Card
      asChild
      color="primary"
      className="pressable flex h-full flex-col justify-between gap-4 p-6"
    >
      <Link href="/editor">
        <IconBadge variant="onSurface" size="lg" shape="square">
          <PlusIcon className="size-5" />
        </IconBadge>
        <Stack className="gap-1">
          <Heading level={3} as="p" size="xs" className="text-primary-foreground">
            Create your own
          </Heading>
          <Text size="sm" className="text-primary-foreground/80">
            Compose multi-step recipes with the visual editor.
          </Text>
        </Stack>
      </Link>
    </Card>
  );
}
