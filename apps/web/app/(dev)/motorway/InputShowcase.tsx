"use client";

import { useState } from "react";

import {
  Checkbox,
  Input,
  Label,
  PasswordInput,
  RadialSlider,
  RadioGroup,
  Row,
  Slider,
  Stack,
  Switch,
  Text,
  Textarea,
} from "@bnto/ui";

export function InputShowcase() {
  const [switchA, setSwitchA] = useState(true);
  const [switchB, setSwitchB] = useState(false);
  const [slider, setSlider] = useState([65]);
  const [dial, setDial] = useState(75);
  const [halfDial, setHalfDial] = useState(60);
  const [radioVal, setRadioVal] = useState("jpeg");

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

      {/* Password inputs */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Password inputs</Text>
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

      {/* Radio groups */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Radio groups</Text>
        <div className="grid grid-cols-2 gap-8">
          <Stack gap="sm">
            <Label>Output format</Label>
            <RadioGroup value={radioVal} onValueChange={setRadioVal}>
              <Row className="items-center gap-2">
                <RadioGroup.Item value="jpeg" id="radio-jpeg" />
                <Label htmlFor="radio-jpeg">JPEG</Label>
              </Row>
              <Row className="items-center gap-2">
                <RadioGroup.Item value="png" id="radio-png" />
                <Label htmlFor="radio-png">PNG</Label>
              </Row>
              <Row className="items-center gap-2">
                <RadioGroup.Item value="webp" id="radio-webp" />
                <Label htmlFor="radio-webp">WebP</Label>
              </Row>
            </RadioGroup>
          </Stack>
          <Stack gap="sm">
            <Label>Disabled</Label>
            <RadioGroup defaultValue="medium" disabled>
              <Row className="items-center gap-2">
                <RadioGroup.Item value="small" id="radio-small" />
                <Label htmlFor="radio-small" className="text-muted-foreground">Small</Label>
              </Row>
              <Row className="items-center gap-2">
                <RadioGroup.Item value="medium" id="radio-medium" />
                <Label htmlFor="radio-medium" className="text-muted-foreground">Medium</Label>
              </Row>
              <Row className="items-center gap-2">
                <RadioGroup.Item value="large" id="radio-large" />
                <Label htmlFor="radio-large" className="text-muted-foreground">Large</Label>
              </Row>
            </RadioGroup>
          </Stack>
        </div>
      </div>

      {/* Sliders */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Sliders</Text>
        <div className="grid grid-cols-3 gap-8">
          <Stack gap="sm">
            <Row className="justify-between">
              <Label>Quality</Label>
              <Text size="xs" mono color="muted">{slider[0]}%</Text>
            </Row>
            <Slider value={slider} onValueChange={setSlider} max={100} />
          </Stack>
          <Stack gap="sm" className="items-center">
            <Label>Radial</Label>
            <RadialSlider
              min={0}
              max={100}
              value={dial}
              onChange={setDial}
              startAngle={240}
              endAngle={120}
              size={180}
              aria-label="Quality"
            >
              <Text size="lg" mono className="font-semibold">{dial}%</Text>
            </RadialSlider>
          </Stack>
          <Stack gap="sm" className="items-center">
            <Label>Half-circle</Label>
            <RadialSlider
              min={0}
              max={100}
              value={halfDial}
              onChange={setHalfDial}
              startAngle={270}
              endAngle={90}
              size={180}
              hideRing
              aria-label="Half-circle gauge"
            >
              <Text size="lg" mono className="font-semibold">{halfDial}%</Text>
            </RadialSlider>
          </Stack>
        </div>
      </div>
    </Stack>
  );
}
