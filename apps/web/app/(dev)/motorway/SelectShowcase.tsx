"use client";

import { useState } from "react";

import { Label } from "@/components/ui/Label";
import { Row } from "@/components/ui/Row";
import { Select } from "@/components/ui/Select";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

export function SelectShowcase() {
  const [format, setFormat] = useState("webp");
  const [quality, setQuality] = useState("high");
  const [size, setSize] = useState("");

  return (
    <Stack className="gap-10">
      {/* Default selects */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Select variants</Text>
        <Row className="flex-wrap gap-8">
          <Stack gap="xs">
            <Label>Output format</Label>
            <Select value={format} onValueChange={setFormat}>
              <Select.Trigger>
                <Select.Value />
              </Select.Trigger>
              <Select.Content>
                <Select.Item value="jpeg">JPEG</Select.Item>
                <Select.Item value="png">PNG</Select.Item>
                <Select.Item value="webp">WebP</Select.Item>
                <Select.Item value="avif">AVIF</Select.Item>
              </Select.Content>
            </Select>
          </Stack>

          <Stack gap="xs">
            <Label>Quality</Label>
            <Select value={quality} onValueChange={setQuality}>
              <Select.Trigger>
                <Select.Value />
              </Select.Trigger>
              <Select.Content>
                <Select.Item value="low">Low</Select.Item>
                <Select.Item value="medium">Medium</Select.Item>
                <Select.Item value="high">High</Select.Item>
                <Select.Item value="lossless">Lossless</Select.Item>
              </Select.Content>
            </Select>
          </Stack>

          <Stack gap="xs">
            <Label>Placeholder</Label>
            <Select value={size} onValueChange={setSize}>
              <Select.Trigger>
                <Select.Value placeholder="Select size..." />
              </Select.Trigger>
              <Select.Content>
                <Select.Item value="sm">Small (480px)</Select.Item>
                <Select.Item value="md">Medium (1024px)</Select.Item>
                <Select.Item value="lg">Large (1920px)</Select.Item>
                <Select.Item value="xl">Extra Large (3840px)</Select.Item>
              </Select.Content>
            </Select>
          </Stack>

          <Stack gap="xs">
            <Label className="text-muted-foreground">Disabled</Label>
            <Select value="jpeg" disabled>
              <Select.Trigger>
                <Select.Value />
              </Select.Trigger>
              <Select.Content>
                <Select.Item value="jpeg">JPEG</Select.Item>
              </Select.Content>
            </Select>
          </Stack>
        </Row>
      </div>

      {/* Small size */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Small trigger</Text>
        <Row className="flex-wrap gap-8">
          <Stack gap="xs">
            <Label>Node type</Label>
            <Select defaultValue="image">
              <Select.Trigger size="sm">
                <Select.Value />
              </Select.Trigger>
              <Select.Content>
                <Select.Item value="image">Image</Select.Item>
                <Select.Item value="file">File</Select.Item>
                <Select.Item value="csv">CSV</Select.Item>
                <Select.Item value="http">HTTP</Select.Item>
              </Select.Content>
            </Select>
          </Stack>
        </Row>
      </div>

      {/* With groups and labels */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Grouped items</Text>
        <Row className="flex-wrap gap-8">
          <Stack gap="xs">
            <Label>Operation</Label>
            <Select defaultValue="compress">
              <Select.Trigger>
                <Select.Value />
              </Select.Trigger>
              <Select.Content>
                <Select.Group>
                  <Select.Label>Image</Select.Label>
                  <Select.Item value="compress">Compress</Select.Item>
                  <Select.Item value="resize">Resize</Select.Item>
                  <Select.Item value="convert">Convert</Select.Item>
                </Select.Group>
                <Select.Separator />
                <Select.Group>
                  <Select.Label>File</Select.Label>
                  <Select.Item value="rename">Rename</Select.Item>
                  <Select.Item value="organize">Organize</Select.Item>
                </Select.Group>
              </Select.Content>
            </Select>
          </Stack>
        </Row>
      </div>
    </Stack>
  );
}
