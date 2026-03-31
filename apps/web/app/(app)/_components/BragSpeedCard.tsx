import {
  AnimatedCounter,
  Card,
  ComparisonBar,
  IconBadge,
  Row,
  Stack,
  Text,
  ZapIcon,
} from "@bnto/ui";

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

export function BragSpeedCard() {
  return (
    <Card className="p-5">
      <Stack className="gap-3">
        <SpeedCounter />
        <ComparisonBar
          active
          height="h-2.5"
          items={[
            { label: "bnto", value: 50, subtitle: "Local WASM" },
            { label: "Cloud upload", value: 8000, subtitle: "Upload, process, download" },
          ]}
        />
        <Text size="xs" color="muted">
          Avg processing time — local vs cloud round-trip
        </Text>
      </Stack>
    </Card>
  );
}
