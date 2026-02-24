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
import { Upload, ArrowRight } from "lucide-react";

import { AnimationShowcase } from "./AnimationShowcase";


/* ------------------------------------------------------------------ */
/*  Cards — static depth + pressable                                   */
/* ------------------------------------------------------------------ */

function CardShowcase() {
  const sizes: { label: string; depth: "none" | "sm" | "md" | "lg" }[] = [
    { label: "depth-none", depth: "none" },
    { label: "depth-sm", depth: "sm" },
    { label: "depth-md (default)", depth: "md" },
    { label: "depth-lg", depth: "lg" },
  ];

  return (
    <div className="space-y-8">
      {/* Row 1: Static depth cards (no pressable interaction) */}
      <div>
        <p className="mb-3 text-sm font-medium text-muted-foreground">
          Static depth &mdash; walls + ground shadow, no interaction
        </p>
        <div className="grid grid-cols-4 gap-6">
          {sizes.map(({ label, depth }) => (
            <Card
              key={label}
              depth={depth}
              className="flex h-24 items-center justify-center rounded-xl font-display font-semibold"
              data-testid={`static-card-${label}`}
            >
              {label}
            </Card>
          ))}
        </div>
      </div>

      {/* Row 2: Pressable cards (hover to sink, active to flush) */}
      <div>
        <p className="mb-3 text-sm font-medium text-muted-foreground">
          Pressable &mdash; hover to sink, active pushes flush
        </p>
        <div className="grid grid-cols-4 gap-6">
          {sizes.map(({ label, depth }) => (
            <Card
              key={label}
              depth={depth}
              className="flex h-24 cursor-pointer items-center justify-center rounded-xl font-display font-semibold pressable"
              data-testid={`pressable-card-${label}`}
            >
              {label}
            </Card>
          ))}
        </div>
      </div>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Color Swatches                                                     */
/* ------------------------------------------------------------------ */

function ColorSwatches() {
  const swatches = [
    { name: "Primary", bg: "bg-primary", fg: "text-primary-foreground", depth: "depth-default" },
    { name: "Secondary", bg: "bg-secondary", fg: "text-secondary-foreground", depth: "depth-secondary" },
    { name: "Accent", bg: "bg-accent", fg: "text-accent-foreground", depth: "depth-accent" },
    { name: "Muted", bg: "bg-muted", fg: "text-muted-foreground", depth: "depth-muted" },
    { name: "Destructive", bg: "bg-destructive", fg: "text-destructive-foreground", depth: "depth-destructive" },
    { name: "Success", bg: "bg-success", fg: "text-success-foreground", depth: "depth-success" },
    { name: "Warning", bg: "bg-warning", fg: "text-warning-foreground", depth: "depth-warning" },
    { name: "Card", bg: "bg-card", fg: "text-card-foreground", depth: "depth-outline" },
  ];

  return (
    <div className="grid grid-cols-4 gap-4">
      {swatches.map(({ name, bg, fg, depth }) => (
        <Card
          key={name}
          className={cn(
            "flex h-20 items-center justify-center rounded-xl",
            bg,
            fg,
            depth,
          )}
        >
          <span className="text-sm font-semibold">{name}</span>
        </Card>
      ))}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Pressable Buttons (all variants × sizes)                           */
/* ------------------------------------------------------------------ */

function ButtonRow({
  variant,
  label,
}: {
  variant: "default" | "secondary" | "muted" | "destructive" | "success" | "warning" | "outline";
  label: string;
}) {
  return (
    <div className="flex flex-row items-center gap-6">
      <span className="font-mono text-xs text-muted-foreground w-24 shrink-0">
        {label}
      </span>
      <Button variant={variant} size="icon">
        <ArrowRight />
      </Button>
      <Button variant={variant} size="sm">
        Learn More
      </Button>
      <Button variant={variant} size="default">
        Learn More
      </Button>
      <Button variant={variant} size="lg" data-testid={`pressable-${label.toLowerCase()}-lg`}>
        Learn More
      </Button>
    </div>
  );
}

function ButtonShowcase() {
  return (
    <div className="space-y-8">
      <ButtonRow variant="default" label="Default" />
      <ButtonRow variant="secondary" label="Secondary" />
      <ButtonRow variant="outline" label="Outline" />
      <ButtonRow variant="muted" label="Muted" />
      <ButtonRow variant="destructive" label="Destructive" />
      <ButtonRow variant="success" label="Success" />
      <ButtonRow variant="warning" label="Warning" />
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
/*  Tool Grid                                                          */
/* ------------------------------------------------------------------ */

function ToolGrid() {
  const items = [
    { title: "Compress Images", variant: "default" as const },
    { title: "Clean CSV", variant: "secondary" as const },
    { title: "Rename Files", variant: "muted" as const },
    { title: "Convert Format", variant: "default" as const },
    { title: "Call API", variant: "secondary" as const },
    { title: "Resize Images", variant: "muted" as const },
  ];

  return (
    <div className="grid grid-cols-3 gap-4">
      {items.map((item) => (
        <Button
          key={item.title}
          variant={item.variant}
          className="h-28 w-full rounded-xl font-display font-semibold"
        >
          {item.title}
        </Button>
      ))}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Notification / Toast Cards                                         */
/* ------------------------------------------------------------------ */

function NotificationCards() {
  return (
    <div className="space-y-3">
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
        id="demo-cards"
        title="Cards"
        description="Depth correlates with perspective: small elements are nearly top-down, larger elements are more isometric. Add .pressable for hover/active interaction."
      >
        <CardShowcase />
      </Section>

      <Section
        id="demo-animations"
        title="Animations"
        description="Mini Motorways motion language. Entrances are springy (spring easing via CSS linear()). Transitions are smooth (ease-out cubic-bezier). Hit Replay to re-trigger."
      >
        <AnimationShowcase />
      </Section>

      <Section
        id="demo-colors"
        title="Colors"
        description="Terracotta, teal, golden, cream — the warm Mini Motorways palette."
      >
        <ColorSwatches />
      </Section>

      <Section id="demo-typography" title="Typography" description="DM Sans display + Inter body + Geist Mono code.">
        <TypographyShowcase />
      </Section>

      <Section
        id="demo-buttons"
        title="Buttons"
        description="All variants × sizes with depth + pressable. Icon buttons included."
      >
        <ButtonShowcase />
      </Section>

      <Section id="demo-form" title="Form Elements" description="Inputs, switches, checkboxes, slider inside an elevated card.">
        <FormShowcase />
      </Section>

      <Section id="demo-dropzone" title="Dropzone" description="File upload area with card depth.">
        <DropzoneShowcase />
      </Section>

      <Section
        id="demo-tool-grid"
        title="Tool Grid"
        description="Pressable Buttons in a grid layout."
      >
        <ToolGrid />
      </Section>

      <Section
        id="demo-notifications"
        title="Notification Cards"
        description="List items with static card depth — walls + ground shadow."
      >
        <NotificationCards />
      </Section>
    </div>
  );
}
