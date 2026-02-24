"use client";

import { useState } from "react";
import {
  Minimize2,
  Scaling,
  ArrowRightLeft,
  PenLine,
  Sparkles,
  Columns3,
  RotateCcw,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";

/* ── Helpers ─────────────────────────────────────────────── */

const DEMO_RECIPES = [
  { title: "Compress Images", icon: Minimize2 },
  { title: "Resize Images", icon: Scaling },
  { title: "Convert Format", icon: ArrowRightLeft },
  { title: "Rename Files", icon: PenLine },
  { title: "Clean CSV", icon: Sparkles },
  { title: "Rename Columns", icon: Columns3 },
];

function ReplayButton({ onClick }: { onClick: () => void }) {
  return (
    <Button variant="outline" size="sm" onClick={onClick} depth={false}>
      <RotateCcw className="size-3.5" />
      Replay
    </Button>
  );
}

function SectionLabel({
  children,
  onReplay,
}: {
  children: React.ReactNode;
  onReplay?: () => void;
}) {
  return (
    <div className="mb-3 flex items-center justify-between">
      <p className="text-sm font-medium text-muted-foreground">{children}</p>
      {onReplay && <ReplayButton onClick={onReplay} />}
    </div>
  );
}

/* ── Stagger Cascade ─────────────────────────────────────── */

/*
 * Animations go on a wrapper <div>, NOT on the <Card>.
 * Why: .depth uses transform-style: preserve-3d + translate3d
 * to push walls/shadow behind the face. Opacity animation
 * flattens preserve-3d, causing walls to render over content.
 */

function StaggerCascade() {
  const [key, setKey] = useState(0);

  return (
    <div>
      <SectionLabel onReplay={() => setKey((k) => k + 1)}>
        Stagger cascade &mdash; cards materialize like buildings on the map
      </SectionLabel>
      <div key={key} className="stagger-cascade grid grid-cols-3 gap-4">
        {DEMO_RECIPES.map((recipe, i) => {
          const Icon = recipe.icon;
          return (
            <div
              key={recipe.title}
              className="motion-safe:animate-scale-in"
              style={
                { "--stagger-index": i, "--scale-from": "0.85" } as React.CSSProperties
              }
            >
              <Card className="h-full">
                <Card.Content className="flex flex-col gap-3 p-5">
                  <div className="flex size-10 items-center justify-center rounded-lg bg-primary/10 text-primary">
                    <Icon className="size-5" />
                  </div>
                  <h3 className="font-display text-base font-semibold tracking-tight">
                    {recipe.title}
                  </h3>
                  <p className="text-xs text-muted-foreground">
                    Browser-based, no signup
                  </p>
                </Card.Content>
              </Card>
            </div>
          );
        })}
      </div>
    </div>
  );
}

/* ── Entrance Animations ─────────────────────────────────── */

function EntranceAnimations() {
  const [key, setKey] = useState(0);

  return (
    <div>
      <SectionLabel onReplay={() => setKey((k) => k + 1)}>
        Entrance animations &mdash; springy pop for appearing elements
      </SectionLabel>
      <div key={key} className="grid grid-cols-4 gap-4">
        <div className="motion-safe:animate-fade-in">
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            fade-in
          </Card>
        </div>
        <div className="motion-safe:animate-scale-in">
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            scale-in
          </Card>
        </div>
        <div className="motion-safe:animate-slide-up">
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            slide-up
          </Card>
        </div>
        <div className="motion-safe:animate-slide-down">
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            slide-down
          </Card>
        </div>
      </div>
    </div>
  );
}

/* ── Hero Pop-in ─────────────────────────────────────────── */

function HeroPopIn() {
  const [key, setKey] = useState(0);

  return (
    <div>
      <SectionLabel onReplay={() => setKey((k) => k + 1)}>
        Hero pop-in &mdash; <code className="font-mono text-xs">--scale-from: 0.5</code> for
        dramatic entrance
      </SectionLabel>
      <div key={key} className="flex gap-4">
        <div
          className="motion-safe:animate-scale-in flex-1"
          style={{ "--scale-from": "0.5" } as React.CSSProperties}
        >
          <Card className="flex h-32 items-center justify-center rounded-xl font-display text-lg font-bold">
            Hero Card
          </Card>
        </div>
        <div
          className="motion-safe:animate-scale-in flex-1"
          style={
            {
              "--scale-from": "0.5",
              animationTimingFunction: "var(--ease-spring-bouncy)",
            } as React.CSSProperties
          }
        >
          <Card className="flex h-32 items-center justify-center rounded-xl font-display text-lg font-bold">
            Bouncy Spring
          </Card>
        </div>
        <div
          className="motion-safe:animate-scale-in flex-1"
          style={
            {
              "--scale-from": "0.5",
              animationTimingFunction: "var(--ease-spring-bouncier)",
            } as React.CSSProperties
          }
        >
          <Card className="flex h-32 items-center justify-center rounded-xl font-display text-lg font-bold">
            Bouncier Spring
          </Card>
        </div>
      </div>
    </div>
  );
}

/* ── Bouncy Stagger — the Mini Motorways feel ────────────── */

function BouncyStagger() {
  const [key, setKey] = useState(0);

  return (
    <div>
      <SectionLabel onReplay={() => setKey((k) => k + 1)}>
        Bouncy stagger &mdash; pressable cards bounce onto the map
      </SectionLabel>
      <div key={key} className="stagger-cascade grid grid-cols-3 gap-4">
        {DEMO_RECIPES.map((recipe, i) => {
          const Icon = recipe.icon;
          return (
            <div
              key={recipe.title}
              className="motion-safe:animate-scale-in"
              style={
                {
                  "--stagger-index": i,
                  "--scale-from": "0.6",
                  animationTimingFunction: "var(--ease-spring-bouncier)",
                } as React.CSSProperties
              }
            >
              <Button
                variant="outline"
                className="h-auto w-full flex-col items-start gap-3 rounded-xl p-5"
              >
                <div className="flex size-10 items-center justify-center rounded-lg bg-primary/10 text-primary">
                  <Icon className="size-5" />
                </div>
                <h3 className="font-display text-base font-semibold tracking-tight">
                  {recipe.title}
                </h3>
                <p className="text-xs text-muted-foreground">
                  Browser-based, no signup
                </p>
              </Button>
            </div>
          );
        })}
      </div>
    </div>
  );
}

/* ── Emphasis ────────────────────────────────────────────── */

function EmphasisAnimations() {
  return (
    <div>
      <SectionLabel>
        Emphasis &mdash; calm looping animations for ambient presence
      </SectionLabel>
      <div className="grid grid-cols-2 gap-4">
        <div className="motion-safe:animate-pulse-soft">
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            pulse-soft
          </Card>
        </div>
        <div className="motion-safe:animate-breathe">
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            breathe
          </Card>
        </div>
      </div>
    </div>
  );
}

/* ── Root ────────────────────────────────────────────────── */

export function AnimationShowcase() {
  return (
    <div className="space-y-10">
      <StaggerCascade />
      <EntranceAnimations />
      <HeroPopIn />
      <EmphasisAnimations />
      <BouncyStagger />
    </div>
  );
}
