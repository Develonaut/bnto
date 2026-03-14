"use client";

import { Label, Stack, Textarea, Text } from "@bnto/ui";

export function TextareaShowcase() {
  return (
    <Stack className="gap-10">
      {/* Size variants */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Size variants
        </Text>
        <div className="grid grid-cols-2 gap-4">
          <Stack gap="xs">
            <Label htmlFor="textarea-sm">Small</Label>
            <Textarea id="textarea-sm" size="sm" placeholder="Small textarea…" />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="textarea-md">Medium (default)</Label>
            <Textarea id="textarea-md" placeholder="Medium textarea…" />
          </Stack>
        </div>
      </div>

      {/* States */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          States
        </Text>
        <div className="grid grid-cols-3 gap-4">
          <Stack gap="xs">
            <Label htmlFor="textarea-empty">Empty</Label>
            <Textarea id="textarea-empty" placeholder="Enter a description..." />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="textarea-filled">Filled</Label>
            <Textarea
              id="textarea-filled"
              defaultValue="Compress PNG, JPEG, and WebP images without losing quality. Files never leave your browser."
            />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="textarea-disabled">Disabled</Label>
            <Textarea id="textarea-disabled" defaultValue="Read-only content" disabled />
          </Stack>
        </div>
      </div>
    </Stack>
  );
}
