"use client";

import { useCallback, useState } from "react";

/**
 * useDialog — standardized open/close state for dialogs.
 *
 * Returns { open, onOpenChange, openDialog, closeDialog } — spread
 * `open` and `onOpenChange` onto the Dialog, use openDialog/closeDialog
 * from triggers.
 */

interface UseDialogReturn {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  openDialog: () => void;
  closeDialog: () => void;
}

function useDialog(initial = false): UseDialogReturn {
  const [open, setOpen] = useState(initial);

  const openDialog = useCallback(() => setOpen(true), []);
  const closeDialog = useCallback(() => setOpen(false), []);

  return { open, onOpenChange: setOpen, openDialog, closeDialog };
}

export { useDialog };
export type { UseDialogReturn };
