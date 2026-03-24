"use client";

import { Divider, Tabs, TabsList, TabsTrigger, TabsContent, TerminalIcon, Text } from "@bnto/ui";
import { useEditor } from "../../context";
import { ResultsTab } from "./ResultsTab";
import { LogsTab } from "./LogsTab";
import { EditorMenuPanel } from "../EditorMenuPanel";

/**
 * RunPanel — Menu-based results/logs panel.
 *
 * Opens to the left from the right toolbar trigger. Children
 * consume execution state directly from the editor store.
 */
function RunPanelRoot() {
  const editor = useEditor();
  const { results, logs } = editor.execution.useExecution();

  return (
    <EditorMenuPanel
      panelId="run"
      side="left"
      width="w-80"
      boundaryPadding={16}
      label="Run panel"
      icon={<TerminalIcon className="size-4" />}
    >
      <Tabs defaultValue="results" className="flex h-full flex-col">
        <div className="flex shrink-0 items-center gap-2 px-3 pt-3 pb-2">
          <TabsList>
            <TabsTrigger value="results" data-testid="tab-run">
              <Text size="xs">Run{results.length > 0 ? ` (${results.length})` : ""}</Text>
            </TabsTrigger>
            <TabsTrigger value="logs" data-testid="tab-logs">
              <Text size="xs">Logs{logs.length > 0 ? ` (${logs.length})` : ""}</Text>
            </TabsTrigger>
          </TabsList>
        </div>
        <Divider />
        <TabsContent value="results" className="mt-0 min-h-0 flex-1">
          <ResultsTab />
        </TabsContent>
        <TabsContent value="logs" className="mt-0 min-h-0 flex-1">
          <LogsTab />
        </TabsContent>
      </Tabs>
    </EditorMenuPanel>
  );
}

export { RunPanelRoot };
