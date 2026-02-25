"use client";

import type { SpringMode } from "@/components/ui/button";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";

const MODES: { mode: SpringMode; label: string; description: string }[] = [
  { mode: "sm", label: "Firm", description: "150ms ease-out — direct, no overshoot" },
  { mode: "md", label: "Bounce", description: "400ms spring — gentle single settle" },
  { mode: "lg", label: "Elastic", description: "550ms spring — rubber-band, 3 oscillations (default)" },
];

export function ButtonAnimationShowcase() {
  return (
    <div className="flex flex-wrap gap-8">
      {MODES.map(({ mode, label, description }) => (
        <Stack key={mode} gap="xs" className="items-center">
          <Button variant="default" size="lg" spring={mode}>
            {label}
          </Button>
          <Text size="xs" color="muted" mono className="text-center max-w-36">
            spring=&quot;{mode}&quot;
          </Text>
          <Text size="xs" color="muted" className="text-center max-w-44">
            {description}
          </Text>
        </Stack>
      ))}
    </div>
  );
}
