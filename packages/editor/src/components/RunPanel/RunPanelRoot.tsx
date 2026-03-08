"use client";

import {
  cn,
  Panel,
  PanelHeader,
  PanelDivider,
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
  Text,
} from "@bnto/ui";
import { usePanel } from "../../hooks/useEditorPanels";
import { useEditorExecutionContext } from "../../hooks/EditorExecutionContext";
import { ResultsTab } from "./ResultsTab";
import { LogsTab } from "./LogsTab";

/**
 * RunPanel — bottom-right panel with Results and Logs tabs.
 *
 * Opens automatically when execution starts. Shows output files
 * in the Results tab and streaming engine events in the Logs tab.
 * Positioned to the right side, below the config panel area.
 */
function RunPanelRoot() {
  const { isOpen: runPanelOpen } = usePanel("run");
  const { phase, results, errors, logs, downloadFile, downloadAll } = useEditorExecutionContext();

  const resultCount = results.length;
  const hasErrors = errors.length > 0;

  return (
    <div
      onPointerDownCapture={(e) => e.stopPropagation()}
      className={cn(
        "pointer-events-auto absolute bottom-0 right-0 w-80",
        "motion-safe:transition-[translate,opacity]",
        runPanelOpen
          ? "translate-y-0 opacity-100 motion-safe:duration-slow motion-safe:ease-spring-bouncy"
          : "pointer-events-none translate-y-[110%] opacity-0 motion-safe:duration-fast motion-safe:ease-out",
      )}
      style={{ height: "min(50%, 20rem)" }}
    >
      <Panel className="flex h-full w-full flex-col">
        <Tabs defaultValue="results">
          <PanelHeader className="gap-2 px-3 pt-2 pb-0">
            <TabsList>
              <TabsTrigger value="results">
                <Text size="xs">Results{resultCount > 0 ? ` (${resultCount})` : ""}</Text>
              </TabsTrigger>
              <TabsTrigger value="logs">
                <Text size="xs">Logs{hasErrors ? " (!)" : ""}</Text>
              </TabsTrigger>
            </TabsList>
          </PanelHeader>
          <PanelDivider />
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
      </Panel>
    </div>
  );
}

export { RunPanelRoot };
