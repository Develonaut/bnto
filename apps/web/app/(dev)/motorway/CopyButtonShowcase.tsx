"use client";

import { CopyButton, Input, Label, Row, Stack } from "@bnto/ui";

export function CopyButtonShowcase() {
  return (
    <div className="grid grid-cols-3 gap-4">
      <Stack gap="xs">
        <Label>Default</Label>
        <Row className="gap-2">
          <Input defaultValue="https://bnto.io/compress-images" wrapperClassName="min-w-0 flex-1" readOnly />
          <CopyButton value="https://bnto.io/compress-images" />
        </Row>
      </Stack>
      <Stack gap="xs">
        <Label>Copied</Label>
        <Row className="gap-2">
          <Input defaultValue="npx bnto run compress" wrapperClassName="min-w-0 flex-1" readOnly />
          <CopyButton value="npx bnto run compress" />
        </Row>
      </Stack>
      <Stack gap="xs">
        <Label>Empty</Label>
        <Row className="gap-2">
          <Input placeholder="Nothing to copy" wrapperClassName="min-w-0 flex-1" readOnly />
          <CopyButton value="" />
        </Row>
      </Stack>
    </div>
  );
}
