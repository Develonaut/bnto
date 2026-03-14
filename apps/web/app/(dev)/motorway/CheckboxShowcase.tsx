"use client";

import { Checkbox, Label, Row } from "@bnto/ui";

export function CheckboxShowcase() {
  return (
    <Row className="flex-wrap gap-8">
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
  );
}
