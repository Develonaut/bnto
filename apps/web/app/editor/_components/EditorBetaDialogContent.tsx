"use client";

import {
  Button,
  DialogBody,
  DialogClose,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@bnto/ui";

interface EditorBetaDialogContentProps {
  onDismiss: () => void;
}

export function EditorBetaDialogContent({ onDismiss }: EditorBetaDialogContentProps) {
  return (
    <DialogContent data-testid="editor-beta-dialog" size="lg">
      <DialogHeader>
        <DialogTitle>Recipe Editor (Experimental)</DialogTitle>
        <DialogClose />
      </DialogHeader>
      <DialogBody className="flex gap-6">
        {/* eslint-disable-next-line @next/next/no-img-element -- SVG mascot, next/image not needed */}
        <img
          src="/mascots/beta-mascot.svg"
          alt=""
          aria-hidden
          className="hidden h-64 w-auto shrink-0 sm:block"
        />
        <div className="space-y-3 text-sm text-muted-foreground">
          <p>
            We&apos;re still working out the kinks. This is an early look at visual recipe building.
            Add nodes, configure them, and export as{" "}
            <code className="rounded bg-muted px-1 font-mono text-xs">.bnto.json</code>.
          </p>
          <p>
            Our focus right now is the CLI. Everything bnto does is powered by the engine, and the
            CLI is where it&apos;s sharpest. Try{" "}
            <code className="rounded bg-muted px-1 font-mono text-xs">bnto run</code> to see what we
            mean.
          </p>
        </div>
      </DialogBody>
      <DialogFooter>
        <DialogClose asChild>
          <Button variant="secondary" onClick={onDismiss} data-testid="beta-get-started">
            Explore anyway
          </Button>
        </DialogClose>
      </DialogFooter>
    </DialogContent>
  );
}
