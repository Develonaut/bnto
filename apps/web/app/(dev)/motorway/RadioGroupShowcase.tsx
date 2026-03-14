"use client";

import { useState } from "react";

import { Label, RadioGroup, RadioGroupItem, Row, Stack } from "@bnto/ui";

export function RadioGroupShowcase() {
  const [radioVal, setRadioVal] = useState("jpeg");

  return (
    <div className="grid grid-cols-2 gap-8">
      <Stack gap="sm">
        <Label>Output format</Label>
        <RadioGroup value={radioVal} onValueChange={setRadioVal}>
          <Row className="items-center gap-2">
            <RadioGroupItem value="jpeg" id="radio-jpeg" />
            <Label htmlFor="radio-jpeg">JPEG</Label>
          </Row>
          <Row className="items-center gap-2">
            <RadioGroupItem value="png" id="radio-png" />
            <Label htmlFor="radio-png">PNG</Label>
          </Row>
          <Row className="items-center gap-2">
            <RadioGroupItem value="webp" id="radio-webp" />
            <Label htmlFor="radio-webp">WebP</Label>
          </Row>
        </RadioGroup>
      </Stack>
      <Stack gap="sm">
        <Label>Disabled</Label>
        <RadioGroup defaultValue="medium" disabled>
          <Row className="items-center gap-2">
            <RadioGroupItem value="small" id="radio-small" />
            <Label htmlFor="radio-small" className="text-muted-foreground">
              Small
            </Label>
          </Row>
          <Row className="items-center gap-2">
            <RadioGroupItem value="medium" id="radio-medium" />
            <Label htmlFor="radio-medium" className="text-muted-foreground">
              Medium
            </Label>
          </Row>
          <Row className="items-center gap-2">
            <RadioGroupItem value="large" id="radio-large" />
            <Label htmlFor="radio-large" className="text-muted-foreground">
              Large
            </Label>
          </Row>
        </RadioGroup>
      </Stack>
    </div>
  );
}
