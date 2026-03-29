"use client";

import { Card, CardContent, Text } from "@bnto/ui";
import type { BentoFlowPhase } from "../../_hooks/useBentoRecipeFlow";
import { ToolbarActions } from "./ToolbarActions";
import { ToolbarStatus } from "./ToolbarStatus";

export interface ToolbarCardProps {
  phase: BentoFlowPhase;
  hasFiles: boolean;
  resultCount: number;
  onRun: () => void;
  onReset: () => void;
  onDownloadAll: () => void;
}

/** Run button, progress, and download controls. */
export function ToolbarCard(props: ToolbarCardProps) {
  return (
    <Card elevation="sm" className="p-5">
      <CardContent className="flex flex-col gap-3 p-0">
        <Text size="xs" color="muted" className="font-medium uppercase tracking-wider">
          Actions
        </Text>
        <ToolbarActions {...props} />
        <ToolbarStatus phase={props.phase} />
      </CardContent>
    </Card>
  );
}
