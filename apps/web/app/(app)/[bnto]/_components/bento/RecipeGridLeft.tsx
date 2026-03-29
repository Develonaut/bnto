"use client";

import { GridItem } from "@bnto/ui";
import type { BntoEntry } from "@/lib/bntoRegistry";
import type { BentoRecipeFlow } from "../../_hooks/useBentoRecipeFlow";
import { RecipeInfoCard } from "./RecipeInfoCard";
import { InputCard } from "./InputCard";
import { ToolbarCard } from "./ToolbarCard";

interface RecipeGridLeftProps {
  entry: BntoEntry;
  flow: BentoRecipeFlow;
}

/** Left column grid items — info, input, toolbar. */
export function RecipeGridLeft({ entry, flow }: RecipeGridLeftProps) {
  return (
    <>
      <GridItem colSpan={2} rowSpan={3}>
        <RecipeInfoCard entry={entry} />
      </GridItem>
      <GridItem colSpan={2} rowSpan={2} colStart={3}>
        <InputCard
          files={flow.files}
          onFilesChange={flow.setFiles}
          acceptLabel={flow.acceptLabel}
          dropzoneAccept={flow.dropzoneAccept}
          disabled={flow.phase === "running"}
        />
      </GridItem>
      <GridItem colSpan={2} rowSpan={3} rowStart={4}>
        <ToolbarCard
          phase={flow.phase}
          hasFiles={flow.files.length > 0}
          resultCount={flow.exec.results.length}
          onRun={flow.handleRun}
          onReset={flow.handleReset}
          onDownloadAll={flow.downloadAll}
        />
      </GridItem>
    </>
  );
}
