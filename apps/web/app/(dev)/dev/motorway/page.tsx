"use client";

import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Tabs } from "@/components/ui/Tabs";
import { Text } from "@/components/ui/Text";

import { AnimationShowcase } from "./AnimationShowcase";
import { ButtonShowcase } from "./ButtonShowcase";
import { CardShowcase } from "./CardShowcase";
import { CastShadowShowcase } from "./CastShadowShowcase";
import { ColorSwatches } from "./ColorSwatches";
import { ConveyorShowcase } from "./ConveyorShowcase";
import { GridShowcase } from "./GridShowcase";
import { FileListShowcase } from "./FileListShowcase";
import { FormShowcase } from "./FormShowcase";
import { InputShowcase } from "./InputShowcase";
import { MenuShowcase } from "./MenuShowcase";
import { NotificationCards } from "./NotificationCards";
import { ShowcaseSection } from "./ShowcaseSection";
import { TypographyShowcase } from "./TypographyShowcase";

export default function MotorwayPage() {
  return (
    <div className="mx-auto flex w-full max-w-6xl flex-col gap-6 px-6 py-8">
      {/* Header */}
      <div>
        <Heading level={1} size="md">Motorway</Heading>
        <Text color="muted" className="mt-1 max-w-xl">
          The bnto design system. Warm, organized, satisfying, like a
          well-packed bento box on a Mini Motorways map. 3D surfaces, springy
          motion, three themed palettes.
        </Text>
      </div>

      <Tabs defaultValue="surfaces" className="[&>[role=tabpanel]]:mt-8">
        <Tabs.List>
          <Tabs.Trigger value="surfaces">Surfaces</Tabs.Trigger>
          <Tabs.Trigger value="typography">Typography</Tabs.Trigger>
          <Tabs.Trigger value="controls">Controls</Tabs.Trigger>
          <Tabs.Trigger value="layout">Layout</Tabs.Trigger>
          <Tabs.Trigger value="motion">Motion</Tabs.Trigger>
          <Tabs.Trigger value="forms">Forms</Tabs.Trigger>
          <Tabs.Trigger value="grids">Grids</Tabs.Trigger>
          <Tabs.Trigger value="lab">Lab</Tabs.Trigger>
        </Tabs.List>

        {/* ── Surfaces ────────────────────────────────────────── */}
        <Tabs.Content value="surfaces">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="colors"
              title="Colors"
              description="Semantic color tokens across all variants. Each swatch renders with its matching surface tint."
            >
              <ColorSwatches />
            </ShowcaseSection>

            <ShowcaseSection
              id="elevation"
              title="Surface & Elevation"
              description="Cards cast directional shadows that follow the light source. Four elevation tiers from flush to floating."
            >
              <CardShowcase />
            </ShowcaseSection>
          </Stack>
        </Tabs.Content>

        {/* ── Typography ──────────────────────────────────────── */}
        <Tabs.Content value="typography">
          <ShowcaseSection
            id="typography"
            title="Typography"
            description="Geist for display headings, Inter for body, Geist Mono for technical output."
          >
            <TypographyShowcase />
          </ShowcaseSection>
        </Tabs.Content>

        {/* ── Controls ──────────────────────────────────────── */}
        <Tabs.Content value="controls">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="buttons"
              title="Buttons"
              description="Every variant, size, icon layout, and press state. Buttons sit on the ground plane and sink when pressed."
            >
              <ButtonShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="inputs"
              title="Inputs"
              description="Text fields, toggles, checkboxes, linear and radial sliders."
            >
              <InputShowcase />
            </ShowcaseSection>
          </Stack>
        </Tabs.Content>

        {/* ── Layout ──────────────────────────────────────────── */}
        <Tabs.Content value="layout">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="menus"
              title="Menus"
              description="Popover-based dropdowns. The trigger sinks into its pressed state when open."
            >
              <MenuShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="notifications"
              title="Lists"
              description="List items with static card surface."
            >
              <NotificationCards />
            </ShowcaseSection>
          </Stack>
        </Tabs.Content>

        {/* ── Motion ────────────────────────────────────────── */}
        <Tabs.Content value="motion">
          <ShowcaseSection
            id="motion"
            title="Motion"
            description="Entrances are springy. Elements pop onto the page like buildings materializing on the map. Transitions are smooth and restrained."
          >
            <AnimationShowcase />
          </ShowcaseSection>
        </Tabs.Content>

        {/* ── Forms ───────────────────────────────────────────── */}
        <Tabs.Content value="forms">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="forms"
              title="Form Composition"
              description="Controls composed into a realistic form inside an elevated card."
            >
              <FormShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="file-upload"
              title="File Upload"
              description="Drop zone with surface elevation, replaced by a staggered file list on selection. Clear to return to the drop zone."
            >
              <FileListShowcase />
            </ShowcaseSection>
          </Stack>
        </Tabs.Content>

        {/* ── Grids ───────────────────────────────────────────── */}
        <Tabs.Content value="grids">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="grid"
              title="Grid"
              description="Explicit grid placement via composition. Items define their own position — the Grid provides the context."
            >
              <GridShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="conveyor"
              title="Conveyor Belts"
              description="Animated connections between stations. Data flows like sushi on a kaiten belt — items ride the surface between processing nodes."
            >
              <ConveyorShowcase />
            </ShowcaseSection>
          </Stack>
        </Tabs.Content>

        {/* ── Lab ─────────────────────────────────────────────── */}
        <Tabs.Content value="lab">
          <ShowcaseSection
            id="cast-shadow"
            title="Geometric Cast Shadow"
            description="clip-path polygon shadows with straight extrusion lines instead of rounded-rect copies. Same perf profile — polygon is GPU-composited, interactions are transform + opacity only."
          >
            <CastShadowShowcase />
          </ShowcaseSection>
        </Tabs.Content>
      </Tabs>
    </div>
  );
}
