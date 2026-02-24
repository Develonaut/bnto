"use client";

import { useState } from "react";

import { Container } from "@/components/ui/Container";
import { Heading } from "@/components/ui/Heading";
import { Text } from "@/components/ui/Text";
import { Stack } from "@/components/ui/Stack";

import { AnimationShowcase } from "./AnimationShowcase";
import { ButtonShowcase } from "./ButtonShowcase";
import { CardShowcase } from "./CardShowcase";
import { ColorSwatches } from "./ColorSwatches";
import { DropzoneShowcase } from "./DropzoneShowcase";
import { FormShowcase } from "./FormShowcase";
import { LightSourceSlider } from "./LightSourceSlider";
import { NotificationCards } from "./NotificationCards";
import { ShowcaseSection } from "./ShowcaseSection";
import { ToolGrid } from "./ToolGrid";
import { TypographyShowcase } from "./TypographyShowcase";

export default function ThemeDemoPage() {
  const [lightAngle, setLightAngle] = useState(135);

  return (
    <div style={{ "--light-angle": `${lightAngle}deg` } as React.CSSProperties}>
      <Container size="lg" className="pb-8 pt-24 lg:pt-32">
        <Stack className="gap-16">
          <div>
            <Heading level={1} size="md">Theme Demo</Heading>
            <Text color="muted" className="mt-2">
              Visual showcase for the Mini Motorways depth theme. Every shadow on
              this page is driven by CSS tokens &mdash; drag the slider to rotate
              the light source.
            </Text>
            <div className="mt-4">
              <LightSourceSlider value={lightAngle} onChange={setLightAngle} />
            </div>
          </div>

          <ShowcaseSection
            id="demo-cards"
            title="Cards"
            description="All tiers share the same light direction. Only magnitude differs — taller buildings cast longer shadows. Add .pressable for hover/active interaction."
          >
            <CardShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="demo-animations"
            title="Animations"
            description="Mini Motorways motion language. Entrances are springy (spring easing via CSS linear()). Transitions are smooth (ease-out cubic-bezier). Hit Replay to re-trigger."
          >
            <AnimationShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="demo-colors"
            title="Colors"
            description="Terracotta, teal, golden, cream — the warm Mini Motorways palette."
          >
            <ColorSwatches />
          </ShowcaseSection>

          <ShowcaseSection id="demo-typography" title="Typography" description="DM Sans display + Inter body + Geist Mono code.">
            <TypographyShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="demo-buttons"
            title="Buttons"
            description="All variants × sizes with depth + pressable. Icon buttons included."
          >
            <ButtonShowcase />
          </ShowcaseSection>

          <ShowcaseSection id="demo-form" title="Form Elements" description="Inputs, switches, checkboxes, slider inside an elevated card.">
            <FormShowcase />
          </ShowcaseSection>

          <ShowcaseSection id="demo-dropzone" title="Dropzone" description="File upload area with card depth.">
            <DropzoneShowcase />
          </ShowcaseSection>

          <ShowcaseSection
            id="demo-tool-grid"
            title="Tool Grid"
            description="Pressable Buttons in a grid layout."
          >
            <ToolGrid />
          </ShowcaseSection>

          <ShowcaseSection
            id="demo-notifications"
            title="Notification Cards"
            description="List items with static card depth — walls + ground shadow."
          >
            <NotificationCards />
          </ShowcaseSection>
        </Stack>
      </Container>
    </div>
  );
}
