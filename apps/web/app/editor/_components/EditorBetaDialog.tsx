"use client";

import { useCallback } from "react";
import { Dialog } from "@bnto/ui";
import { useDismissible } from "@/lib/useDismissible";
import { EditorBetaDialogContent } from "./EditorBetaDialogContent";

/**
 * One-time beta dialog — greets users before they start editing.
 *
 * Persists dismissal in localStorage so it only shows once per browser.
 */
export function EditorBetaDialog() {
  const { dismissed, dismiss } = useDismissible("bnto-editor-experimental-dismissed");

  const handleOpenChange = useCallback(
    (v: boolean) => {
      if (!v) dismiss();
    },
    [dismiss],
  );

  return (
    <Dialog open={!dismissed} onOpenChange={handleOpenChange}>
      <EditorBetaDialogContent onDismiss={dismiss} />
    </Dialog>
  );
}
