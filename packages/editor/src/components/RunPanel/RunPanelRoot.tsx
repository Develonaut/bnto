"use client";

import { useMemo, useState } from "react";
import {
  CopyButton,
  Divider,
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
  TerminalIcon,
  Text,
} from "@bnto/ui";
import { useEditor } from "../../context";
import { ResultsTab } from "./ResultsTab";
import { LogsTab } from "./LogsTab";
import { EditorMenuPanel } from "../EditorMenuPanel";
import { formatLogEntry } from "./formatLogEntry";

/**
 * RunPanel — Menu-based results/logs panel.
 *
 * Opens upward from the toolbar trigger. Dismisses on outside click.
 * Children consume execution state directly from the editor store.
 */
function RunPanelRoot() {
  const editor = useEditor();
  const { results, logs } = editor.execution.useExecution();
  const [activeTab, setActiveTab] = useState("results");

  const logsText = useMemo(
    () =>
      logs
        .map((entry) => {
          const { time, icon, message } = formatLogEntry(entry);
          return `${time} ${icon} ${message}`;
        })
        .join("\n"),
    [logs],
  );

  return (
    <EditorMenuPanel
      panelId="run"
      side="top"
      width="w-full"
      boundaryPadding={16}
      label="Run panel"
      icon={<TerminalIcon className="size-4" />}
      dismissOnOutsideClick
      className="!h-[300px] !w-[40vw] !min-w-0"
    >
      <Tabs value={activeTab} onValueChange={setActiveTab} className="flex h-full flex-col">
        <div className="flex shrink-0 items-center justify-between px-3 pt-3 pb-2">
          <TabsList>
            <TabsTrigger value="results" data-testid="tab-run">
              <Text size="xs">Run{results.length > 0 ? ` (${results.length})` : ""}</Text>
            </TabsTrigger>
            <TabsTrigger value="logs" data-testid="tab-logs">
              <Text size="xs">Logs{logs.length > 0 ? ` (${logs.length})` : ""}</Text>
            </TabsTrigger>
          </TabsList>
          {activeTab === "logs" && logs.length > 0 && (
            <CopyButton value={logsText} label="Copy logs" />
          )}
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
