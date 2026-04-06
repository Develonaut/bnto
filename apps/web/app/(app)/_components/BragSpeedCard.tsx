import { Card, ComparisonBar, Stack, Text } from "@bnto/ui";

import { SpeedCounter } from "./SpeedCounter";

export function BragSpeedCard() {
  return (
    <Card className="p-5">
      <Stack className="gap-3">
        <SpeedCounter />
        <ComparisonBar
          active
          height="h-2.5"
          items={[
            { label: "bnto", value: 50, subtitle: "Local processing" },
            { label: "Cloud upload", value: 8000, subtitle: "Upload, process, download" },
          ]}
        />
        <Text size="xs" color="muted">
          Avg processing time, local vs cloud round-trip
        </Text>
      </Stack>
    </Card>
  );
}
