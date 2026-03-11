"use client";

import { useCallback, useState } from "react";
import {
  Button,
  Row,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Stack,
  Text,
} from "@bnto/ui";
import { RECIPES, getRecipeBySlug } from "@bnto/nodes";
import { useEditor } from "../../context";
import type { ExecutionPhase, FileProgress } from "../../store/types";
import { MOCK_RESULTS } from "./devMockData";
import { DevNodeControls } from "./DevNodeControls";

/** Build a FileProgress object for a given percentage. */
function buildFileProgress(percent: number): FileProgress {
  return {
    fileIndex: Math.floor((percent / 100) * 3),
    totalFiles: 3,
    overallPercent: percent,
    message: `Dev: ${percent}% progress`,
  };
}

/**
 * DevTab — dev-only controls for forcing execution states in the editor.
 *
 * Allows forcing:
 * - Execution phase (idle, running, completed, failed)
 * - Progress percentage (0-100%)
 * - Recipe loading (6 public recipes)
 *
 * Only rendered when NODE_ENV === "development".
 */
function DevTab() {
  const editor = useEditor();
  const [progress, setProgress] = useState(0);

  const forcePhase = useCallback(
    (phase: ExecutionPhase) => {
      if (phase === "idle") return editor.execution.resetRun();

      const stateByPhase = {
        running: {
          executionPhase: "running" as const,
          executionResults: [],
          executionErrors: [],
          executionFileProgress: buildFileProgress(progress),
        },
        completed: {
          executionPhase: "completed" as const,
          executionResults: MOCK_RESULTS,
          executionErrors: [],
          executionFileProgress: null,
        },
        failed: {
          executionPhase: "failed" as const,
          executionResults: [],
          executionErrors: ["Dev: forced error — something went wrong during processing."],
          executionFileProgress: null,
        },
      };
      editor.execution.forceExecutionState(stateByPhase[phase]);
    },
    [editor, progress],
  );

  const forceProgress = useCallback((percent: number) => {
    setProgress(percent);
    editor.execution.forceExecutionState({
      executionPhase: "running",
      executionFileProgress: buildFileProgress(percent),
    });
  }, [editor]);

  return (
    <div className="flex h-full flex-col overflow-y-auto p-3">
      <Stack className="gap-4">
        <PhaseControls onForce={forcePhase} />
        <ProgressControl progress={progress} onForce={forceProgress} />
        <DevNodeControls />
        <RecipeSelect />
      </Stack>
    </div>
  );
}

function PhaseControls({ onForce }: { onForce: (p: ExecutionPhase) => void }) {
  return (
    <Stack className="gap-1.5">
      <Text size="xs" color="muted" weight="medium">
        Force Phase
      </Text>
      <Row gap="xs" className="flex-wrap">
        <Button variant="outline" size="sm" onClick={() => onForce("idle")}>
          Idle
        </Button>
        <Button variant="outline" size="sm" onClick={() => onForce("running")}>
          Running
        </Button>
        <Button variant="outline" size="sm" onClick={() => onForce("completed")}>
          Completed
        </Button>
        <Button variant="destructive" size="sm" onClick={() => onForce("failed")}>
          Failed
        </Button>
      </Row>
    </Stack>
  );
}

function ProgressControl({
  progress,
  onForce,
}: {
  progress: number;
  onForce: (p: number) => void;
}) {
  return (
    <Stack className="gap-1.5">
      <Row className="justify-between">
        <Text size="xs" color="muted" weight="medium">
          Force Progress
        </Text>
        <Text size="xs" color="muted" className="font-mono">
          {progress}%
        </Text>
      </Row>
      <Slider value={[progress]} onValueChange={([v]) => onForce(v)} min={0} max={100} step={1} />
    </Stack>
  );
}

function RecipeSelect() {
  const editor = useEditor();
  const handleLoadRecipe = useCallback((slug: string) => {
    const recipe = getRecipeBySlug(slug);
    if (recipe) editor.definition.loadDefinition(recipe.definition);
  }, [editor]);

  return (
    <Stack className="gap-1.5">
      <Text size="xs" color="muted" weight="medium">
        Load Recipe
      </Text>
      <Select onValueChange={handleLoadRecipe}>
        <SelectTrigger className="w-full">
          <SelectValue placeholder="Select recipe..." />
        </SelectTrigger>
        <SelectContent>
          {RECIPES.map((recipe) => (
            <SelectItem key={recipe.slug} value={recipe.slug}>
              {recipe.name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </Stack>
  );
}

export { DevTab };
