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
        <DialogTitle>Welcome to the Recipe Editor</DialogTitle>
        <DialogClose />
      </DialogHeader>
      <DialogDescription>
        The recipe editor is in beta. We&apos;re actively experimenting and testing functionality
        here, so things may change as we refine the experience.
      </DialogDescription>
      <DialogFooter>
        <DialogClose asChild>
          <Button onClick={onDismiss} data-testid="beta-get-started">
            Get started
          </Button>
        </DialogClose>
      </DialogFooter>
    </DialogContent>
  );
}
