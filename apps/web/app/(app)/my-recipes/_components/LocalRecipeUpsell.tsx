"use client";

import { Button, CloudIcon, Row, Text } from "@bnto/ui";

/**
 * Upsell banner shown above local recipes for unauthenticated users.
 *
 * Communicates that recipes are device-local and signing in enables
 * cross-device sync — a natural conversion hook.
 */
export function LocalRecipeUpsell() {
  return (
    <Row className="items-center justify-between gap-4 rounded-lg bg-muted px-4 py-3 mb-4">
      <Row className="items-center gap-2.5 min-w-0">
        <CloudIcon className="size-4 shrink-0 text-muted-foreground" />
        <Text size="sm" color="muted" className="min-w-0">
          Your recipes are saved locally on this device. Sign in to sync across devices.
        </Text>
      </Row>
      <Button href="/signin" variant="primary" className="shrink-0">
        Sign in
      </Button>
    </Row>
  );
}
