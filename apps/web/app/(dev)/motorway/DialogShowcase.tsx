"use client";

import { AuthGate } from "@/components/blocks/AuthGate";
import {
  Button,
  Dialog,
  DialogTrigger,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogClose,
  DialogBody,
  DialogDescription,
  DialogFooter,
  Heading,
  Row,
  Stack,
  Text,
} from "@bnto/ui";

/* ── Basic Dialog ───────────────────────────────────────────── */

function BasicDialogDemo() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="primary">Open Dialog</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Basic Dialog</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogBody>
          <DialogDescription>
            A simple dialog with a title, description, and action buttons. Click outside or press
            Escape to close.
          </DialogDescription>
        </DialogBody>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button variant="primary">Confirm</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/* ── Destructive Confirmation ───────────────────────────────── */

function DestructiveDialogDemo() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="destructive">Delete Recipe</Button>
      </DialogTrigger>
      <DialogContent className="max-w-sm">
        <DialogHeader>
          <DialogTitle>Delete recipe?</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogBody>
          <DialogDescription>
            This will permanently delete &ldquo;Compress Images&rdquo; and all its execution
            history. This action cannot be undone.
          </DialogDescription>
        </DialogBody>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Keep recipe</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button variant="destructive">Delete</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/* ── Form Dialog ────────────────────────────────────────────── */

function FormDialogDemo() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">Rename Recipe</Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Rename recipe</DialogTitle>
          <DialogClose />
        </DialogHeader>
        <DialogBody>
          <DialogDescription>
            Give your recipe a new name. This won&apos;t affect its URL.
          </DialogDescription>
          <label className="mt-4 flex flex-col gap-2">
            <Text size="sm" weight="medium">
              Name
            </Text>
            <input
              type="text"
              defaultValue="Compress Images"
              className="flex h-9 w-full rounded-md border border-border bg-input px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
            />
          </label>
        </DialogBody>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Cancel</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button variant="primary">Save</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/* ── Forced Choice Dialog ──────────────────────────────────── */

function ForcedChoiceDialogDemo() {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="outline">Terms & Conditions</Button>
      </DialogTrigger>
      <DialogContent
        className="max-w-sm"
        onInteractOutside={(e) => e.preventDefault()}
        onEscapeKeyDown={(e) => e.preventDefault()}
      >
        <DialogHeader>
          <DialogTitle>Accept terms to continue</DialogTitle>
        </DialogHeader>
        <DialogBody>
          <DialogDescription>
            By continuing you agree to our Terms of Service and Privacy Policy. You must accept to
            use bnto.
          </DialogDescription>
        </DialogBody>
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">Decline</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button variant="primary">Accept</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

/* ── Showcase ───────────────────────────────────────────────── */

export function DialogShowcase() {
  return (
    <Stack className="gap-10">
      {/* Dialog variants */}
      <Stack className="gap-3">
        <Heading level={4}>Dialogs</Heading>
        <Text size="sm" color="muted">
          Modal dialogs built on Radix Dialog. Focus-trapped, keyboard accessible, backdrop blur.
          Click each button to open.
        </Text>
        <Row className="flex-wrap gap-3">
          <BasicDialogDemo />
          <DestructiveDialogDemo />
          <FormDialogDemo />
          <ForcedChoiceDialogDemo />
        </Row>
      </Stack>

      {/* Auth Gate — Action (per-button dialog) */}
      <Stack className="gap-3">
        <Heading level={4}>AuthGate</Heading>
        <Text size="sm" color="muted">
          Wraps interactive elements. Unauthenticated users see a dismissible conversion dialog.
          Click each button to test.
        </Text>
        <Row className="flex-wrap gap-3">
          <AuthGate
            title="Sign up to re-run"
            description="Create a free account to re-run recipes."
          >
            <Button variant="ghost" className="h-7 px-2 text-xs">
              Re-run
            </Button>
          </AuthGate>
          <AuthGate>
            <Button variant="primary">Save recipe</Button>
          </AuthGate>
          <AuthGate
            title="Sign up to export"
            description="Export your recipe as a .bnto.json file."
          >
            <Button variant="outline">Export</Button>
          </AuthGate>
        </Row>
      </Stack>
    </Stack>
  );
}
