import {
  ArrowRightIcon,
  DownloadIcon,
  GithubIcon,
  HeartIcon,
  SettingsIcon,
  StarIcon,
  ZapIcon,
} from "@/components/ui/icons";

import { Button } from "@/components/ui/button";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

type Variant = "primary" | "secondary" | "outline" | "muted" | "destructive" | "success" | "warning";

const VARIANTS: { value: Variant; label: string }[] = [
  { value: "primary", label: "Primary" },
  { value: "secondary", label: "Secondary" },
  { value: "outline", label: "Outline" },
  { value: "muted", label: "Muted" },
  { value: "destructive", label: "Destructive" },
  { value: "success", label: "Success" },
  { value: "warning", label: "Warning" },
];

export function ButtonShowcase() {
  return (
    <Stack className="gap-10">
      {/* All variants × sizes */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Variants &times; sizes</Text>
        <Stack gap="md">
          {VARIANTS.map(({ value, label }) => (
            <Row key={value} className="items-center gap-3">
              <Text size="xs" color="muted" mono as="span" className="w-24 shrink-0">
                {label}
              </Text>
              <Button variant={value} size="sm">Label</Button>
              <Button variant={value} size="md">Label</Button>
              <Button variant={value} size="lg">Label</Button>
              <Button variant={value} size="icon-sm"><ZapIcon /></Button>
              <Button variant={value} size="icon"><ZapIcon /></Button>
              <Button variant={value} size="icon-lg"><ZapIcon /></Button>
            </Row>
          ))}
        </Stack>
      </div>

      {/* Icon + text combinations */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Icon + text</Text>
        <Row className="flex-wrap gap-3">
          <Button variant="primary"><DownloadIcon /> Download</Button>
          <Button variant="outline"><SettingsIcon /> Settings</Button>
          <Button variant="secondary"><StarIcon /> Favorite</Button>
          <Button variant="destructive"><HeartIcon /> Remove</Button>
          <Button variant="outline">Next <ArrowRightIcon /></Button>
        </Row>
      </div>

      {/* Press states */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Press states &mdash; resting, hover, active</Text>
        <Row className="items-end gap-3">
          <Button variant="outline">Resting</Button>
          <Button variant="outline" pseudo="hover">Hover</Button>
          <Button variant="outline" pseudo="active">Active</Button>
          <Button variant="outline" size="icon"><StarIcon /></Button>
          <Button variant="outline" size="icon" pseudo="hover"><HeartIcon /></Button>
          <Button variant="outline" size="icon" pseudo="active"><ZapIcon /></Button>
        </Row>
      </div>

      {/* Disabled */}
      <div>
        <Text size="sm" color="muted" className="mb-3">Disabled</Text>
        <Row className="flex-wrap items-end gap-3">
          <Button variant="primary" disabled>Primary</Button>
          <Button variant="secondary" disabled>Secondary</Button>
          <Button variant="outline" disabled>Outline</Button>
          <Button variant="destructive" disabled>Destructive</Button>
          <Button variant="outline" size="icon" disabled><SettingsIcon /></Button>
          <Button variant="primary" size="icon" disabled><GithubIcon /></Button>
        </Row>
      </div>
    </Stack>
  );
}
