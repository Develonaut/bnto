"use client";

import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

/**
 * Conversion hook for anonymous users visiting /workflows.
 * Encourages sign-up to unlock saved workflows, execution history, etc.
 */
export function SignUpPrompt() {
  return (
    <Card elevation="md" className="p-8">
      <Stack className="items-center gap-4 text-center">
        <Heading level={2} size="md">
          Save your workflows
        </Heading>
        <Text color="muted" leading="snug" className="max-w-md">
          Sign in to save your recipes, access execution history, and pick up
          where you left off from any device.
        </Text>
        <div className="flex gap-3 pt-2">
          <Button href="/signin" variant="primary" elevation="sm">
            Sign in
          </Button>
          <Button href="/signin" variant="outline">
            Create account
          </Button>
        </div>
      </Stack>
    </Card>
  );
}
