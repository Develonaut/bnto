"use client";

import { useState } from "react";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Switch } from "@/components/ui/switch";
import { Checkbox } from "@/components/ui/checkbox";
import { Slider } from "@/components/ui/slider";
import { Dropzone } from "@/components/ui/dropzone";
import { Press, PopIn, SlideIn, StaggerCascade } from "@/components/animations";
import { Upload } from "lucide-react";

/* ------------------------------------------------------------------ */
/*  Shadow Showcase                                                    */
/* ------------------------------------------------------------------ */

function ShadowScale() {
  const levels = [
    { name: "shadow-2xs", cls: "shadow-2xs" },
    { name: "shadow-xs", cls: "shadow-xs" },
    { name: "shadow-sm", cls: "shadow-sm" },
    { name: "shadow", cls: "shadow" },
    { name: "shadow-md", cls: "shadow-md" },
    { name: "shadow-lg", cls: "shadow-lg" },
    { name: "shadow-xl", cls: "shadow-xl" },
    { name: "shadow-2xl", cls: "shadow-2xl" },
  ];

  return (
    <div className="grid grid-cols-4 gap-6">
      {levels.map(({ name, cls }) => (
        <div
          key={name}
          className={cn(
            "flex h-24 items-center justify-center rounded-xl bg-card",
            cls,
          )}
        >
          <span className="font-mono text-xs text-muted-foreground">
            {name}
          </span>
        </div>
      ))}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Depth Scale (::before pseudo-element system)                       */
/* ------------------------------------------------------------------ */

function DepthScale() {
  const levels = [
    { name: "depth-2xs", cls: "depth-2xs" },
    { name: "depth-xs", cls: "depth-xs" },
    { name: "depth-sm", cls: "depth-sm" },
    { name: "depth", cls: "depth" },
    { name: "depth-md", cls: "depth-md" },
    { name: "depth-lg", cls: "depth-lg" },
    { name: "depth-xl", cls: "depth-xl" },
    { name: "depth-2xl", cls: "depth-2xl" },
  ];

  return (
    <div className="grid grid-cols-4 gap-6">
      {levels.map(({ name, cls }) => (
        <div
          key={name}
          className={cn(
            "flex h-24 items-center justify-center rounded-xl bg-card",
            cls,
          )}
        >
          <span className="font-mono text-xs text-muted-foreground">
            {name}
          </span>
        </div>
      ))}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Press + Shadow Interaction                                         */
/* ------------------------------------------------------------------ */

function PressShowcase() {
  const levels = [
    { name: "2xs", depth: 1 },
    { name: "xs", depth: 2 },
    { name: "sm", depth: 3 },
    { name: "default", depth: 4 },
    { name: "md", depth: 5 },
    { name: "lg", depth: 7 },
    { name: "xl", depth: 10 },
    { name: "2xl", depth: 14 },
  ];

  return (
    <div className="grid grid-cols-4 gap-6">
      {levels.map(({ name, depth }) => (
        <Press key={name} depth={depth} data-testid={`press-${name}`}>
          <Card className="select-none p-6 text-center">
            <Card.Title className="font-display text-sm">
              shadow-{name}
            </Card.Title>
            <Card.Description className="mt-1 text-xs">
              depth={depth}
            </Card.Description>
          </Card>
        </Press>
      ))}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Elevation Cards                                                    */
/* ------------------------------------------------------------------ */

function ElevationCards() {
  return (
    <div className="grid grid-cols-3 gap-6">
      <Card className="shadow-sm">
        <Card.Header>
          <Card.Title className="font-display">Flush</Card.Title>
          <Card.Description>
            shadow-sm &mdash; sitting on the surface, barely lifted
          </Card.Description>
        </Card.Header>
        <Card.Content>
          <p className="text-sm text-muted-foreground">
            Used for inputs, form fields, and subtle containers.
          </p>
        </Card.Content>
      </Card>

      <Card className="shadow-md">
        <Card.Header>
          <Card.Title className="font-display">Standard</Card.Title>
          <Card.Description>
            shadow-md &mdash; default card elevation
          </Card.Description>
        </Card.Header>
        <Card.Content>
          <p className="text-sm text-muted-foreground">
            Used for content cards, panels, and list items.
          </p>
        </Card.Content>
      </Card>

      <Card className="shadow-xl">
        <Card.Header>
          <Card.Title className="font-display">Floating</Card.Title>
          <Card.Description>
            shadow-xl &mdash; high above the surface
          </Card.Description>
        </Card.Header>
        <Card.Content>
          <p className="text-sm text-muted-foreground">
            Used for Press resting state, modals, featured elements.
          </p>
        </Card.Content>
      </Card>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Color Swatches                                                     */
/* ------------------------------------------------------------------ */

function ColorSwatches() {
  const swatches = [
    { name: "Primary", bg: "bg-primary", fg: "text-primary-foreground" },
    { name: "Secondary", bg: "bg-secondary", fg: "text-secondary-foreground" },
    { name: "Accent", bg: "bg-accent", fg: "text-accent-foreground" },
    { name: "Muted", bg: "bg-muted", fg: "text-muted-foreground" },
    { name: "Destructive", bg: "bg-destructive", fg: "text-destructive-foreground" },
    { name: "Card", bg: "bg-card", fg: "text-card-foreground" },
  ];

  return (
    <div className="grid grid-cols-6 gap-4">
      {swatches.map(({ name, bg, fg }) => (
        <div
          key={name}
          className={cn(
            "flex h-20 items-center justify-center rounded-xl shadow-md",
            bg,
            fg,
          )}
        >
          <span className="text-sm font-semibold">{name}</span>
        </div>
      ))}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Button Depth States                                                */
/* ------------------------------------------------------------------ */

const DEPTH_VARIANTS = [
  { variant: "default" as const, label: "Primary" },
  { variant: "secondary" as const, label: "Secondary" },
  { variant: "outline" as const, label: "Outline" },
  { variant: "destructive" as const, label: "Destructive" },
];

const FORCE_STATES = [
  { state: undefined, label: "Resting" },
  { state: "hover" as const, label: "Hover" },
  { state: "active" as const, label: "Active" },
];

function ButtonDepthStates() {
  return (
    <div className="space-y-4">
      {/* Column headers */}
      <div className="grid grid-cols-[8rem_1fr_1fr_1fr] items-center gap-3">
        <span />
        {FORCE_STATES.map(({ label }) => (
          <span key={label} className="text-center font-mono text-xs text-muted-foreground">
            {label}
          </span>
        ))}
      </div>
      {/* Rows — one per variant */}
      {DEPTH_VARIANTS.map(({ variant, label }) => (
        <div key={variant} className="grid grid-cols-[8rem_1fr_1fr_1fr] items-center gap-3">
          <span className="font-mono text-xs text-muted-foreground">{label}</span>
          {FORCE_STATES.map(({ state, label: stateLabel }) => (
            <div key={stateLabel} className="flex justify-center">
              <Button
                variant={variant}
                size="lg"
                data-force-state={state}
              >
                {label}
              </Button>
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Katherine Kato Pressable Button (reference implementation)         */
/* ------------------------------------------------------------------ */

function KatherinePressable() {
  return (
    <div className="flex flex-row items-start gap-6">
      <div className="text-center">
        <span className="block font-mono text-xs text-muted-foreground mb-2">Interactive</span>
        <button className="pressable">Learn More</button>
      </div>
      <div className="text-center">
        <span className="block font-mono text-xs text-muted-foreground mb-2">Resting</span>
        <button className="pressable">Learn More</button>
      </div>
      <div className="text-center">
        <span className="block font-mono text-xs text-muted-foreground mb-2">Hover</span>
        <button className="pressable" data-force-state="hover">Learn More</button>
      </div>
      <div className="text-center">
        <span className="block font-mono text-xs text-muted-foreground mb-2">Active</span>
        <button className="pressable" data-force-state="active">Learn More</button>
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Button Variants                                                    */
/* ------------------------------------------------------------------ */

function ButtonShowcase() {
  return (
    <div className="space-y-4">
      <div className="flex flex-wrap items-center gap-3">
        <Button>Primary</Button>
        <Button variant="secondary">Secondary</Button>
        <Button variant="outline">Outline</Button>
        <Button variant="ghost">Ghost</Button>
        <Button variant="destructive">Destructive</Button>
        <Button variant="link">Link</Button>
      </div>
      <div className="flex flex-wrap items-center gap-3">
        <Button size="sm">Small</Button>
        <Button size="default">Default</Button>
        <Button size="lg">Large</Button>
      </div>
      {/* Buttons with Press */}
      <div className="flex flex-wrap items-center gap-3">
        <Press depth={3} className="rounded-md">
          <Button>Press Primary</Button>
        </Press>
        <Press depth={3} className="rounded-md">
          <Button variant="secondary">Press Secondary</Button>
        </Press>
        <Press depth={3} className="rounded-md">
          <Button variant="outline">Press Outline</Button>
        </Press>
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Form Elements                                                      */
/* ------------------------------------------------------------------ */

function FormShowcase() {
  const [switchOn, setSwitchOn] = useState(false);
  const [sliderVal, setSliderVal] = useState([50]);

  return (
    <Card className="shadow-lg">
      <Card.Header>
        <Card.Title className="font-display">Form Elements</Card.Title>
        <Card.Description>
          Inputs, switches, checkboxes, and controls with shadow integration
        </Card.Description>
      </Card.Header>
      <Card.Content className="space-y-6">
        <div className="grid grid-cols-2 gap-6">
          <div className="space-y-2">
            <Label htmlFor="name">Name</Label>
            <Input id="name" placeholder="Enter your name" />
          </div>
          <div className="space-y-2">
            <Label htmlFor="email">Email</Label>
            <Input id="email" type="email" placeholder="you@example.com" />
          </div>
        </div>

        <div className="space-y-2">
          <Label htmlFor="bio">Bio</Label>
          <Textarea id="bio" placeholder="Tell us about yourself..." />
        </div>

        <div className="flex items-center gap-8">
          <div className="flex items-center gap-2">
            <Switch checked={switchOn} onCheckedChange={setSwitchOn} />
            <Label>Notifications {switchOn ? "on" : "off"}</Label>
          </div>
          <div className="flex items-center gap-2">
            <Checkbox id="terms" />
            <Label htmlFor="terms">Accept terms</Label>
          </div>
        </div>

        <div className="space-y-2">
          <Label>Quality: {sliderVal[0]}%</Label>
          <Slider value={sliderVal} onValueChange={setSliderVal} max={100} />
        </div>
      </Card.Content>
      <Card.Footer className="gap-3">
        <Button>Save changes</Button>
        <Button variant="outline">Cancel</Button>
      </Card.Footer>
    </Card>
  );
}

/* ------------------------------------------------------------------ */
/*  Dropzone                                                           */
/* ------------------------------------------------------------------ */

function DropzoneShowcase() {
  return (
    <Press depth={4} className="rounded-lg">
      <Dropzone accept={{ "image/*": [] }}>
        {({ isDragActive }) => (
          <div className="flex flex-col items-center gap-2 text-muted-foreground">
            <Upload className="size-8" />
            <p className="text-sm font-medium">
              {isDragActive ? "Drop files here" : "Drag images here or click to browse"}
            </p>
            <p className="text-xs">PNG, JPEG, WebP up to 10MB</p>
          </div>
        )}
      </Dropzone>
    </Press>
  );
}

/* ------------------------------------------------------------------ */
/*  Typography                                                         */
/* ------------------------------------------------------------------ */

function TypographyShowcase() {
  return (
    <div className="space-y-4">
      <h1 className="font-display text-4xl font-bold tracking-tight text-foreground">
        Display Heading
      </h1>
      <h2 className="font-display text-2xl font-semibold text-foreground">
        Page Heading
      </h2>
      <h3 className="font-display text-lg font-semibold text-foreground">
        Section Heading
      </h3>
      <p className="text-base text-foreground">
        Body text in Inter. Clean and highly legible at all sizes. The warm
        terracotta palette carries through every element.
      </p>
      <p className="text-sm text-muted-foreground">
        Secondary text for descriptions, labels, and metadata.
      </p>
      <p className="font-mono text-xs uppercase tracking-wider text-muted-foreground">
        Monospace technical output
      </p>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Animated Card Grid                                                 */
/* ------------------------------------------------------------------ */

function AnimatedCardGrid() {
  const items = [
    { title: "Compress Images", color: "bg-primary text-primary-foreground" },
    { title: "Clean CSV", color: "bg-secondary text-secondary-foreground" },
    { title: "Rename Files", color: "bg-accent text-accent-foreground" },
    { title: "Convert Format", color: "bg-primary text-primary-foreground" },
    { title: "Call API", color: "bg-secondary text-secondary-foreground" },
    { title: "Resize Images", color: "bg-accent text-accent-foreground" },
  ];

  return (
    <StaggerCascade className="grid grid-cols-3 gap-4">
      {items.map((item) => (
        <PopIn key={item.title}>
          <Press depth={5} className="rounded-xl">
            <div
              className={cn(
                "flex h-28 cursor-pointer items-center justify-center rounded-xl font-display font-semibold",
                item.color,
              )}
            >
              {item.title}
            </div>
          </Press>
        </PopIn>
      ))}
    </StaggerCascade>
  );
}

/* ------------------------------------------------------------------ */
/*  Notification / Toast Cards                                         */
/* ------------------------------------------------------------------ */

function NotificationCards() {
  return (
    <div className="space-y-3">
      <Press depth={3}>
        <Card className="flex items-center gap-4 p-4">
          <div className="flex size-10 items-center justify-center rounded-full bg-primary text-primary-foreground font-display font-bold">
            B
          </div>
          <div className="flex-1">
            <p className="text-sm font-semibold text-foreground">
              Workflow completed
            </p>
            <p className="text-xs text-muted-foreground">
              compress-images finished in 2.4s
            </p>
          </div>
          <span className="text-xs text-muted-foreground">2m ago</span>
        </Card>
      </Press>

      <Press depth={3}>
        <Card className="flex items-center gap-4 p-4">
          <div className="flex size-10 items-center justify-center rounded-full bg-accent text-accent-foreground font-display font-bold">
            3
          </div>
          <div className="flex-1">
            <p className="text-sm font-semibold text-foreground">
              3 files processed
            </p>
            <p className="text-xs text-muted-foreground">
              resize-images batch complete
            </p>
          </div>
          <span className="text-xs text-muted-foreground">5m ago</span>
        </Card>
      </Press>

      <Press depth={3}>
        <Card className="flex items-center gap-4 p-4">
          <div className="flex size-10 items-center justify-center rounded-full bg-destructive text-destructive-foreground font-display font-bold">
            !
          </div>
          <div className="flex-1">
            <p className="text-sm font-semibold text-foreground">
              Execution failed
            </p>
            <p className="text-xs text-muted-foreground">
              call-api returned 503 &mdash; retry available
            </p>
          </div>
          <span className="text-xs text-muted-foreground">12m ago</span>
        </Card>
      </Press>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Page Layout                                                        */
/* ------------------------------------------------------------------ */

function Section({
  id,
  title,
  description,
  children,
}: {
  id: string;
  title: string;
  description?: string;
  children: React.ReactNode;
}) {
  return (
    <section data-testid={id} className="space-y-4">
      <div>
        <h2 className="font-display text-xl font-semibold text-foreground">
          {title}
        </h2>
        {description && (
          <p className="mt-1 text-sm text-muted-foreground">{description}</p>
        )}
      </div>
      {children}
    </section>
  );
}

export default function ThemeDemoPage() {
  return (
    <div className="mx-auto max-w-4xl space-y-16 px-6 pb-8 pt-24 lg:pt-32">
      <div>
        <h1 className="font-display text-3xl font-bold text-foreground">
          Theme Demo
        </h1>
        <p className="mt-2 text-muted-foreground">
          Visual showcase for the Mini Motorways depth theme. Every shadow on
          this page is driven by CSS tokens &mdash; swap the stylesheet and
          shadows go from centered to directional.
        </p>
      </div>

      <Section
        id="demo-shadow-scale"
        title="Shadow Scale"
        description="All 8 shadow levels. With the depth theme, each casts to the bottom-right like buildings in Mini Motorways."
      >
        <ShadowScale />
      </Section>

      <Section
        id="demo-depth-scale"
        title="Depth Scale"
        description="::before pseudo-element depth system. Same colors as shadow scale, rendered via pseudo-elements for composability with Press."
      >
        <DepthScale />
      </Section>

      <Section
        id="demo-elevation"
        title="Elevation Cards"
        description="Cards at different heights above the surface. The directional shadow makes the floating effect tangible."
      >
        <ElevationCards />
      </Section>

      <Section
        id="demo-press"
        title="Press + Shadow"
        description="Hover to sink. The directional shadow collapses as the element presses flush with the surface."
      >
        <PressShowcase />
      </Section>

      <Section
        id="demo-colors"
        title="Color Palette"
        description="Terracotta, teal, golden, cream — the warm Mini Motorways palette."
      >
        <ColorSwatches />
      </Section>

      <Section id="demo-typography" title="Typography" description="DM Sans display + Inter body + Geist Mono code.">
        <TypographyShowcase />
      </Section>

      <Section
        id="demo-button-depth"
        title="Button Depth States"
        description="Forced pseudo-states via data-force-state. Resting, hover, and active side by side for depth tuning."
      >
        <ButtonDepthStates />
      </Section>

      <Section
        id="demo-pressable"
        title="Pressable Button (Katherine Kato)"
        description="Pure CSS 3D press effect. The button face sinks toward the surface on hover/active while the ::before edge and ground shadow collapse."
      >
        <KatherinePressable />
      </Section>

      <Section id="demo-buttons" title="Buttons" description="All variants, sizes, and with Press animation.">
        <ButtonShowcase />
      </Section>

      <Section id="demo-form" title="Form Elements" description="Inputs, switches, checkboxes, slider inside an elevated card.">
        <FormShowcase />
      </Section>

      <Section id="demo-dropzone" title="Dropzone" description="File upload area with Press depth effect.">
        <DropzoneShowcase />
      </Section>

      <Section
        id="demo-tool-grid"
        title="Animated Tool Grid"
        description="StaggerCascade + PopIn + Press. The signature bnto interaction."
      >
        <AnimatedCardGrid />
      </Section>

      <Section
        id="demo-notifications"
        title="Notification Cards"
        description="List items with Press — hover to push them into the surface."
      >
        <NotificationCards />
      </Section>
    </div>
  );
}
