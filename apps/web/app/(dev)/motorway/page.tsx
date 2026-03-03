"use client";

import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Tabs } from "@/components/ui/Tabs";
import { Text } from "@/components/ui/Text";

import { AnimationShowcase } from "./AnimationShowcase";
import { BentoBoxShowcase } from "./BentoBoxShowcase";
import { EmptyStateShowcase } from "./EmptyStateShowcase";
import { ButtonShowcase } from "./ButtonShowcase";
import { CardShowcase } from "./CardShowcase";
import { ColorSwatches } from "./ColorSwatches";
import { ConveyorShowcase } from "./ConveyorShowcase";
import { DialogShowcase } from "./DialogShowcase";
import { GridShowcase } from "./GridShowcase";
import { FileListShowcase } from "./FileListShowcase";
import { FormShowcase } from "./FormShowcase";
import { InputShowcase } from "./InputShowcase";
import { MenuShowcase } from "./MenuShowcase";
import { NotificationCards } from "./NotificationCards";
import { PhaseFlowShowcase } from "./PhaseFlowShowcase";
import { ProgressShowcase } from "./ProgressShowcase";
import { RecipeCardShowcase } from "./RecipeCardShowcase";
import { ShowcaseSection } from "./ShowcaseSection";
import { LoadingCardShowcase, SpringableShowcase } from "./SpringableShowcase";
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
          <Tabs.Trigger value="overlays">Overlays</Tabs.Trigger>
          <Tabs.Trigger value="forms">Forms</Tabs.Trigger>
          <Tabs.Trigger value="grids">Grids</Tabs.Trigger>
          <Tabs.Trigger value="progress">Progress</Tabs.Trigger>
          <Tabs.Trigger value="features">Features</Tabs.Trigger>
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

            <ShowcaseSection
              id="springable"
              title="Springable Surfaces"
              description="Configure spring mode and elevation independently. The spring foundation shared by all animated surfaces — pressable buttons and loading cards alike."
            >
              <SpringableShowcase />
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

            <ShowcaseSection
              id="loading-card"
              title="Loading Cards"
              description="Pass loading to Card for the common case. Springs up with bounciest when content arrives. Skeleton placeholders swap to real content in place."
            >
              <LoadingCardShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="recipe-card"
              title="Recipe Card"
              description="Saved recipe cards for the dashboard grid. Shows name, node count, last run status, and relative timestamp. Includes skeleton loading state."
            >
              <RecipeCardShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="empty-state"
              title="Empty State"
              description="Composable empty state for lists, dashboards, and search results. Bare icon, title, description, and optional action. Three size variants."
            >
              <EmptyStateShowcase />
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

        {/* ── Overlays ──────────────────────────────────────────── */}
        <Tabs.Content value="overlays">
          <ShowcaseSection
            id="overlays"
            title="Dialogs & Overlays"
            description="Modal dialogs, confirmation prompts, form dialogs, and the AuthGate conversion patterns (Action + Section). Focus-trapped, keyboard accessible, backdrop blur."
          >
            <DialogShowcase />
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
          <ShowcaseSection
            id="grid"
            title="Grid"
            description="Explicit grid placement via composition. Items define their own position — the Grid provides the context."
          >
            <GridShowcase />
          </ShowcaseSection>
        </Tabs.Content>

        {/* ── Progress ──────────────────────────────────────────── */}
        <Tabs.Content value="progress">
          <ShowcaseSection
            id="progress-indicators"
            title="Progress Indicators"
            description="LinearProgress primitive at different values, with icons, labels, and helper text."
          >
            <ProgressShowcase />
          </ShowcaseSection>
        </Tabs.Content>

        {/* ── Features ─────────────────────────────────────────── */}
        <Tabs.Content value="features">
          <Stack gap="xl" className="gap-16">
            <ShowcaseSection
              id="phase-flow"
              title="Recipe Phase Flow"
              description="Step through the recipe page phases: Dropzone → Configure → Processing → Completed. The file grid persists across phases — no DOM jumping."
            >
              <PhaseFlowShowcase />
            </ShowcaseSection>

            <ShowcaseSection
              id="bento-box"
              title="Bento Box"
              description="Add compartments to the grid and watch the box grow. Surface cards of different sizes tile like a real bento box. The viewport smoothly zooms out as the recipe expands — like Mini Motorways pulling back as the city grows."
            >
              <BentoBoxShowcase />
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
      </Tabs>
    </div>
  );
}
