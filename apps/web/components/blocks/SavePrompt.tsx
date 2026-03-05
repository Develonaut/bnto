"use client";

import { core } from "@bnto/core";

import { SlideUp, Button, Card, Row, Stack, Text } from "@bnto/ui";

/**
 * Post-execution conversion prompt for unauthenticated users.
 *
 * Surfaces inline after a successful browser execution — a natural
 * value moment. Non-blocking: the user already has their results.
 * The prompt nudges them to sign up to save and revisit later.
 *
 * Renders nothing for authenticated users or when not triggered.
 */
export function SavePrompt({ show }: { show: boolean }) {
  const { isAuthenticated, isLoading } = core.auth.useAuth();

  if (isLoading || isAuthenticated || !show) return null;

  return (
    <SlideUp>
      <Card elevation="sm" className="px-5 py-4">
        <Row justify="between" align="center" wrap className="gap-3">
          <Stack className="min-w-0 gap-0.5">
            <Text size="sm" weight="medium">
              Want to save this recipe?
            </Text>
            <Text size="xs" color="muted">
              Sign up to keep your history and pick up where you left off.
            </Text>
          </Stack>
          <Button href="/signin" variant="primary" elevation="sm" className="h-8 px-3 text-sm">
            Sign up free
          </Button>
        </Row>
      </Card>
    </SlideUp>
  );
}
