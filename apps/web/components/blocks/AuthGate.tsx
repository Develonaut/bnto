"use client";

import type { ReactNode } from "react";
import { core } from "@bnto/core";

import { useControlled } from "@/hooks/useControlled";

import { AuthGateDialogPrompt } from "./AuthGateDialogPrompt";
import { AuthGateMenuPrompt } from "./AuthGateMenuPrompt";
import { useGateHandlers } from "./useGateHandlers";

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

/* ── AuthGate.Action ──────────────────────────────────────────── */

/**
 * Wraps an interactive element. Authenticated users click through normally.
 * Unauthenticated users see a conversion prompt.
 */
function AuthGateAction({
  children,
  title = "Sign up to continue",
  description = "Create a free account to unlock this feature.",
  variant = "menu",
  open: controlledOpen,
  onOpenChange,
}: AuthGateActionProps) {
  const { isAuthenticated, isLoading } = core.auth.useAuth();
  const isGated = !isLoading && !isAuthenticated;
  const [open, setOpen] = useControlled(controlledOpen, false, onOpenChange);
  const { handleGateClick, handleGateKeyDown } = useGateHandlers(setOpen);

  if (!isGated) return <>{children}</>;

  const Prompt = variant === "dialog" ? AuthGateDialogPrompt : AuthGateMenuPrompt;

  return (
    <Prompt
      open={open}
      setOpen={setOpen}
      handleGateClick={handleGateClick}
      handleGateKeyDown={handleGateKeyDown}
      title={title}
      description={description}
    >
      {children}
    </Prompt>
  );
}

export { AuthGateAction as AuthGate };
