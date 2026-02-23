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
import { PopIn, SlideIn, StaggerCascade } from "@/components/animations";
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
/*  Depth Scale (3-layer building system)                              */
/* ------------------------------------------------------------------ */

function DepthScale() {
  const levels = [
    { name: "depth-sm (4px)", cls: "depth depth-sm" },
    { name: "depth (6px)", cls: "depth" },
    { name: "depth-lg (7px)", cls: "depth depth-lg" },
  ];

  return (
    <div className="grid grid-cols-3 gap-6">
      {levels.map(({ name, cls }) => (
        <div
          key={name}
          className={cn(
            "flex h-24 items-center justify-center rounded-xl border bg-card",
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
/*  Pressable Cards                                                    */
/* ------------------------------------------------------------------ */

function PressableCards() {
  const sizes = [
    { label: "depth-sm (4px)", size: "sm" as const },
    { label: "depth-md (6px)", size: "default" as const },
    { label: "depth-lg (7px)", size: "lg" as const },
  ];

  return (
    <div className="grid grid-cols-3 gap-6">
      {sizes.map(({ label, size }) => (
        <Button
          key={label}
          variant="muted"
          size={size}
          className="h-24 rounded-xl font-display"
          data-testid={`pressable-card-${size}`}
        >
          {label}
        </Button>
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
            Used for modals, featured elements, hero cards.
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
/*  Pressable Buttons (all variants × sizes)                           */
/* ------------------------------------------------------------------ */

function PressableRow({
  variant,
  label,
}: {
  variant: "default" | "secondary" | "muted" | "destructive";
  label: string;
}) {
  return (
    <div className="flex flex-row items-center gap-6">
      <span className="font-mono text-xs text-muted-foreground w-24 shrink-0">
        {label}
      </span>
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

function PressableButtons() {
  return (
    <div className="space-y-8">
      <PressableRow variant="default" label="Default" />
      <PressableRow variant="secondary" label="Secondary" />
      <PressableRow variant="muted" label="Muted" />
      <PressableRow variant="destructive" label="Destructive" />
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
        <Button variant="muted">Muted</Button>
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
/*  Animated Card Grid                                                 */
/* ------------------------------------------------------------------ */

function AnimatedCardGrid() {
  const items = [
    { title: "Compress Images", variant: "default" as const },
    { title: "Clean CSV", variant: "secondary" as const },
    { title: "Rename Files", variant: "muted" as const },
    { title: "Convert Format", variant: "default" as const },
    { title: "Call API", variant: "secondary" as const },
    { title: "Resize Images", variant: "muted" as const },
  ];

  return (
    <StaggerCascade className="grid grid-cols-3 gap-4">
      {items.map((item) => (
        <PopIn key={item.title}>
          <Button
            variant={item.variant}
            className="h-28 w-full rounded-xl font-display font-semibold"
          >
            {item.title}
          </Button>
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
        id="demo-shadow-scale"
        title="Shadow Scale"
        description="All 8 shadow levels. With the depth theme, each casts to the bottom-right like buildings in Mini Motorways."
      >
        <ShadowScale />
      </Section>

      <Section
        id="demo-depth-scale"
        title="Depth Scale"
        description="3-layer building system: walls (::before) + ground shadow (::after). Three size levels: sm (4px), default (6px), lg (7px)."
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
        title="Pressable Cards"
        description="Hover to sink the building toward the ground. Walls shrink, shadow fades. Active pushes flush."
      >
        <PressableCards />
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
        id="demo-pressable"
        title="Pressable Buttons"
        description="Component-driven 3D press: depth + pressable + color variant + size. All via Button CVA props."
      >
        <PressableButtons />
      </Section>

      <Section id="demo-buttons" title="Buttons" description="All variants and sizes with built-in depth animation.">
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
        title="Animated Tool Grid"
        description="StaggerCascade + PopIn + pressable Buttons. The signature bnto interaction."
      >
        <AnimatedCardGrid />
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
