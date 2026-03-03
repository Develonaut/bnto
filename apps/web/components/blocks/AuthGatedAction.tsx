"use client";

import type { ReactNode } from "react";
import { useState } from "react";
import { core } from "@bnto/core";

import { Button } from "@/components/ui/Button";
import { Dialog } from "@/components/ui/Dialog";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

interface AuthGatedActionProps {
  children: ReactNode;
  /** Dialog heading shown to unauthenticated users. */
  title?: string;
  /** Dialog body text — explain why signing up is worth it. */
  description?: string;
}

/**
 * Wraps an interactive element so unauthenticated users see a conversion
 * dialog instead of the action firing. Authenticated users pass through
 * with no wrapper.
 *
 * The "carrot-on-stick" pattern: features are visible to everyone,
 * but gated interactions convert unauthenticated users at the moment
 * of intent — when they've already seen the value.
 */
export function AuthGatedAction({
  children,
  title = "Sign up to continue",
  description = "Create a free account to unlock this feature.",
}: AuthGatedActionProps) {
  const { isAuthenticated, isLoading } = core.auth.useAuth();
  const [open, setOpen] = useState(false);

  if (isLoading || isAuthenticated) {
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
            <Stack className="gap-2 sm:flex-row">
              <Button href="/signin" variant="primary">
                Sign up free
              </Button>
            </Stack>
          </Dialog.Footer>
        </Dialog.Content>
      </Dialog>
    </>
  );
}
