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
        <DialogTitle>Recipe Editor (Experimental)</DialogTitle>
        <DialogClose />
      </DialogHeader>
      <DialogDescription>
        This is an early look at visual recipe building — drag nodes, wire them together, export as
        .bnto.json. It&apos;s a dream of what composing recipes could feel like, but our focus right
        now is the CLI. Everything bnto does is powered by the engine, and the CLI is where
        it&apos;s sharpest. Try{" "}
        <code className="rounded bg-muted px-1 font-mono text-xs">bnto run</code> to see what we
        mean.
      </DialogDescription>
      <DialogFooter>
        <DialogClose asChild>
          <Button onClick={onDismiss} data-testid="beta-get-started">
            Explore anyway
          </Button>
        </DialogClose>
      </DialogFooter>
    </DialogContent>
  );
}
