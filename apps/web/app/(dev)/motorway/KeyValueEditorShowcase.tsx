"use client";

import { useState } from "react";

import { KeyValueEditor, Label, Stack, Text } from "@bnto/ui";

export function KeyValueEditorShowcase() {
  const [columns, setColumns] = useState<Record<string, string>>({
    name: "full_name",
    email: "contact_email",
  });
  const [empty, setEmpty] = useState<Record<string, string>>({});

  return (
    <Stack className="gap-10">
      {/* Default with pairs */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          With pairs
        </Text>
        <div className="grid grid-cols-2 gap-8">
          <Stack gap="xs">
            <Label>Column renames</Label>
            <KeyValueEditor
              value={columns}
              onChange={setColumns}
              keyPlaceholder="Old name"
              valuePlaceholder="New name"
            />
          </Stack>
          <Stack gap="xs">
            <Label>Empty</Label>
            <KeyValueEditor value={empty} onChange={setEmpty} />
          </Stack>
        </div>
      </div>

      {/* Edge cases */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Edge cases
        </Text>
        <div className="grid grid-cols-2 gap-8">
          <Stack gap="xs">
            <Label>Max 3 pairs</Label>
            <KeyValueEditor value={columns} onChange={setColumns} max={3} />
          </Stack>
          <Stack gap="xs">
            <Label>Disabled</Label>
            <KeyValueEditor
              value={{ host: "localhost", port: "8080" }}
              onChange={() => {}}
              disabled
            />
          </Stack>
        </div>
      </div>
    </Stack>
  );
}
