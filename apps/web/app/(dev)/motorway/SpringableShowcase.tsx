"use client";

import { useCallback, useRef, useState, useEffect } from "react";

import { Button, Card, Heading, Row, Skeleton, Stack, Text } from "@bnto/ui";

/* ── Shared OptionBar ──────────────────────────────────────────── */

type Elevation = "sm" | "md" | "lg";

const ELEVATIONS: { value: Elevation; label: string }[] = [
  { value: "sm", label: "sm" },
  { value: "md", label: "md" },
  { value: "lg", label: "lg" },
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

/* ── Dormant + Elevation playground ────────────────────────────── */

function DormantPlayground() {
  const [elevation, setElevation] = useState<Elevation>("md");
  const [dormant, setDormant] = useState(true);

  return (
    <Stack gap="md">
      <OptionBar label="elevation" options={ELEVATIONS} value={elevation} onChange={setElevation} />

      <Row gap="sm" className="items-center">
        <Button variant={dormant ? "secondary" : "outline"} onClick={() => setDormant((d) => !d)}>
          {dormant ? "Load Content" : "Reset to Loading"}
        </Button>
        <Text size="sm" color="muted">
          elevation-{elevation} — {dormant ? "dormant" : "raised"}
        </Text>
      </Row>

      <div className="grid grid-cols-3 gap-6">
        {[1, 2, 3].map((i) => (
          <Card
            key={i}
            elevation={elevation}
            dormant={dormant}
            className="flex h-48 flex-col justify-between p-5"
          >
            {dormant ? (
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
              {dormant ? "dormant" : "raised"}
            </Text>
          </Card>
        ))}
      </div>
    </Stack>
  );
}

/* ── Scroll-triggered — IntersectionObserver + dormant ────────── */

function useInView(threshold = 0.3) {
  const [inView, setInView] = useState(false);
  const elRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const el = elRef.current;
    if (!el || inView) return;
    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setInView(true);
          observer.disconnect();
        }
      },
      { threshold, rootMargin: "-10% 0px" },
    );
    observer.observe(el);
    return () => observer.disconnect();
  }, [inView, threshold]);

  const ref = useCallback((node: HTMLDivElement | null) => {
    elRef.current = node;
  }, []);

  return [inView, ref] as const;
}

function ScrollRevealCard({ index, elevation }: { index: number; elevation: "sm" | "md" | "lg" }) {
  const [inView, ref] = useInView(0.6);

  return (
    <Card
      ref={ref}
      dormant={!inView}
      elevation={elevation}
      className="flex h-48 flex-col justify-between p-5"
    >
      <Stack gap="sm">
        <Heading level={3} size="xs">
          Card {index}
        </Heading>
        <Text size="sm" color="muted">
          Sprung up when scrolled into view.
        </Text>
      </Stack>
      <Text size="xs" color="muted" className="font-mono uppercase tracking-wider">
        {inView ? "raised" : "dormant"}
      </Text>
    </Card>
  );
}

/* ── Main showcase ────────────────────────────────────────────── */

export function SpringableShowcase() {
  return (
    <Stack gap="lg">
      <DormantPlayground />

      {/* Scroll-triggered reveal */}
      <Stack gap="sm">
        <Text size="sm" color="muted">
          Scroll down — cards spring up from dormancy when they enter the viewport.
        </Text>
        <div className="grid grid-cols-3 gap-6">
          {[1, 2, 3].map((i) => (
            <ScrollRevealCard key={i} index={i} elevation="md" />
          ))}
        </div>
      </Stack>
    </Stack>
  );
}
