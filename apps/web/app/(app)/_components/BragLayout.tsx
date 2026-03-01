"use client";

import { Fragment } from "react";

import {
  CheckIcon,
  CircleMinusIcon,
  ZapIcon,
} from "@/components/ui/icons";

import { Animate } from "@/components/ui/Animate";
import { AnimatedCounter } from "@/components/ui/AnimatedCounter";
import { ComparisonBar } from "@/components/ui/ComparisonBar";
import { Card } from "@/components/ui/Card";
import { IconBadge } from "@/components/ui/IconBadge";
import { Row } from "@/components/ui/Row";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";
import { cn } from "@/lib/cn";

/* ── Data ────────────────────────────────────────────────────── */

interface ComparisonRow {
  label: string;
  bnto: string;
  bntoWin: boolean;
  tinypng: string;
  iloveimg: string;
}

const COMPARISON_ROWS: ComparisonRow[] = [
  { label: "Speed", bnto: "50ms", bntoWin: true, tinypng: "~8s", iloveimg: "~12s" },
  { label: "File upload", bnto: "None", bntoWin: true, tinypng: "Required", iloveimg: "Required" },
  { label: "Daily limit", bnto: "Unlimited", bntoWin: true, tinypng: "20/day", iloveimg: "15/day" },
  { label: "Signup", bnto: "None", bntoWin: true, tinypng: "For bulk", iloveimg: "For bulk" },
  { label: "Cost", bnto: "Free", bntoWin: true, tinypng: "$25/yr", iloveimg: "$7/mo" },
];

/* ── Comparison cell ─────────────────────────────────────────── */

function ComparisonCell({ value, win }: { value: string; win?: boolean }) {
  return (
    <div className="flex items-center justify-center gap-1.5">
      {win !== undefined && (
        win
          ? <CheckIcon className="size-3.5 shrink-0 text-success" />
          : <CircleMinusIcon className="size-3.5 shrink-0 text-muted-foreground/50" />
      )}
      <span className="text-xs">{value}</span>
    </div>
  );
}

/* ── Brag layout ─────────────────────────────────────────────── */

export function BragLayout() {
  const speedCard = (
    <Card elevation="md" className="p-5">
      <Stack className="gap-3">
      <Row className="gap-3">
        <IconBadge variant="primary" size="md" shape="square">
          <ZapIcon className="size-4" />
        </IconBadge>
        <Row className="gap-1.5" align="baseline">
          <AnimatedCounter
            value={50}
            active
            className="font-display text-2xl font-bold tracking-tight"
          />
          <span className="text-muted-foreground text-sm font-medium">ms</span>
        </Row>
      </Row>
      <ComparisonBar
        active
        height="h-2.5"
        items={[
          { label: "bnto", value: 50, subtitle: "Local WASM" },
          { label: "TinyPNG", value: 8000, subtitle: "Upload, process, download" },
          { label: "iLoveIMG", value: 12000, subtitle: "Upload, queue, download" },
        ]}
      />
      <Text size="xs" color="muted">Avg processing time</Text>
      </Stack>
    </Card>
  );

  const comparisonCard = (
    <Card elevation="md">
    <div className="grid grid-cols-4 overflow-hidden rounded-[inherit]">
      {/* Header row */}
      <div className="border-b border-border p-3" />
      <div className="flex items-center justify-center border-b border-border bg-primary/5 p-3">
        <span className="font-display text-xs font-semibold text-primary">bnto</span>
      </div>
      <div className="flex items-center justify-center border-b border-border p-3">
        <span className="text-xs font-medium text-muted-foreground">TinyPNG</span>
      </div>
      <div className="flex items-center justify-center border-b border-border p-3">
        <span className="text-xs font-medium text-muted-foreground">iLoveIMG</span>
      </div>

      {/* Data rows */}
      {COMPARISON_ROWS.map((row, i) => {
        const isLast = i === COMPARISON_ROWS.length - 1;
        const borderCn = isLast ? "" : "border-b border-border";
        return (
          <Fragment key={row.label}>
            <div className={cn("flex items-center p-3", borderCn)}>
              <span className="text-xs font-medium text-muted-foreground">{row.label}</span>
            </div>
            <div className={cn("flex items-center justify-center bg-primary/5 p-3", borderCn)}>
              <ComparisonCell value={row.bnto} win={row.bntoWin} />
            </div>
            <div className={cn("flex items-center justify-center p-3", borderCn)}>
              <ComparisonCell value={row.tinypng} win={false} />
            </div>
            <div className={cn("flex items-center justify-center p-3", borderCn)}>
              <ComparisonCell value={row.iloveimg} win={false} />
            </div>
          </Fragment>
        );
      })}
    </div>
    </Card>
  );

  return (
    <Animate.Stagger asChild>
    <Stack className="gap-3">
      <Animate.ScaleIn index={0} from={0.9} easing="spring-bouncy">
        {speedCard}
      </Animate.ScaleIn>
      <Animate.ScaleIn index={1} from={0.9} easing="spring-bouncy">
        {comparisonCard}
      </Animate.ScaleIn>
    </Stack>
    </Animate.Stagger>
  );
}
