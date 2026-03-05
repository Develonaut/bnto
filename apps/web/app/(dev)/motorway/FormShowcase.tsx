"use client";

import { useState } from "react";
import {
  Button,
  Card,
  Checkbox,
  Input,
  Label,
  Row,
  Slider,
  Stack,
  Switch,
  Textarea,
} from "@bnto/ui";

export function FormShowcase() {
  const [switchOn, setSwitchOn] = useState(false);
  const [sliderVal, setSliderVal] = useState([50]);

  return (
    <Card elevation="lg">
      <Card.Header>
        <Card.Title className="font-display">Form Elements</Card.Title>
        <Card.Description>
          Inputs, switches, checkboxes, and controls with shadow integration
        </Card.Description>
      </Card.Header>
      <Card.Content className="space-y-6">
        <div className="grid grid-cols-2 gap-6">
          <Stack gap="sm">
            <Label htmlFor="name">Name</Label>
            <Input id="name" placeholder="Enter your name" />
          </Stack>
          <Stack gap="sm">
            <Label htmlFor="email">Email</Label>
            <Input id="email" type="email" placeholder="you@example.com" />
          </Stack>
        </div>

        <Stack gap="sm">
          <Label htmlFor="bio">Bio</Label>
          <Textarea id="bio" placeholder="Tell us about yourself..." />
        </Stack>

        <Row className="gap-8">
          <Row className="items-center gap-2">
            <Switch checked={switchOn} onCheckedChange={setSwitchOn} />
            <Label>Notifications {switchOn ? "on" : "off"}</Label>
          </Row>
          <Row className="items-center gap-2">
            <Checkbox id="terms" />
            <Label htmlFor="terms">Accept terms</Label>
          </Row>
        </Row>

        <Stack gap="sm">
          <Label>Quality: {sliderVal[0]}%</Label>
          <Slider value={sliderVal} onValueChange={setSliderVal} max={100} />
        </Stack>
      </Card.Content>
      <Card.Footer className="gap-3">
        <Button>Save changes</Button>
        <Button variant="muted">Cancel</Button>
      </Card.Footer>
    </Card>
  );
}
