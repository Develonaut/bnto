"use client";

import {
  Button,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@bnto/ui";

interface EditorBetaDialogContentProps {
  onDismiss: () => void;
}

export function EditorBetaDialogContent({ onDismiss }: EditorBetaDialogContentProps) {
  return (
    <DialogContent data-testid="editor-beta-dialog" size="sm">
      <DialogHeader>
        <DialogTitle>Recipe Editor (Beta)</DialogTitle>
        <DialogClose />
      </DialogHeader>
      <DialogDescription>
        Build custom multi-step recipes by composing nodes. You can import and export recipes as
        .bnto.json files to keep your work. Cloud-based recipe saving is coming soon — create an
        account and we&apos;ll persist and manage your recipes for you.
      </DialogDescription>
      <DialogFooter>
        <DialogClose asChild>
          <Button onClick={onDismiss} data-testid="beta-get-started">
            Got it
          </Button>
        </DialogClose>
      </DialogFooter>
    </DialogContent>
  );
}
