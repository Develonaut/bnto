"use client";

import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";

import { AnimationShowcase } from "./AnimationShowcase";
import { BentoGridShowcase } from "./BentoGridShowcase";
import { ButtonShowcase } from "./ButtonShowcase";
import { CardShowcase } from "./CardShowcase";
import { ColorSwatches } from "./ColorSwatches";
import { DropzoneShowcase } from "./DropzoneShowcase";
import { FileListShowcase } from "./FileListShowcase";
import { FormShowcase } from "./FormShowcase";
import { InputShowcase } from "./InputShowcase";
import { MenuShowcase } from "./MenuShowcase";
import { NotificationCards } from "./NotificationCards";
import { ShowcaseSection } from "./ShowcaseSection";
import { TypographyShowcase } from "./TypographyShowcase";

export default function MotorwayPage() {
  return (
    <div>
      <Container size="lg" className="pb-8 pt-24 lg:pt-32">
        <Stack className="gap-16">
          {/* Header */}
          <div>
            <Heading level={1} size="md">Motorway</Heading>
            <Text color="muted" className="mt-2 max-w-xl">
              The bnto design system. Warm, organized, satisfying &mdash; like a
              well-packed bento box on a Mini Motorways map. 3D depth, springy
              motion, three themed palettes.
            </Text>
          </div>

          {/* ── Foundations ────────────────────────────────────── */}

          <ShowcaseSection
            id="colors"
            title="Colors"
            description="Semantic color tokens across all variants. Each swatch renders with its matching depth tint."
          >
            <ColorSwatches />
          </ShowcaseSection>

          <ShowcaseSection
            id="typography"
            title="Typography"
            description="Geist for display headings, Inter for body, Geist Mono for technical output."
          >
            <TypographyShowcase />
          </ShowcaseSection>

          {/* ── Depth ───────────────────────────────────────────── */}

          <ShowcaseSection
            id="depth"
            title="Depth"
            description="Cards cast directional shadows that follow the light source. Four elevation tiers from flush to floating."
          >
            <CardShowcase />
          </ShowcaseSection>

          {/* ── Controls ──────────────────────────────────────── */}

          <ShowcaseSection
            id="buttons"
            title="Buttons"
            description="Every variant, size, icon layout, and press state. Buttons sit on the ground plane and sink when pressed."
          >
            <ButtonShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="menus"
            title="Menus"
            description="Popover-based dropdowns. The trigger sinks into its pressed state when open."
          >
            <MenuShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="inputs"
            title="Inputs"
            description="Text fields, toggles, checkboxes, linear and radial sliders."
          >
            <InputShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="forms"
            title="Form Composition"
            description="Controls composed into a realistic form inside an elevated card."
          >
            <FormShowcase />
          </ShowcaseSection>

          {/* ── Motion ────────────────────────────────────────── */}

          <ShowcaseSection
            id="motion"
            title="Motion"
            description="Entrances are springy — elements pop onto the page like buildings materializing on the map. Transitions are smooth and restrained."
          >
            <AnimationShowcase />
          </ShowcaseSection>

          {/* ── Patterns ──────────────────────────────────────── */}

          <ShowcaseSection
            id="dropzone"
            title="Dropzone"
            description="File upload area with depth integration."
          >
            <DropzoneShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="file-list"
            title="File List"
            description="Dropzone to file list transition with staggered entrance animation."
          >
            <FileListShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="notifications"
            title="Notifications"
            description="List items with static card depth."
          >
            <NotificationCards />
          </ShowcaseSection>

          <ShowcaseSection
            id="bento-grid"
            title="Bento Grid"
            description="Dynamic layout driven by item count. Featured cells span multiple rows and columns, adapting as the grid grows."
          >
            <BentoGridShowcase />
          </ShowcaseSection>
        </Stack>
      </Container>
    </div>
  );
}
