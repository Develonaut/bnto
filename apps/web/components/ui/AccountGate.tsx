"use client";

import type { ReactNode } from "react";

import { core } from "@bnto/core";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

interface AccountGateProps {
  /** Content rendered underneath — always visible, blurred when gated. */
  children: ReactNode;
  /** Heading shown in the floating prompt. */
  title?: string;
  /** Description shown in the floating prompt. */
  description?: string;
}

/**
 * Conversion gate for users without an account.
 *
 * Renders children normally for users with a real account. For anonymous
 * visitors (including Convex anonymous sessions), the content renders
 * blurred and non-interactive with a floating sign-up card centered on
 * top — "peek behind the curtain."
 *
 * Use on any page where you want to show the structure but gate
 * interaction behind having an account.
 */
export function AccountGate({
  children,
  title = "Sign in to get started",
  description = "Create a free account to save your recipes, track execution history, and pick up where you left off.",
}: AccountGateProps) {
  const { isAuthenticated, isLoading, user } = core.auth.useAuth();

  // Gate on a real account (has email), not just an anonymous session.
  // Convex auto-creates anonymous sessions, so isAuthenticated is true
  // even for users who haven't signed up.
  const hasAccount = isAuthenticated && !!user?.email;

  if (isLoading || hasAccount) {
    return <>{children}</>;
  }

  return (
    <div className="relative">
      {/* Blurred, non-interactive content underneath */}
      <div
        aria-hidden="true"
        className="pointer-events-none select-none blur-[6px] opacity-60"
      >
        {children}
      </div>

      {/* Floating sign-up prompt */}
      <div className="absolute inset-0 z-10 flex items-start justify-center pt-16">
        <Animate.ScaleIn from={0.6} easing="spring-bouncier">
          <Card elevation="lg" className="w-full max-w-md p-8">
            <Stack className="items-center gap-4 text-center">
              <Heading level={2} size="md">
                {title}
              </Heading>
              <Text color="muted" leading="snug" className="max-w-sm">
                {description}
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
        </Animate.ScaleIn>
      </div>
    </div>
  );
}
