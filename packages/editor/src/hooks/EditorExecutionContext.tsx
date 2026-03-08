"use client";

/**
 * EditorExecutionContext — shares a single useEditorExecution() instance
 * across multiple editor components.
 *
 * Without this, each component calling useEditorExecution() independently
 * would create its own phase/results/errors state — losing coordination.
 * The provider calls the hook once and distributes via React context.
 */

import { createContext, useContext, type ReactNode } from "react";
import { useEditorExecution } from "./useEditorExecution";
import type { EditorExecutionResult } from "./useEditorExecution";

const ExecutionContext = createContext<EditorExecutionResult | null>(null);

interface EditorExecutionProviderProps {
  children: ReactNode;
}

function EditorExecutionProvider({ children }: EditorExecutionProviderProps) {
  const execution = useEditorExecution();
  return <ExecutionContext.Provider value={execution}>{children}</ExecutionContext.Provider>;
}

/**
 * Access the shared execution instance. Must be inside EditorExecutionProvider.
 */
function useEditorExecutionContext(): EditorExecutionResult {
  const ctx = useContext(ExecutionContext);
  if (!ctx) {
    throw new Error("useEditorExecutionContext must be inside <EditorExecutionProvider>");
  }
  return ctx;
}

export { EditorExecutionProvider, useEditorExecutionContext };
