"use client";

import { Stack } from "@bnto/ui";

import { Phase1 } from "./recipe-layout/Phase1";
import { Phase2 } from "./recipe-layout/Phase2";
import { Phase3 } from "./recipe-layout/Phase3";
import { ShowcaseSection } from "./ShowcaseSection";

export function RecipeLayoutShowcase() {
  return (
    <Stack gap="xl" className="gap-16">
      <ShowcaseSection
        id="phase-1"
        title="Phase 1: Files"
        description="Upload state. Dropzone dominates the right side. Left column shows title, description, and accepted formats."
      >
        <Phase1 />
      </ShowcaseSection>

      <ShowcaseSection
        id="phase-2"
        title="Phase 2: Configure"
        description="Files selected. Controls appear in the mosaic — quality dial, phase stepper, action bar. Stats and file list fill the bottom."
      >
        <Phase2 />
      </ShowcaseSection>

      <ShowcaseSection
        id="phase-3"
        title="Phase 3: Results"
        description="Compression complete. Stats update with actual output sizes and savings. Each result card shows before/after with a download button."
      >
        <Phase3 />
      </ShowcaseSection>
    </Stack>
  );
}
