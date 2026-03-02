"use client";

import { useCallback, useEffect, useRef, useState } from "react";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";
import { Heading } from "@/components/ui/Heading";
import { Row } from "@/components/ui/Row";
import { Skeleton } from "@/components/ui/Skeleton";
import { Stack } from "@/components/ui/Stack";
import { Surface } from "@/components/ui/Surface";
import { Text } from "@/components/ui/Text";
import type { SurfaceElevation } from "@/components/ui/Surface";
import {
  LoaderIcon,
  ImageIcon,
  FileTextIcon,
  FileCogIcon,
  ZapIcon,
} from "@/components/ui/icons";

/* ── Helpers ─────────────────────────────────────────────────── */

const ELEVATIONS: SurfaceElevation[] = ["sm", "md", "lg"];

/**
 * A single demo card that shows the loading → loaded transition.
 * Content inside uses Skeleton primitives for the interior,
 * while the Surface itself handles the elevation spring.
 */
function DemoCard({
  loading,
  elevation = "md",
  label,
  icon: Icon,
}: {
  loading: boolean;
  elevation?: SurfaceElevation;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
}) {
  return (
    <Card
      loading={loading}
      elevation={elevation}
      className="flex h-40 flex-col justify-between p-5"
    >
      <Row align="start" justify="between">
        {loading ? (
          <Skeleton className="size-10 rounded-lg" />
        ) : (
          <div className="flex size-10 items-center justify-center rounded-lg bg-muted">
            <Icon className="size-5 text-muted-foreground" />
          </div>
        )}
        {loading ? (
          <Skeleton className="h-5 w-16 rounded-full" />
        ) : (
          <span className="rounded-full bg-muted px-2.5 py-0.5 text-xs font-medium text-muted-foreground">
            {elevation}
          </span>
        )}
      </Row>
      <Stack className="mt-auto gap-1.5 pt-4">
        {loading ? (
          <>
            <Skeleton className="h-5 w-3/4" />
            <Skeleton className="h-4 w-1/2" />
          </>
        ) : (
          <>
            <Text weight="semibold">{label}</Text>
            <Text size="sm" color="muted">
              elevation-{elevation}
            </Text>
          </>
        )}
      </Stack>
    </Card>
  );
}

/* ── Main Showcase ───────────────────────────────────────────── */

