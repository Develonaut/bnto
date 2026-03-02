"use client";

import { Button } from "@/components/ui/Button";
import { Dialog } from "@/components/ui/Dialog";
import { Heading } from "@/components/ui/Heading";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { Card } from "@/components/ui/Card";
import { Skeleton } from "@/components/ui/Skeleton";

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

/* ── Auth Gate Demo ─────────────────────────────────────────── */

function AccountGateDemo() {
  return (
    <Stack className="gap-3">
      <Text size="sm" color="muted">
        Blurs content and overlays a sign-up prompt for unauthenticated users.
        Since you&apos;re viewing the showcase, here&apos;s the visual effect:
      </Text>
      <div className="relative rounded-xl border border-border p-6">
        {/* Simulated blurred content */}
        <div
          aria-hidden="true"
          className="pointer-events-none select-none blur-[6px] opacity-75"
        >
          <Stack className="gap-4">
            <Row className="gap-3">
              {Array.from({ length: 3 }).map((_, i) => (
                <Card key={i} elevation="sm" className="flex-1 px-4 py-3">
                  <Stack className="gap-1.5">
                    <Skeleton className="h-3 w-16" />
                    <Skeleton className="h-6 w-12" />
                  </Stack>
                </Card>
              ))}
            </Row>
            <Stack className="gap-2">
              {Array.from({ length: 3 }).map((_, i) => (
                <Card key={i} elevation="sm" className="px-4 py-3">
                  <Row justify="between" align="center">
                    <Stack className="gap-1">
                      <Skeleton className="h-4 w-32" />
                      <Skeleton className="h-3 w-20" />
                    </Stack>
                    <Skeleton className="h-5 w-16 rounded-full" />
                  </Row>
                </Card>
              ))}
            </Stack>
          </Stack>
        </div>

        {/* Floating sign-up card */}
        <div className="absolute inset-0 flex items-center justify-center">
          <Card elevation="lg" className="w-full max-w-sm p-8">
            <Stack className="items-center gap-4 text-center">
              <Heading level={3} size="sm">
                Sign in to get started
              </Heading>
              <Text size="sm" color="muted" className="max-w-xs">
                Create a free account to save recipes and track history.
              </Text>
              <Row className="gap-3 pt-2">
                <Button variant="primary" elevation="sm" onClick={() => {}}>
                  Sign in
                </Button>
                <Button variant="outline" onClick={() => {}}>
                  Create account
                </Button>
              </Row>
            </Stack>
          </Card>
        </div>
      </div>
    </Stack>
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

      {/* Account Gate */}
      <Stack className="gap-3">
        <Heading level={4}>Account Gate</Heading>
        <AccountGateDemo />
      </Stack>
    </Stack>
  );
}
