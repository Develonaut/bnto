"use client";

import { useCallback } from "react";
import {
  Divider,
  Heading,
  Label,
  Text,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@bnto/ui";
import type { IterationMode } from "@bnto/core";
import { useEditor } from "../../context";

/**
 * RecipeSettingsPanel — shown in ConfigPanel when no node is selected.
 *
 * Surfaces recipe-level settings: name and iteration mode.
 * Replaces the old "Select a node to configure" placeholder.
 */
function RecipeSettingsPanel() {
  const editor = useEditor();
  const { definition, recipeMetadata } = editor.definition.useDefinition();

  const iterationMode: IterationMode = definition?.settings?.iteration ?? "explicit";

  const handleIterationChange = useCallback(
    (value: string) => {
      editor.definition.setSettings({
        ...definition?.settings,
        iteration: value as IterationMode,
      });
    },
    [editor, definition?.settings],
  );

  return (
    <>
      <div className="shrink-0 px-3 pt-3 pb-2">
        <Heading level={3} size="xs">
          Recipe Settings
        </Heading>
        <Text size="xs" color="muted" className="mt-1">
          {recipeMetadata.name}
        </Text>
      </div>
      <Divider />
      <div className="flex-1 overflow-y-auto">
        <div className="flex flex-col gap-3 p-3">
          <div className="flex flex-col gap-1.5">
            <Label>File Iteration</Label>
            <Select value={iterationMode} onValueChange={handleIterationChange}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="auto">Auto</SelectItem>
                <SelectItem value="explicit">Explicit</SelectItem>
              </SelectContent>
            </Select>
            <Text size="xs" color="muted">
              {iterationMode === "auto"
                ? "Automatically processes each file through the pipeline."
                : "Requires explicit loop nodes for per-file iteration."}
            </Text>
          </div>
        </div>
      </div>
    </>
  );
}

export { RecipeSettingsPanel };
