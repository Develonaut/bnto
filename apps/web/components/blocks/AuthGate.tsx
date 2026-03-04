"use client";

import type { ReactNode } from "react";
import { core } from "@bnto/core";

import { useControlled } from "@/hooks/useControlled";

import { Button } from "@/components/ui/Button";
import { Dialog } from "@/components/ui/Dialog";
import { Menu } from "@/components/ui/Menu";
import { Popover } from "@/components/ui/Popover";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

/* ── Shared props ─────────────────────────────────────────────── */

interface AuthGateActionProps {
  children: ReactNode;
  /** Prompt heading shown to unauthenticated users. */
  title?: string;
  /** Prompt body — explain why signing up is worth it. */
  description?: string;
  /**
   * Prompt style: "menu" anchors a springy Card near the trigger,
   * "dialog" opens a full overlay. Menu is lighter weight.
   */
  variant?: "menu" | "dialog";
  /** Controlled open state — parent can programmatically open the prompt. */
  open?: boolean;
  /** Called when the prompt's open state changes (dismiss, close, etc.). */
  onOpenChange?: (open: boolean) => void;
}

/* ── Shared CTA ───────────────────────────────────────────────── */

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

/** CTA for menu prompt. */
function AuthGateMenuCTA() {
  return (
    <Row className="gap-2 justify-center">
      <Button
        href="/signin"
        variant="primary"
        elevation="sm"
        className="h-8 px-4 text-sm"
      >
        Sign up free
      </Button>
      <Button href="/signin" variant="ghost" className="h-8 px-4 text-sm">
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
 * Unauthenticated users see a conversion prompt.
 *
 * Two variants:
 * - "menu" (default): springy Card anchored near the trigger
 * - "dialog": full overlay dialog for heavier prompts
 */
function AuthGateAction({
  children,
  title = "Sign up to continue",
  description = "Create a free account to unlock this feature.",
  variant = "menu",
  open: controlledOpen,
  onOpenChange,
}: AuthGateActionProps) {
  const { isGated } = useAuthGate();
  const [open, setOpen] = useControlled(controlledOpen, false, onOpenChange);

  if (!isGated) {
    return <>{children}</>;
  }

  if (variant === "dialog") {
    return (
      <>
        <span
          role="button"
          tabIndex={0}
          onClickCapture={(e) => {
            e.preventDefault();
            e.stopPropagation();
            setOpen(true);
          }}
          onKeyDownCapture={(e) => {
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
              <AuthGateCTA />
            </Dialog.Footer>
          </Dialog.Content>
        </Dialog>
      </>
    );
  }

  // Menu mode (default) — anchored Card with spring animation
  return (
    <Menu open={open} onOpenChange={setOpen}>
      <Popover.Anchor asChild>
        <span
          role="button"
          tabIndex={0}
          onClickCapture={(e) => {
            e.preventDefault();
            e.stopPropagation();
            setOpen(true);
          }}
          onKeyDownCapture={(e) => {
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
      </Popover.Anchor>
      <Menu.Content side="top" align="center" className="w-72 p-5">
        <Stack className="gap-4">
          <Stack className="gap-1.5">
            <Text size="base" weight="medium">
              {title}
            </Text>
            <Text size="sm" color="muted">
              {description}
            </Text>
          </Stack>
          <AuthGateMenuCTA />
        </Stack>
      </Menu.Content>
    </Menu>
  );
}

/* ── Compound export ──────────────────────────────────────────── */

export const AuthGate = Object.assign(AuthGateAction, {
  Action: AuthGateAction,
});
