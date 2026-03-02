import { CheckIcon } from "@/components/ui/icons";

import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

const FREE_FEATURES = [
  "Unlimited image compression, resizing, and conversion",
  "Unlimited CSV cleaning and column renaming",
  "Unlimited batch file renaming",
  "Process multiple files at once",
  "No account required",
  "No watermarks or quality reduction",
  "Browser tools run on your device",
  "Open source engine (MIT)",
];

export function Pricing() {
  return (
    <div className="text-center">
      <Stack gap="md">
        <Heading level={2}>Simple pricing.</Heading>
        <Text color="muted" leading="snug" balance className="mx-auto max-w-xl">
          Every browser tool is free. No limits, no signup, no catch.
        </Text>
      </Stack>

      <Card elevation="md" className="mx-auto mt-12 max-w-md">
        <Card.Content className="flex flex-col gap-6 px-6 py-6">
          <Stack gap="xs">
            <Heading level={3} size="sm">Free</Heading>
            <Text size="sm" color="muted">Everything. No strings attached.</Text>
          </Stack>

          <Stack className="gap-3">
            {FREE_FEATURES.map((feature) => (
              <div key={feature} className="flex items-start gap-2.5">
                <CheckIcon className="text-primary mt-0.5 size-4 shrink-0" />
                <span className="text-sm">{feature}</span>
              </div>
            ))}
          </Stack>

          <Button variant="primary" href="/" elevation="sm" className="w-full">
            Start using bnto
          </Button>
        </Card.Content>
      </Card>

      <Text size="sm" color="muted" className="mx-auto mt-8 max-w-md">
        Premium features with server-side processing are coming soon.
      </Text>
    </div>
  );
}
