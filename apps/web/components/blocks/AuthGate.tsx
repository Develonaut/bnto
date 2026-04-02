import type { ReactNode } from "react";

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
 * Auth gates disabled — open-source-first, no conversion prompts.
 * Keeping the component so consumers don't need to change their imports.
 * Re-enable when premium features (visual editor, cloud execution) justify auth.
 */
export function AuthGateAction({ children }: AuthGateActionProps) {
  return <>{children}</>;
}

export { AuthGateAction as AuthGate };