export function SurfaceLoadingShowcase() {
  const [loading, setLoading] = useState(true);
  const [autoplay, setAutoplay] = useState(false);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const toggle = useCallback(() => setLoading((l) => !l), []);

  // Autoplay: cycle loading ↔ loaded every 2s
  useEffect(() => {
    if (!autoplay) {
      if (intervalRef.current) clearInterval(intervalRef.current);
      return;
    }
    intervalRef.current = setInterval(toggle, 2000);
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, [autoplay, toggle]);

  return (
    <Stack gap="lg">
      {/* Controls */}
      <Row gap="sm" align="center">
        <Button
          variant={loading ? "primary" : "outline"}

          onClick={toggle}
        >
          {loading ? "Show Content" : "Show Loading"}
        </Button>
        <Button
          variant={autoplay ? "secondary" : "outline"}

          onClick={() => setAutoplay((a) => !a)}
        >
          {autoplay ? "Stop Autoplay" : "Autoplay"}
        </Button>
        <Text size="sm" color="muted" className="ml-2">
          {loading ? "Loading — surface flat on ground" : "Loaded — walls spring up"}
        </Text>
      </Row>

      {/* Section 1: Elevation comparison */}
      <div>
        <Heading level={3} size="xs" className="mb-3">
          Elevation Tiers
        </Heading>
        <Text size="sm" color="muted" className="mb-4">
          Same loading state, different target elevations. Taller buildings spring up more dramatically.
        </Text>
        <div className="grid grid-cols-3 gap-4">
          {ELEVATIONS.map((elev, i) =>
            loading ? (
              <DemoCard
                key={`${elev}-loading`}
                loading={true}
                elevation={elev}
                label={`elevation-${elev}`}
                icon={elev === "sm" ? ImageIcon : elev === "md" ? FileTextIcon : ZapIcon}
              />
            ) : (
              <Animate.ScaleIn key={`${elev}-loaded`} index={i} from={0.9} easing="spring-bouncy">
                <DemoCard
                  loading={false}
                  elevation={elev}
                  label={`elevation-${elev}`}
                  icon={elev === "sm" ? ImageIcon : elev === "md" ? FileTextIcon : ZapIcon}
                />
              </Animate.ScaleIn>
            ),
          )}
        </div>
      </div>

      {/* Section 2: Staggered dashboard loading */}
      <div>
        <Heading level={3} size="xs" className="mb-3">
          Staggered Dashboard
        </Heading>
        <Text size="sm" color="muted" className="mb-4">
          Cards load with staggered delays — each one springs up after the previous, like buildings appearing on a Mini Motorways map.
        </Text>
        <StaggeredDashboard />
      </div>

      {/* Section 3: Bare surface (no Card wrapper) */}
      <div>
        <Heading level={3} size="xs" className="mb-3">
          Raw Surface
        </Heading>
        <Text size="sm" color="muted" className="mb-4">
          The loading state works on any Surface — not just Card. Here it's a raw Surface with different color variants.
        </Text>
        <div className="grid grid-cols-4 gap-4">
          {(["default", "primary", "secondary", "accent"] as const).map(
            (variant, i) =>
              loading ? (
                <Surface
                  key={`${variant}-loading`}
                  variant={variant}
                  elevation="md"
                  loading={true}
                  rounded="xl"
                  className="flex h-24 items-center justify-center"
                >
                  <Skeleton className="h-4 w-20" />
                </Surface>
              ) : (
                <Animate.ScaleIn key={`${variant}-loaded`} index={i} from={0.9} easing="spring-bouncy">
                  <Surface
                    variant={variant}
                    elevation="md"
                    rounded="xl"
                    className="flex h-24 items-center justify-center"
                  >
                    <Text size="sm" weight="medium">
                      {variant}
                    </Text>
                  </Surface>
                </Animate.ScaleIn>
              ),
          )}
        </div>
      </div>
    </Stack>
  );
}

/* ── Staggered Dashboard ─────────────────────────────────────── */

const STAGGER_ITEMS = [
  { label: "Compress Images", icon: ImageIcon, delay: 0 },
  { label: "Clean CSV", icon: FileTextIcon, delay: 150 },
  { label: "Rename Files", icon: FileCogIcon, delay: 300 },
  { label: "Quick Convert", icon: ZapIcon, delay: 450 },
  { label: "API Request", icon: LoaderIcon, delay: 600 },
  { label: "Transform", icon: FileCogIcon, delay: 750 },
];

function StaggeredDashboard() {
  const [loaded, setLoaded] = useState<boolean[]>(
    new Array(STAGGER_ITEMS.length).fill(false),
  );
  const [running, setRunning] = useState(false);
  const timeoutsRef = useRef<ReturnType<typeof setTimeout>[]>([]);

  function startStagger() {
    // Reset all to loading
    setLoaded(new Array(STAGGER_ITEMS.length).fill(false));
    setRunning(true);

    // Clear previous timeouts
    timeoutsRef.current.forEach(clearTimeout);
    timeoutsRef.current = [];

    // Stagger each card's loading → loaded transition
    STAGGER_ITEMS.forEach((item, i) => {
      const t = setTimeout(() => {
        setLoaded((prev) => {
          const next = [...prev];
          next[i] = true;
          return next;
        });
      }, 800 + item.delay); // 800ms initial delay + stagger
      timeoutsRef.current.push(t);
    });

    // Auto-reset after all animations complete
    const resetTimeout = setTimeout(() => setRunning(false), 2500);
    timeoutsRef.current.push(resetTimeout);
  }

  function resetAll() {
    timeoutsRef.current.forEach(clearTimeout);
    timeoutsRef.current = [];
    setLoaded(new Array(STAGGER_ITEMS.length).fill(false));
    setRunning(false);
  }

  return (
    <Stack gap="sm">
      <Row gap="sm">
        <Button
          variant="primary"

          onClick={startStagger}
          disabled={running}
        >
          {running ? "Loading..." : "Simulate Load"}
        </Button>
        <Button variant="outline" onClick={resetAll}>
          Reset
        </Button>
      </Row>
      <div className="grid grid-cols-3 gap-4">
        {STAGGER_ITEMS.map((item, i) =>
          loaded[i] ? (
            <Animate.ScaleIn key={`${item.label}-loaded`} index={i} from={0.85} easing="spring-bouncy">
              <DemoCard
                loading={false}
                elevation="md"
                label={item.label}
                icon={item.icon}
              />
            </Animate.ScaleIn>
          ) : (
            <DemoCard
              key={`${item.label}-loading`}
              loading={true}
              elevation="md"
              label={item.label}
              icon={item.icon}
            />
          ),
        )}
      </div>
    </Stack>
  );
}
