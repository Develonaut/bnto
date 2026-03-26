import { Card, Heading, Stack, Text } from "@bnto/ui";
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";

export function HeroStatCard() {
  return (
    <Card color="accent" className="flex h-full flex-col justify-between gap-4 p-6">
      <Text size="sm" mono className="text-accent-foreground/80 uppercase tracking-wider">
        Free forever
      </Text>
      <Stack className="gap-1">
        <Heading
          level={3}
          as="p"
          className="font-display text-4xl font-bold text-accent-foreground"
        >
          {BNTO_REGISTRY.length}
        </Heading>
        <Text size="sm" className="text-accent-foreground/80">
          recipes — no limits, no signup
        </Text>
      </Stack>
    </Card>
  );
}
