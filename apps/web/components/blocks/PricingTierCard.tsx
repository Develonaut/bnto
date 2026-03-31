import { Button, Card, CardContent, CheckIcon, Stack } from "@bnto/ui";

import { PricingTierHeader } from "./PricingTierHeader";

interface PricingTierCardProps {
  name: string;
  price: string;
  subtitle: string;
  features: string[];
  badge?: string;
  ctaLabel: string;
  ctaHref?: string;
  ctaDisabled?: boolean;
  highlight?: boolean;
}

export function FeatureList({ features }: { features: string[] }) {
  return (
    <Stack className="gap-3">
      {features.map((feature) => (
        <div key={feature} className="flex items-start gap-2.5">
          <CheckIcon className="text-primary mt-0.5 size-4 shrink-0" />
          <span className="text-sm text-left">{feature}</span>
        </div>
      ))}
    </Stack>
  );
}

export function PricingTierCard({
  name,
  price,
  subtitle,
  features,
  badge,
  ctaLabel,
  ctaHref,
  ctaDisabled,
  highlight,
}: PricingTierCardProps) {
  return (
    <Card className={highlight ? "ring-2 ring-primary" : undefined}>
      <CardContent className="flex flex-col gap-6 px-6 py-6">
        <PricingTierHeader name={name} price={price} subtitle={subtitle} badge={badge} />
        <FeatureList features={features} />
        <Button
          variant={highlight ? undefined : "outline"}
          elevation={highlight ? "sm" : undefined}
          disabled={ctaDisabled}
          href={ctaHref}
          className="w-full"
        >
          {ctaLabel}
        </Button>
      </CardContent>
    </Card>
  );
}
