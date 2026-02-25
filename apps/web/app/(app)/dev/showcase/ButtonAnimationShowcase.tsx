"use client";

import type { SpringMode } from "@/components/ui/button";
import { Button } from "@/components/ui/button";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";

/* ── Spring Prop Demo ────────────────────────────────────────
 *
 * Demonstrates the three spring modes available via the
 * Button `spring` prop. Default is "lg" — every button
 * in the app gets the rubber-band release feel unless
 * explicitly overridden.
 * ──────────────────────────────────────────────────────────── */

const MODES: { mode: SpringMode; description: string }[] = [
  { mode: "sm", description: "150ms ease-out (firm)" },
  { mode: "md", description: "400ms spring-bouncier" },
  { mode: "lg", description: "550ms pressable spring (default)" },
];

export function ButtonAnimationShowcase() {
  return (
    <div className="flex flex-wrap gap-8">
      {MODES.map(({ mode, description }) => (
        <Stack key={mode} gap="xs" className="items-center">
          <Button variant="default" size="lg" spring={mode}>
            spring=&quot;{mode}&quot;
          </Button>
          <Text size="xs" color="muted" mono className="text-center max-w-32">
            {description}
          </Text>
        </Stack>
      ))}
    </div>
  );
}
