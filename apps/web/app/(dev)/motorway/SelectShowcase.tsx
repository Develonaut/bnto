"use client";

import { useState } from "react";

import {
  Label,
  Row,
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
  SelectGroup,
  SelectLabel,
  SelectSeparator,
  Stack,
  Text,
} from "@bnto/ui";

export function SelectShowcase() {
  const [format, setFormat] = useState("webp");
  const [quality, setQuality] = useState("high");
  const [size, setSize] = useState("");

  return (
    <Stack className="gap-10">
      {/* Default selects */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Select variants
        </Text>
        <Row className="flex-wrap gap-8">
          <Stack gap="xs">
            <Label>Output format</Label>
            <Select value={format} onValueChange={setFormat}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="jpeg">JPEG</SelectItem>
                <SelectItem value="png">PNG</SelectItem>
                <SelectItem value="webp">WebP</SelectItem>
                <SelectItem value="avif">AVIF</SelectItem>
              </SelectContent>
            </Select>
          </Stack>

          <Stack gap="xs">
            <Label>Quality</Label>
            <Select value={quality} onValueChange={setQuality}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="low">Low</SelectItem>
                <SelectItem value="medium">Medium</SelectItem>
                <SelectItem value="high">High</SelectItem>
                <SelectItem value="lossless">Lossless</SelectItem>
              </SelectContent>
            </Select>
          </Stack>

          <Stack gap="xs">
            <Label>Placeholder</Label>
            <Select value={size} onValueChange={setSize}>
              <SelectTrigger>
                <SelectValue placeholder="Select size..." />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="sm">Small (480px)</SelectItem>
                <SelectItem value="md">Medium (1024px)</SelectItem>
                <SelectItem value="lg">Large (1920px)</SelectItem>
                <SelectItem value="xl">Extra Large (3840px)</SelectItem>
              </SelectContent>
            </Select>
          </Stack>

          <Stack gap="xs">
            <Label className="text-muted-foreground">Disabled</Label>
            <Select value="jpeg" disabled>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="jpeg">JPEG</SelectItem>
              </SelectContent>
            </Select>
          </Stack>
        </Row>
      </div>

      {/* Small size */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Small trigger
        </Text>
        <Row className="flex-wrap gap-8">
          <Stack gap="xs">
            <Label>Node type</Label>
            <Select defaultValue="image">
              <SelectTrigger size="sm">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="image">Image</SelectItem>
                <SelectItem value="file">File</SelectItem>
                <SelectItem value="csv">CSV</SelectItem>
                <SelectItem value="http">HTTP</SelectItem>
              </SelectContent>
            </Select>
          </Stack>
        </Row>
      </div>

      {/* With groups and labels */}
      <div>
        <Text size="sm" color="muted" className="mb-3">
          Grouped items
        </Text>
        <Row className="flex-wrap gap-8">
          <Stack gap="xs">
            <Label>Operation</Label>
            <Select defaultValue="compress">
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  <SelectLabel>Image</SelectLabel>
                  <SelectItem value="compress">Compress</SelectItem>
                  <SelectItem value="resize">Resize</SelectItem>
                  <SelectItem value="convert">Convert</SelectItem>
                </SelectGroup>
                <SelectSeparator />
                <SelectGroup>
                  <SelectLabel>File</SelectLabel>
                  <SelectItem value="rename">Rename</SelectItem>
                  <SelectItem value="organize">Organize</SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </Stack>
        </Row>
      </div>
    </Stack>
  );
}
