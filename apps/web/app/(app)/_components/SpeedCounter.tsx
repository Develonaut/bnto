import { AnimatedCounter, IconBadge, Row, ZapIcon } from "@bnto/ui";

export function SpeedCounter() {
  return (
    <Row className="gap-3">
      <IconBadge variant="primary" size="md" shape="square">
        <ZapIcon className="size-4" />
      </IconBadge>
      <Row className="gap-1.5" align="baseline">
        <AnimatedCounter
          value={50}
          active
          className="font-display text-2xl font-bold tracking-tight"
        />
        <span className="text-muted-foreground text-sm font-medium">ms</span>
      </Row>
    </Row>
  );
}
