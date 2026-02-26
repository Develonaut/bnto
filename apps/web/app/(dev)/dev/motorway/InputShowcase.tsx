"use client";

import { useState } from "react";

import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { RadialSlider } from "@/components/ui/RadialSlider";
import { Row } from "@/components/ui/Row";
import { Slider } from "@/components/ui/slider";
import { Stack } from "@/components/ui/Stack";
import { Switch } from "@/components/ui/switch";
import { Text } from "@/components/ui/Text";
import { Textarea } from "@/components/ui/textarea";

export function InputShowcase() {
  const [switchA, setSwitchA] = useState(true);
  const [switchB, setSwitchB] = useState(false);
  const [slider, setSlider] = useState([65]);
  const [dial, setDial] = useState(180);

  return (
    <Stack className="gap-10">
      {/* Text inputs */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Text inputs</Text>
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

      {/* Textarea */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Textarea</Text>
        <div className="grid grid-cols-2 gap-4">
          <Stack gap="xs">
            <Label htmlFor="textarea-default">Description</Label>
            <Textarea id="textarea-default" placeholder="Enter a description..." />
          </Stack>
          <Stack gap="xs">
            <Label htmlFor="textarea-filled">With content</Label>
            <Textarea id="textarea-filled" defaultValue="Compress PNG, JPEG, and WebP images without losing quality. Files never leave your browser." />
          </Stack>
        </div>
      </div>

      {/* Toggles — switches and checkboxes */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Toggles</Text>
        <Row className="flex-wrap gap-8">
          <Row className="items-center gap-2">
            <Switch checked={switchA} onCheckedChange={setSwitchA} />
            <Label>Enabled</Label>
          </Row>
          <Row className="items-center gap-2">
            <Switch checked={switchB} onCheckedChange={setSwitchB} />
            <Label>Disabled</Label>
          </Row>
          <Row className="items-center gap-2">
            <Switch checked disabled />
            <Label className="text-muted-foreground">Locked on</Label>
          </Row>
          <Row className="items-center gap-2">
            <Checkbox defaultChecked />
            <Label>Checked</Label>
          </Row>
          <Row className="items-center gap-2">
            <Checkbox />
            <Label>Unchecked</Label>
          </Row>
          <Row className="items-center gap-2">
            <Checkbox checked disabled />
            <Label className="text-muted-foreground">Disabled</Label>
          </Row>
        </Row>
      </div>

      {/* Sliders */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Sliders</Text>
        <div className="grid grid-cols-2 gap-8">
          <Stack gap="sm">
            <Row className="justify-between">
              <Label>Quality</Label>
              <Text size="xs" mono color="muted">{slider[0]}%</Text>
            </Row>
            <Slider value={slider} onValueChange={setSlider} max={100} />
          </Stack>
          <Stack gap="sm" className="items-center">
            <Label>Direction</Label>
            <RadialSlider
              min={0}
              max={360}
              value={dial}
              onChange={setDial}
              startAngle={0}
              endAngle={360}
              size={96}
              strokeWidth={4}
              aria-label="Direction"
            >
              <Text size="xs" mono color="muted">{dial}&deg;</Text>
            </RadialSlider>
          </Stack>
        </div>
      </div>
    </Stack>
  );
}
