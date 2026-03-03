"use client";

import type { ReactNode } from "react";
import { useState } from "react";
import { core } from "@bnto/core";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Dialog } from "@/components/ui/Dialog";
import { Heading } from "@/components/ui/Heading";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

/* ── Shared props ─────────────────────────────────────────────── */

interface AuthGateBaseProps {
  children: ReactNode;
  /** Prompt heading shown to unauthenticated users. */
  title?: string;
  /** Prompt body — explain why signing up is worth it. */
  description?: string;
}

/* ── Shared CTA buttons ───────────────────────────────────────── */

function AuthGateCTA() {
  return (
    <Row className="gap-3 pt-2">
      <Button href="/signin" variant="primary" elevation="sm">
        Sign up free
      </Button>
      <Button href="/signin" variant="outline">
        Sign in
      </Button>
    </Row>
  );
}

/* ── Shared auth check ────────────────────────────────────────── */

function useAuthGate() {
  const { isAuthenticated, isLoading } = core.auth.useAuth();
  return { isGated: !isLoading && !isAuthenticated, isLoading };
}

/* ── AuthGate.Action ──────────────────────────────────────────── */

/**
 * Wraps an interactive element. Authenticated users click through normally.
 * Unauthenticated users see a dismissible conversion Dialog.
 *
 * The "carrot-on-stick" pattern: features are visible to everyone,
 * but gated interactions convert at the moment of intent.
 */
function AuthGateAction({
  children,
  title = "Sign up to continue",
  description = "Create a free account to unlock this feature.",
}: AuthGateBaseProps) {
  const { isGated, isLoading } = useAuthGate();
  const [open, setOpen] = useState(false);

  if (isLoading || !isGated) {
    return <>{children}</>;
  }

  return (
    <>
      <span
        role="button"
        tabIndex={0}
        onClick={(e) => {
          e.preventDefault();
          e.stopPropagation();
          setOpen(true);
        }}
        onKeyDown={(e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            e.stopPropagation();
            setOpen(true);
          }
        }}
        className="inline-flex cursor-pointer"
      >
        {children}
      </span>

      <Dialog open={open} onOpenChange={setOpen}>
        <Dialog.Content>
          <Dialog.Header>
            <Dialog.Title>{title}</Dialog.Title>
            <Dialog.Close />
          </Dialog.Header>
          <Dialog.Body>
            <Text color="muted">{description}</Text>
          </Dialog.Body>
          <Dialog.Footer>
            <Dialog.Close asChild>
              <Button variant="ghost">Cancel</Button>
            </Dialog.Close>
            <AuthGateCTA />
          </Dialog.Footer>
        </Dialog.Content>
      </Dialog>
    </>
  );
}

/* ── AuthGate.Section ─────────────────────────────────────────── */

/**
 * Wraps a whole section. Authenticated users see content normally.
 * Unauthenticated users see blurred, non-interactive content with a
 * floating sign-up card — "peek behind the curtain."
 *
 * The "velvet rope" pattern: show the structure but gate interaction.
 */
function AuthGateSection({
  children,
  title = "Sign in to get started",
  description = "Create a free account to save your recipes, track execution history, and pick up where you left off.",
}: AuthGateBaseProps) {
  const { isGated, isLoading } = useAuthGate();

  if (isLoading || !isGated) {
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
              <AuthGateCTA />
            </Stack>
          </Card>
        </Animate.ScaleIn>
      </div>
    </div>
  );
}

/* ── Compound export ──────────────────────────────────────────── */

export const AuthGate = Object.assign(
  /** @deprecated Use AuthGate.Action or AuthGate.Section explicitly. */
  AuthGateAction,
  {
    Action: AuthGateAction,
    Section: AuthGateSection,
  },
);
