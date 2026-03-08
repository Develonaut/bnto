"use client";

import { Divider, Tabs, TabsList, TabsTrigger, TabsContent, TerminalIcon, Text } from "@bnto/ui";
import { useEditorExecutionContext } from "../../hooks/EditorExecutionContext";
import { ResultsTab } from "./ResultsTab";
import { LogsTab } from "./LogsTab";
import { EditorMenuPanel } from "../EditorMenuPanel";

/**
 * RunPanel — Menu-based results/logs panel.
 *
 * Opens to the left from the right toolbar trigger. Shows output files
 * in the Results tab and streaming engine events in the Logs tab.
 */
function RunPanelRoot() {
  const { phase, results, errors, logs, downloadFile, downloadAll } = useEditorExecutionContext();

  const resultCount = results.length;
  const hasErrors = errors.length > 0;

  return (
    <EditorMenuPanel
      panelId="run"
      side="left"
      width="w-80"
      label="Run panel"
      icon={<TerminalIcon className="size-4" />}
    >
      <Tabs defaultValue="results">
        <div className="flex shrink-0 items-center gap-2 px-3 pt-3 pb-2">
          <TabsList>
            <TabsTrigger value="results">
              <Text size="xs">Results{resultCount > 0 ? ` (${resultCount})` : ""}</Text>
            </TabsTrigger>
            <TabsTrigger value="logs">
              <Text size="xs">Logs{hasErrors ? " (!)" : ""}</Text>
            </TabsTrigger>
          </TabsList>
        </div>
        <Divider />
        <div className="min-h-0 flex-1 overflow-hidden">
          <TabsContent value="results" className="mt-0 h-full">
            <ResultsTab
              phase={phase}
              results={results}
              errors={errors}
              onDownloadFile={downloadFile}
              onDownloadAll={downloadAll}
            />
          </TabsContent>
          <TabsContent value="logs" className="mt-0 h-full">
            <LogsTab logs={logs} />
          </TabsContent>
        </div>
      </Tabs>
    </EditorMenuPanel>
  );
}

export { RunPanelRoot };
