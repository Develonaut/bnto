"use client";

import { useState } from "react";
import { cn } from "@/lib/utils";
import {
  PopIn,
  SlideIn,
  StaggerCascade,
  Breathe,
  PathDraw,
  LayoutShift,
  ExitDissolve,
  NumberRoll,
  PresenceGate,
} from "@/components/animations";
import { Button } from "@/components/ui/button";

function DemoCard({
  children,
  className,
  ...props
}: React.HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      className={cn(
        "rounded-lg bg-primary p-6 text-center font-display font-semibold text-primary-foreground",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
}

function Section({
  id,
  title,
  children,
}: {
  id: string;
  title: string;
  children: React.ReactNode;
}) {
  return (
    <section data-testid={id} className="space-y-4">
      <h2 className="font-display text-lg font-semibold text-foreground">
        {title}
      </h2>
      {children}
    </section>
  );
}

export default function AnimationsPage() {
  const [showPresence, setShowPresence] = useState(true);
  const [layoutExpanded, setLayoutExpanded] = useState(false);
  const [counter, setCounter] = useState(42);

  return (
    <div className="mx-auto max-w-3xl space-y-12 px-6 pb-8 pt-24 lg:pt-32">
      <div>
        <h1 className="font-display text-3xl font-bold text-foreground">
          Animation Showcase
        </h1>
        <p className="mt-2 text-muted-foreground">
          Test harness for animation primitives. Each section demonstrates a
          component with its default configuration.
        </p>
      </div>

      <Section id="demo-pop-in" title="PopIn">
        <PopIn>
          <DemoCard>PopIn Card</DemoCard>
        </PopIn>
      </Section>

      <Section id="demo-slide-in" title="SlideIn">
        <div className="grid grid-cols-2 gap-4">
          <SlideIn from="left">
            <DemoCard className="bg-secondary text-secondary-foreground">
              From Left
            </DemoCard>
          </SlideIn>
          <SlideIn from="right">
            <DemoCard className="bg-secondary text-secondary-foreground">
              From Right
            </DemoCard>
          </SlideIn>
          <SlideIn from="top">
            <DemoCard className="bg-secondary text-secondary-foreground">
              From Top
            </DemoCard>
          </SlideIn>
          <SlideIn from="bottom">
            <DemoCard className="bg-secondary text-secondary-foreground">
              From Bottom
            </DemoCard>
          </SlideIn>
        </div>
      </Section>

      <Section id="demo-stagger" title="StaggerCascade + PopIn">
        <StaggerCascade className="grid grid-cols-3 gap-3">
          {Array.from({ length: 6 }, (_, i) => (
            <PopIn key={i}>
              <DemoCard className="bg-accent text-accent-foreground">
                Item {i + 1}
              </DemoCard>
            </PopIn>
          ))}
        </StaggerCascade>
      </Section>

      <Section id="demo-breathe" title="Breathe">
        <Breathe>
          <DemoCard className="flex h-28 w-28 items-center justify-center rounded-full">
            Pulse
          </DemoCard>
        </Breathe>
      </Section>

      <Section id="demo-path-draw" title="PathDraw">
        <div className="rounded-lg border border-border bg-card p-6">
          <svg
            width="300"
            height="80"
            viewBox="0 0 300 80"
            fill="none"
            className="text-primary"
          >
            <PathDraw
              d="M 10 70 Q 75 10 150 40 T 290 20"
              stroke="currentColor"
              strokeWidth={3}
            />
          </svg>
        </div>
      </Section>

      <Section id="demo-layout" title="LayoutShift">
        <button
          data-testid="layout-toggle"
          onClick={() => setLayoutExpanded((v) => !v)}
          className="rounded-md bg-muted px-3 py-1.5 text-sm font-medium text-foreground"
        >
          {layoutExpanded ? "Collapse" : "Expand"}
        </button>
        <LayoutShift>
          <DemoCard className={layoutExpanded ? "p-12" : "p-6"}>
            {layoutExpanded ? "Expanded Content" : "Compact"}
          </DemoCard>
        </LayoutShift>
      </Section>

      <Section id="demo-presence" title="PresenceGate + ExitDissolve">
        <button
          data-testid="presence-toggle"
          onClick={() => setShowPresence((v) => !v)}
          className="rounded-md bg-muted px-3 py-1.5 text-sm font-medium text-foreground"
        >
          {showPresence ? "Hide" : "Show"}
        </button>
        <PresenceGate>
          {showPresence && (
            <ExitDissolve>
              <PopIn>
                <DemoCard>Now you see me</DemoCard>
              </PopIn>
            </ExitDissolve>
          )}
        </PresenceGate>
      </Section>

      <Section id="demo-number-roll" title="NumberRoll">
        <div className="flex items-center gap-4">
          <button
            data-testid="counter-increment"
            onClick={() => setCounter((c) => c + 10)}
            className="rounded-md bg-muted px-3 py-1.5 text-sm font-medium text-foreground"
          >
            +10
          </button>
          <span className="font-display text-4xl font-bold text-foreground">
            <NumberRoll value={counter} />
          </span>
        </div>
      </Section>

      <Section id="demo-press" title="Pressable Button">
        <Button
          variant="default"
          size="lg"
          data-testid="press-target"
          className="h-auto rounded-lg px-8 py-6 font-display font-semibold"
        >
          Hover or press me
        </Button>
      </Section>
    </div>
  );
}
