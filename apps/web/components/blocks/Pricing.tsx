"use client";

import { Badge, Button, Card, CheckIcon, Heading, Stack, Text } from "@bnto/ui";


const FREE_FEATURES = [
  "Unlimited image compression, resizing, and conversion",
  "Unlimited CSV cleaning and column renaming",
  "Unlimited batch file renaming",
  "Process multiple files at once",
  "No account required",
  "No watermarks or quality reduction",
  "Files never leave your browser",
  "Open source engine (MIT)",
];

const PRO_FEATURES = [
  "Everything in Free",
  "Save recipes to your account",
  "30-day execution history",
  "AI-powered processing",
  "Server-side video and shell nodes",
  "Team sharing (up to 5 members)",
  "Cloud drive export",
  "API access",
];

function FeatureList({ features }: { features: string[] }) {
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

export function Pricing() {
  return (
    <div className="text-center">
      <Stack gap="md">
        <Heading level={2}>Simple pricing.</Heading>
        <Text color="muted" leading="snug" balance className="mx-auto max-w-xl">
          Every browser tool is free, unlimited, forever. Pro adds persistence,
          collaboration, and premium compute.
        </Text>
      </Stack>

      <div className="mx-auto mt-12 grid max-w-3xl grid-cols-1 gap-6 md:grid-cols-2">
        {/* Free tier */}
        <Card>
          <Card.Content className="flex flex-col gap-6 px-6 py-6">
            <Stack gap="xs">
              <Heading level={3} size="sm">Free</Heading>
              <Text size="lg" weight="bold">$0</Text>
              <Text size="sm" color="muted">Forever. No strings attached.</Text>
            </Stack>

            <FeatureList features={FREE_FEATURES} />

            <Button variant="outline" href="/" className="w-full">
              Start using bnto
            </Button>
          </Card.Content>
        </Card>

        {/* Pro tier */}
        <Card className="ring-2 ring-primary">
          <Card.Content className="flex flex-col gap-6 px-6 py-6">
            <Stack gap="xs">
              <div className="flex items-center justify-center gap-2">
                <Heading level={3} size="sm">Pro</Heading>
                <Badge variant="secondary">Coming soon</Badge>
              </div>
              <Text size="lg" weight="bold">TBD</Text>
              <Text size="sm" color="muted">
                Save, collaborate, and run premium nodes.
              </Text>
            </Stack>

            <FeatureList features={PRO_FEATURES} />

            <Button elevation="sm" disabled className="w-full">
              Coming soon
            </Button>
          </Card.Content>
        </Card>
      </div>

      <Stack gap="sm" className="mx-auto mt-12 max-w-xl">
        <Text size="sm" color="muted">
          Browser tools will always be free and unlimited.
          Pro is for users who want to save their work, collaborate with a team,
          or use server-powered features like AI and video processing.
        </Text>
      </Stack>
    </div>
  );
}
