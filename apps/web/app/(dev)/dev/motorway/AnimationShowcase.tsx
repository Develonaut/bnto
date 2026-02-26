"use client";

import { useState } from "react";
import {
  Minimize2Icon,
  ScalingIcon,
  ArrowRightLeftIcon,
  PenLineIcon,
  SparklesIcon,
  Columns3Icon,
  RotateCcwIcon,
} from "@/components/ui/icons";

import { Animate } from "@/components/ui/Animate";
import { Button } from "@/components/ui/Button";
import { Card } from "@/components/ui/Card";

/* ── Helpers ─────────────────────────────────────────────── */

const DEMO_RECIPES = [
  { title: "Compress Images", icon: Minimize2Icon },
  { title: "Resize Images", icon: ScalingIcon },
  { title: "Convert Format", icon: ArrowRightLeftIcon },
  { title: "Rename Files", icon: PenLineIcon },
  { title: "Clean CSV", icon: SparklesIcon },
  { title: "Rename Columns", icon: Columns3Icon },
];

function ReplayButton({ onClick }: { onClick: () => void }) {
  return (
    <Button variant="outline" size="sm" onClick={onClick} depth={false}>
      <RotateCcwIcon className="size-3.5" />
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
        Stagger cascade. Cards materialize like buildings on the map
      </SectionLabel>
      <Animate.Stagger key={key} className="grid grid-cols-3 gap-4">
        {DEMO_RECIPES.map((recipe, i) => {
          const Icon = recipe.icon;
          return (
            <Animate.ScaleIn key={recipe.title} index={i} from={0.85}>
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
            </Animate.ScaleIn>
          );
        })}
      </Animate.Stagger>
    </div>
  );
}

/* ── Entrance Animations ─────────────────────────────────── */

function EntranceAnimations() {
  const [key, setKey] = useState(0);

  return (
    <div>
      <SectionLabel onReplay={() => setKey((k) => k + 1)}>
        Entrance animations. Springy pop for appearing elements
      </SectionLabel>
      <div key={key} className="grid grid-cols-4 gap-4">
        <Animate.FadeIn>
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            fade-in
          </Card>
        </Animate.FadeIn>
        <Animate.ScaleIn>
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            scale-in
          </Card>
        </Animate.ScaleIn>
        <Animate.SlideUp>
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            slide-up
          </Card>
        </Animate.SlideUp>
        <Animate.SlideDown>
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            slide-down
          </Card>
        </Animate.SlideDown>
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
        Hero pop-in. <code className="font-mono text-xs">from=0.5</code>{" "}
        for dramatic entrance
      </SectionLabel>
      <div key={key} className="flex gap-4">
        <Animate.ScaleIn from={0.5} className="flex-1">
          <Card className="flex h-32 items-center justify-center rounded-xl font-display text-lg font-bold">
            Hero Card
          </Card>
        </Animate.ScaleIn>
        <Animate.ScaleIn from={0.5} easing="spring-bouncy" className="flex-1">
          <Card className="flex h-32 items-center justify-center rounded-xl font-display text-lg font-bold">
            Bouncy Spring
          </Card>
        </Animate.ScaleIn>
        <Animate.ScaleIn from={0.5} easing="spring-bouncier" className="flex-1">
          <Card className="flex h-32 items-center justify-center rounded-xl font-display text-lg font-bold">
            Bouncier Spring
          </Card>
        </Animate.ScaleIn>
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
        Bouncy stagger. Pressable cards bounce onto the map
      </SectionLabel>
      <Animate.Stagger key={key} className="grid grid-cols-3 gap-4">
        {DEMO_RECIPES.map((recipe, i) => {
          const Icon = recipe.icon;
          return (
            <Animate.ScaleIn
              key={recipe.title}
              index={i}
              from={0.6}
              easing="spring-bouncy"
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
            </Animate.ScaleIn>
          );
        })}
      </Animate.Stagger>
    </div>
  );
}

/* ── Emphasis ────────────────────────────────────────────── */

function EmphasisAnimations() {
  return (
    <div>
      <SectionLabel>
        Emphasis. Calm looping animations for ambient presence
      </SectionLabel>
      <div className="grid grid-cols-2 gap-4">
        <Animate.PulseSoft>
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            pulse-soft
          </Card>
        </Animate.PulseSoft>
        <Animate.Breathe>
          <Card className="flex h-24 items-center justify-center rounded-xl font-display text-sm font-semibold">
            breathe
          </Card>
        </Animate.Breathe>
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
