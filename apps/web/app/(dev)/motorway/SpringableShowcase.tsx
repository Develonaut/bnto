"use client";

import { useState } from "react";

import { Button, Card, Heading, Row, Skeleton, Stack, Text } from "@bnto/ui";

/* ── Shared OptionBar ──────────────────────────────────────────── */

type Elevation = "sm" | "md" | "lg";
type Spring = "bouncy" | "bouncier" | "bounciest";

const ELEVATIONS: { value: Elevation; label: string }[] = [
  { value: "sm", label: "sm" },
  { value: "md", label: "md" },
  { value: "lg", label: "lg" },
];

const SPRINGS: { value: Spring; label: string; description: string }[] = [
  { value: "bouncy", label: "bouncy", description: "150ms ease-out" },
  { value: "bouncier", label: "bouncier", description: "400ms single bounce" },
  { value: "bounciest", label: "bounciest (default)", description: "550ms rubber band" },
];

function OptionBar<T extends string>({
  label,
  options,
  value,
  onChange,
}: {
  label: string;
  options: { value: T; label: string }[];
  value: T;
  onChange: (v: T) => void;
}) {
  return (
    <Row gap="sm" className="items-center">
      <Text size="sm" color="muted" className="w-20 shrink-0 font-mono uppercase tracking-wider">
        {label}
      </Text>
      {options.map((opt) => (
        <Button
          key={opt.value}
          variant={value === opt.value ? "primary" : "outline"}
          onClick={() => onChange(opt.value)}
        >
          {opt.label}
        </Button>
      ))}
    </Row>
  );
}

/* ── Spring + Elevation playground ─────────────────────────────── */

export function SpringableShowcase() {
  const [elevation, setElevation] = useState<Elevation>("md");
  const [spring, setSpring] = useState<Spring>("bounciest");
  const [grounded, setGrounded] = useState(true);

  const springDesc = SPRINGS.find((s) => s.value === spring)?.description;

  return (
    <Stack gap="md">
      <Stack gap="sm">
        <OptionBar
          label="elevation"
          options={ELEVATIONS}
          value={elevation}
          onChange={setElevation}
        />
        <OptionBar
          label="spring"
          options={SPRINGS}
          value={spring}
          onChange={setSpring}
        />
      </Stack>

      <Row gap="sm" className="items-center">
        <Button
          variant={grounded ? "secondary" : "outline"}
          onClick={() => setGrounded((g) => !g)}
        >
          {grounded ? "Load Content" : "Reset to Loading"}
        </Button>
        <Text size="sm" color="muted">
          elevation-{elevation} + spring-{spring} ({springDesc})
          {grounded ? " — grounded" : " — raised"}
        </Text>
      </Row>

      <div className="grid grid-cols-3 gap-6">
        {[1, 2, 3].map((i) => (
          <Card
            key={i}
            elevation={elevation}
            spring={spring}
            grounded={grounded}
            className="flex h-48 flex-col justify-between p-5"
          >
            {grounded ? (
              <Stack gap="sm">
                <Skeleton className="h-5 w-2/3 rounded" />
                <Skeleton className="h-3 w-full rounded" />
                <Skeleton className="h-3 w-4/5 rounded" />
              </Stack>
            ) : (
              <Stack gap="sm">
                <Heading level={3} size="xs">
                  Card {i}
                </Heading>
                <Text size="sm" color="muted">
                  Content loaded. Sprung up from the ground plane.
                </Text>
              </Stack>
            )}
            <Text size="xs" color="muted" className="font-mono uppercase tracking-wider">
              {grounded ? "grounded" : "raised"}
            </Text>
          </Card>
        ))}
      </div>
    </Stack>
  );
}

/* ── Card loading prop demo ────────────────────────────────────── */

export function LoadingCardShowcase() {
  const [loading, setLoading] = useState(true);

  return (
    <Stack gap="md">
      <Row gap="sm" className="items-center">
        <Button
          variant={loading ? "secondary" : "outline"}
          onClick={() => setLoading((l) => !l)}
        >
          {loading ? "Load Content" : "Reset to Loading"}
        </Button>
        <Text size="sm" color="muted">
          {'<Card loading={isLoading}>'} — bounciest spring by default
        </Text>
      </Row>

      <div className="grid grid-cols-3 gap-6">
        {[1, 2, 3].map((i) => (
          <Card
            key={i}
            loading={loading}
            className="flex h-48 flex-col justify-between p-5"
          >
            {loading ? (
              <Stack gap="sm">
                <Skeleton className="h-5 w-2/3 rounded" />
                <Skeleton className="h-3 w-full rounded" />
                <Skeleton className="h-3 w-4/5 rounded" />
              </Stack>
            ) : (
              <Stack gap="sm">
                <Heading level={3} size="xs">
                  Card {i}
                </Heading>
                <Text size="sm" color="muted">
                  Content loaded. Sprung up from the ground plane.
                </Text>
              </Stack>
            )}
            <Text size="xs" color="muted" className="font-mono uppercase tracking-wider">
              {loading ? "grounded" : "raised"}
            </Text>
          </Card>
        ))}
      </div>
    </Stack>
  );
}
