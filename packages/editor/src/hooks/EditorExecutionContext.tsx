"use client";

import { createContext, useContext, type ReactNode } from "react";
import { useEditorExecution } from "./useEditorExecution";
import type { EditorExecutionResult } from "./useEditorExecution";

const EditorExecutionContext = createContext<EditorExecutionResult | null>(null);

function EditorExecutionProvider({ children }: { children: ReactNode }) {
  const execution = useEditorExecution();
  return (
    <EditorExecutionContext.Provider value={execution}>{children}</EditorExecutionContext.Provider>
  );
}

function useEditorExecutionContext(): EditorExecutionResult {
  const ctx = useContext(EditorExecutionContext);
  if (!ctx) {
    throw new Error("useEditorExecutionContext must be used inside EditorExecutionProvider");
  }
  return ctx;
}

export { EditorExecutionProvider, useEditorExecutionContext };
