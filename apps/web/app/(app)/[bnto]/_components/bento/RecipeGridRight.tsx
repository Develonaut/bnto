"use client";

import { GridItem } from "@bnto/ui";
import type { BentoRecipeFlow } from "../../_hooks/useBentoRecipeFlow";
import { NodeConfigCard } from "./NodeConfigCard";
import { PipelineCard } from "./PipelineCard";
import { OutputCard } from "./OutputCard";

/** Right column grid items — config, pipeline, output. */
export function RecipeGridRight({ flow }: { flow: BentoRecipeFlow }) {
  return (
    <>
      <GridItem colSpan={2} rowSpan={6} colStart={5}>
        <NodeConfigCard
          nodes={flow.processingNodes}
          parameters={flow.parameters}
          onParameterChange={flow.setParameter}
        />
      </GridItem>
      <GridItem colSpan={2} rowSpan={3} colStart={3} rowStart={3}>
        <PipelineCard nodes={flow.processingNodes} nodeProgress={{}} />
      </GridItem>
      <GridItem colSpan={2} colStart={3} rowStart={6}>
        <OutputCard results={flow.exec.results} onDownload={flow.downloadResult} />
      </GridItem>
    </>
  );
}
