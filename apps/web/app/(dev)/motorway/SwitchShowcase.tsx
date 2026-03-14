"use client";

import { useState } from "react";

import { Label, Row, Switch } from "@bnto/ui";

export function SwitchShowcase() {
  const [switchA, setSwitchA] = useState(true);
  const [switchB, setSwitchB] = useState(false);

  return (
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
    </Row>
  );
}
