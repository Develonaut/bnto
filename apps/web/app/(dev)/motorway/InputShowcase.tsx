"use client";

import { Input, Label, PasswordInput, Stack, Text } from "@bnto/ui";

export function InputShowcase() {
  return (
    <Stack className="gap-10">
      {/* Text inputs */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Text inputs
        </Text>
        <div className="grid grid-cols-3 gap-4">
          <Stack gap="xs">
            <Label htmlFor="input-default">Default</Label>
            <Input id="input-default" placeholder="Placeholder text" />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="input-filled">Filled</Label>
            <Input id="input-filled" defaultValue="compress-images" />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="input-disabled">Disabled</Label>
            <Input id="input-disabled" defaultValue="Read only" disabled />
          </Stack>
        </div>
      </div>

      {/* Password inputs */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Password inputs
        </Text>
        <div className="grid grid-cols-3 gap-4">
          <Stack gap="xs">
            <Label htmlFor="password-default">Default</Label>
            <PasswordInput id="password-default" placeholder="Enter password" />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="password-filled">Filled</Label>
            <PasswordInput id="password-filled" defaultValue="supersecret" />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="password-disabled">Disabled</Label>
            <PasswordInput id="password-disabled" defaultValue="locked" disabled />
          </Stack>
        </div>
      </div>
    </Stack>
  );
}
