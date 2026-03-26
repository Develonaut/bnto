"use client";

import { useCallback, useState } from "react";

import { Dialog } from "@bnto/ui";

import { EditorBetaDialogContent } from "./EditorBetaDialogContent";

const STORAGE_KEY = "bnto-editor-beta-dismissed";

function wasDismissed(): boolean {
  if (typeof window === "undefined") return true;
  return localStorage.getItem(STORAGE_KEY) === "true";
}

/**
 * One-time beta dialog — greets users before they start editing.
 *
 * Persists dismissal in localStorage so it only shows once per browser.
 */
export function EditorBetaDialog() {
  const [open, setOpen] = useState(() => !wasDismissed());

  const handleDismiss = useCallback(() => {
    localStorage.setItem(STORAGE_KEY, "true");
    setOpen(false);
  }, []);

  const handleOpenChange = useCallback(
    (v: boolean) => {
      if (!v) handleDismiss();
    },
    [handleDismiss],
  );

  return (
    <Dialog open={open} onOpenChange={handleOpenChange}>
      <EditorBetaDialogContent onDismiss={handleDismiss} />
    </Dialog>
  );
}
