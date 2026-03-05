"use client";

import { AuthGate } from "@/components/blocks/AuthGate";
import { Button, Dialog, Heading, Row, Stack, Text } from "@bnto/ui";

/* ── Basic Dialog ───────────────────────────────────────────── */

function BasicDialogDemo() {
  return (
    <Dialog>
      <Dialog.Trigger asChild>
        <Button variant="primary">Open Dialog</Button>
      </Dialog.Trigger>
      <Dialog.Content>
        <Dialog.Header>
          <Dialog.Title>Basic Dialog</Dialog.Title>
          <Dialog.Close />
        </Dialog.Header>
        <Dialog.Body>
          <Dialog.Description>
            A simple dialog with a title, description, and action buttons.
            Click outside or press Escape to close.
          </Dialog.Description>
        </Dialog.Body>
        <Dialog.Footer>
          <Dialog.Close asChild>
            <Button variant="outline">Cancel</Button>
          </Dialog.Close>
          <Dialog.Close asChild>
            <Button variant="primary">Confirm</Button>
          </Dialog.Close>
        </Dialog.Footer>
      </Dialog.Content>
    </Dialog>
  );
}

/* ── Destructive Confirmation ───────────────────────────────── */

function DestructiveDialogDemo() {
  return (
    <Dialog>
      <Dialog.Trigger asChild>
        <Button variant="destructive">Delete Recipe</Button>
      </Dialog.Trigger>
      <Dialog.Content className="max-w-sm">
        <Dialog.Header>
          <Dialog.Title>Delete recipe?</Dialog.Title>
          <Dialog.Close />
        </Dialog.Header>
        <Dialog.Body>
          <Dialog.Description>
            This will permanently delete &ldquo;Compress Images&rdquo; and all
            its execution history. This action cannot be undone.
          </Dialog.Description>
        </Dialog.Body>
        <Dialog.Footer>
          <Dialog.Close asChild>
            <Button variant="outline">Keep recipe</Button>
          </Dialog.Close>
          <Dialog.Close asChild>
            <Button variant="destructive">Delete</Button>
          </Dialog.Close>
        </Dialog.Footer>
      </Dialog.Content>
    </Dialog>
  );
}

/* ── Form Dialog ────────────────────────────────────────────── */

function FormDialogDemo() {
  return (
    <Dialog>
      <Dialog.Trigger asChild>
        <Button variant="outline">Rename Recipe</Button>
      </Dialog.Trigger>
      <Dialog.Content>
        <Dialog.Header>
          <Dialog.Title>Rename recipe</Dialog.Title>
          <Dialog.Close />
        </Dialog.Header>
        <Dialog.Body>
          <Dialog.Description>
            Give your recipe a new name. This won&apos;t affect its URL.
          </Dialog.Description>
          <label className="mt-4 flex flex-col gap-2">
            <Text size="sm" weight="medium">Name</Text>
            <input
              type="text"
              defaultValue="Compress Images"
              className="flex h-9 w-full rounded-md border border-border bg-input px-3 py-1 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring"
            />
          </label>
        </Dialog.Body>
        <Dialog.Footer>
          <Dialog.Close asChild>
            <Button variant="outline">Cancel</Button>
          </Dialog.Close>
          <Dialog.Close asChild>
            <Button variant="primary">Save</Button>
          </Dialog.Close>
        </Dialog.Footer>
      </Dialog.Content>
    </Dialog>
  );
}

/* ── Forced Choice Dialog ──────────────────────────────────── */

function ForcedChoiceDialogDemo() {
  return (
    <Dialog>
      <Dialog.Trigger asChild>
        <Button variant="outline">Terms & Conditions</Button>
      </Dialog.Trigger>
      <Dialog.Content
        className="max-w-sm"
        onInteractOutside={(e) => e.preventDefault()}
        onEscapeKeyDown={(e) => e.preventDefault()}
      >
        <Dialog.Header>
          <Dialog.Title>Accept terms to continue</Dialog.Title>
        </Dialog.Header>
        <Dialog.Body>
          <Dialog.Description>
            By continuing you agree to our Terms of Service and Privacy Policy.
            You must accept to use bnto.
          </Dialog.Description>
        </Dialog.Body>
        <Dialog.Footer>
          <Dialog.Close asChild>
            <Button variant="outline">Decline</Button>
          </Dialog.Close>
          <Dialog.Close asChild>
            <Button variant="primary">Accept</Button>
          </Dialog.Close>
        </Dialog.Footer>
      </Dialog.Content>
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
          Modal dialogs built on Radix Dialog. Focus-trapped, keyboard
          accessible, backdrop blur. Click each button to open.
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
        <Heading level={4}>AuthGate.Action</Heading>
        <Text size="sm" color="muted">
          Wraps interactive elements. Unauthenticated users see a dismissible
          conversion dialog. Click each button to test.
        </Text>
        <Row className="flex-wrap gap-3">
          <AuthGate.Action
            title="Sign up to re-run"
            description="Create a free account to re-run recipes."
          >
            <Button variant="ghost" className="h-7 px-2 text-xs">
              Re-run
            </Button>
          </AuthGate.Action>
          <AuthGate.Action>
            <Button variant="primary">Save recipe</Button>
          </AuthGate.Action>
          <AuthGate.Action
            title="Sign up to export"
            description="Export your recipe as a .bnto.json file."
          >
            <Button variant="outline">Export</Button>
          </AuthGate.Action>
        </Row>
      </Stack>

    </Stack>
  );
}
