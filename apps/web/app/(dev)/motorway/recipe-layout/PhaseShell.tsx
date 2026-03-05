"use client";

import type { ReactNode } from "react";
import { Card, Grid, GridItem, Heading, Text } from "@bnto/ui";
import { PhaseIndicator } from "@/app/(app)/[bnto]/_components/PhaseIndicator";

const MOCK_H1 = "Compress Images Online Free";
const MOCK_DESCRIPTION =
  "Compress PNG, JPEG, and WebP images in your browser. Files never leave your machine.";

/**
 * Shared 9x5 grid layout for all recipe phases.
 * Heading, phase indicator are always the same.
 * Slots let each phase inject its own controls, stats, and content.
 */
export function PhaseShell({
  phase,
  controlPanel,
  actionBar,
  statsCard,
  leftBottomCard,
  mainContent,
}: {
  phase: 1 | 2 | 3;
  controlPanel: ReactNode;
  actionBar: ReactNode;
  statsCard: ReactNode;
  leftBottomCard: ReactNode;
  mainContent: ReactNode;
}) {
  return (
    <Grid cols={9} rows={5} gap="md">
      {/* Heading/Description -- always cols 1-3, rows 1-2 */}
      <GridItem colSpan={3} rowSpan={2} colStart={1} rowStart={1}>
        <div className="flex h-full flex-col justify-start">
          <Heading level={2} size="lg">
            {MOCK_H1}
          </Heading>
          <Text color="muted" className="mt-3 leading-snug text-balance">
            {MOCK_DESCRIPTION}
          </Text>
        </div>
      </GridItem>

      {/* Control panels -- cols 4-7, rows 1-2 (phase-specific) */}
      {controlPanel}

      {/* Phase indicator -- cols 7-9, row 1 */}
      <GridItem colSpan={3} rowSpan={1} colStart={7} rowStart={1}>
        <Card className="flex h-full items-center justify-center p-4" elevation="sm">
          <PhaseIndicator activePhase={phase} />
        </Card>
      </GridItem>

      {/* Action bar -- cols 8-9, row 2 */}
      <GridItem colSpan={2} rowSpan={1} colStart={8} rowStart={2}>
        {actionBar}
      </GridItem>

      {/* Stats card -- cols 1-3, row 3 */}
      <GridItem colSpan={3} rowSpan={1} colStart={1} rowStart={3}>
        {statsCard}
      </GridItem>

      {/* Left bottom card -- cols 1-3, row 4 */}
      <GridItem colSpan={3} rowSpan={1} colStart={1} rowStart={4}>
        {leftBottomCard}
      </GridItem>

      {/* Main content -- cols 4-9, rows 3-5 */}
      <GridItem colSpan={6} rowSpan={3} colStart={4} rowStart={3}>
        {mainContent}
      </GridItem>
    </Grid>
  );
}
