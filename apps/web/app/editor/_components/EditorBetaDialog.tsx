"use client";

import { useState } from "react";

import {
  Button,
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@bnto/ui";

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

  function handleDismiss() {
    localStorage.setItem(STORAGE_KEY, "true");
    setOpen(false);
  }

  return (
    <Dialog open={open} onOpenChange={(v) => !v && handleDismiss()}>
      <DialogContent data-testid="editor-beta-dialog" size="sm">
        <DialogHeader>
          <DialogTitle>Welcome to the Recipe Editor</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogDescription>
          The recipe editor is in beta. We&apos;re actively experimenting and
          testing functionality here, so things may change as we refine the
          experience.
        </DialogDescription>
        <DialogFooter>
          <DialogClose asChild>
            <Button onClick={handleDismiss}>Get started</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
