"use client";

import { useState } from "react";

import { Combobox, Label, Stack, Text } from "@bnto/ui";

const imageExtensions = [
  { value: ".jpg", label: ".jpg" },
  { value: ".jpeg", label: ".jpeg" },
  { value: ".png", label: ".png" },
  { value: ".webp", label: ".webp" },
  { value: ".gif", label: ".gif" },
  { value: ".svg", label: ".svg" },
  { value: ".bmp", label: ".bmp" },
  { value: ".tiff", label: ".tiff" },
];

const outputFormats = [
  { value: "jpeg", label: "JPEG" },
  { value: "png", label: "PNG" },
  { value: "webp", label: "WebP" },
];

export function ComboboxShowcase() {
  const [extensions, setExtensions] = useState([".jpg", ".png", ".webp"]);
  const [empty, setEmpty] = useState<string[]>([]);
  const [formats, setFormats] = useState(["jpeg"]);

  return (
    <Stack className="gap-10">
      {/* Multi-select with badges */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Multi-select with badges
        </Text>
        <div className="grid grid-cols-2 gap-4">
          <Stack gap="xs">
            <Label>File extensions</Label>
            <Combobox
              options={imageExtensions}
              value={extensions}
              onChange={setExtensions}
              placeholder="Select extensions…"
              searchPlaceholder="Search extensions…"
            />
          </Stack>
          <Stack gap="xs">
            <Label>Empty state</Label>
            <Combobox
              options={imageExtensions}
              value={empty}
              onChange={setEmpty}
              placeholder="No extensions selected…"
            />
          </Stack>
        </div>
      </div>

      {/* Edge cases */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Edge cases
        </Text>
        <div className="grid grid-cols-2 gap-4">
          <Stack gap="xs">
            <Label>Max 2 selections</Label>
            <Combobox
              options={outputFormats}
              value={formats}
              onChange={setFormats}
              max={2}
              placeholder="Select formats…"
            />
          </Stack>
          <Stack gap="xs">
            <Label>Disabled</Label>
            <Combobox
              options={outputFormats}
              value={["jpeg", "webp"]}
              onChange={() => {}}
              disabled
            />
          </Stack>
        </div>
      </div>
    </Stack>
  );
}
