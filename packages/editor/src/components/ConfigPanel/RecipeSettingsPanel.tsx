"use client";

import { useCallback } from "react";
import { Divider, Heading, Text } from "@bnto/ui";
import type { IterationMode } from "@bnto/core";
import { useEditor } from "../../context";
import { IterationModeSelect } from "./IterationModeSelect";

/**
 * RecipeSettingsPanel — shown in ConfigPanel when no node is selected.
 *
 * Surfaces recipe-level settings: name and iteration mode.
 */
function RecipeSettingsPanel() {
  const editor = useEditor();
  const { definition, recipeMetadata } = editor.definition.useDefinition();

  const iterationMode: IterationMode = definition?.settings?.iteration ?? "explicit";

  const handleIterationChange = useCallback(
    (value: string) => {
      editor.definition.setSettings({ ...definition?.settings, iteration: value as IterationMode });
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
          <IterationModeSelect value={iterationMode} onChange={handleIterationChange} />
        </div>
      </div>
    </>
  );
}

export { RecipeSettingsPanel };
