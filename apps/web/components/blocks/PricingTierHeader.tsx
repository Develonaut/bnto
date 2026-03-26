import { Badge, Heading, Stack, Text } from "@bnto/ui";

interface PricingTierHeaderProps {
  name: string;
  price: string;
  subtitle: string;
  badge?: string;
}

export function PricingTierHeader({ name, price, subtitle, badge }: PricingTierHeaderProps) {
  return (
    <Stack gap="xs">
      {badge ? (
        <div className="flex items-center justify-center gap-2">
          <Heading level={3} size="sm">
            {name}
          </Heading>
          <Badge variant="secondary">{badge}</Badge>
        </div>
      ) : (
        <Heading level={3} size="sm">
          {name}
        </Heading>
      )}
      <Text size="lg" weight="bold">
        {price}
      </Text>
      <Text size="sm" color="muted">
        {subtitle}
      </Text>
    </Stack>
  );
}
