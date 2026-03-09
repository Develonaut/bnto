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
import {
  RECIPES,
  batchCompress,
  batchConvert,
  batchRename,
  batchResize,
  columnRenamer,
  csvCleaner,
} from "@bnto/nodes";
import type { Definition } from "@bnto/nodes";
import { getEditorStore } from "../../store/instance";
import { loadDefinition } from "../../actions/loadDefinition";
import type { ExecutionPhase, FileProgress } from "../../store/types";
import { MOCK_RESULTS } from "./devMockData";

/** Sub-recipes (primitives) available for direct loading. */
const SUB_RECIPES: { label: string; definition: Definition }[] = [
  { label: "Batch Compress", definition: batchCompress },
  { label: "Batch Convert", definition: batchConvert },
  { label: "Batch Rename", definition: batchRename },
  { label: "Batch Resize", definition: batchResize },
  { label: "Column Renamer", definition: columnRenamer },
  { label: "CSV Cleaner", definition: csvCleaner },
];

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
 * - Progress percentage (0–100%)
 * - Recipe and sub-recipe loading
 *
 * Only rendered when NODE_ENV === "development".
 */
function DevTab() {
  const [progress, setProgress] = useState(0);

  const forcePhase = useCallback(
    (phase: ExecutionPhase) => {
      const store = getEditorStore();
      if (phase === "idle") return store.getState().resetRun();

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
      store.setState(stateByPhase[phase]);
    },
    [progress],
  );

  const forceProgress = useCallback((percent: number) => {
    setProgress(percent);
    getEditorStore().setState({
      executionPhase: "running",
      executionFileProgress: buildFileProgress(percent),
    });
  }, []);

  const handleLoadSubRecipe = useCallback((id: string) => {
    const sub = SUB_RECIPES.find((s) => s.definition.id === id);
    if (sub) getEditorStore().setState(loadDefinition(sub.definition));
  }, []);

  return (
    <div className="flex h-full flex-col overflow-y-auto p-3">
      <Stack className="gap-4">
        <PhaseControls onForce={forcePhase} />
        <ProgressControl progress={progress} onForce={forceProgress} />
        <RecipeSelect />
        <SubRecipeSelect onSelect={handleLoadSubRecipe} />
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
  return (
    <Stack className="gap-1.5">
      <Text size="xs" color="muted" weight="medium">
        Load Recipe
      </Text>
      <Select onValueChange={(slug) => getEditorStore().getState().loadRecipe(slug)}>
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

function SubRecipeSelect({ onSelect }: { onSelect: (id: string) => void }) {
  return (
    <Stack className="gap-1.5">
      <Text size="xs" color="muted" weight="medium">
        Load Sub-Recipe
      </Text>
      <Select onValueChange={onSelect}>
        <SelectTrigger className="w-full">
          <SelectValue placeholder="Select sub-recipe..." />
        </SelectTrigger>
        <SelectContent>
          {SUB_RECIPES.map((sub) => (
            <SelectItem key={sub.definition.id} value={sub.definition.id}>
              {sub.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </Stack>
  );
}

export { DevTab };
