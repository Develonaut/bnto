import { Card, Heading, IconBadge, ShieldCheckIcon, Stack, Text } from "@bnto/ui";

export function HeroPrivacyCard() {
  return (
    <Card color="secondary" className="flex h-full flex-col justify-between gap-4 p-6">
      <IconBadge variant="onSurface" size="lg" shape="square">
        <ShieldCheckIcon className="size-5" />
      </IconBadge>
      <Stack className="gap-1">
        <Heading level={3} as="p" size="xs" className="text-secondary-foreground">
          100% private
        </Heading>
        <Text size="sm" className="text-secondary-foreground/80">
          Runs in your browser. Files never leave your device.
        </Text>
      </Stack>
    </Card>
  );
}
