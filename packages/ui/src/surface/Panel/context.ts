"use client";

import { createContext, useContext } from "react";

export interface PanelContextValue {
  collapsed: boolean;
  onToggle?: () => void;
}

export const PanelContext = createContext<PanelContextValue | null>(null);

export function usePanelContext() {
  const ctx = useContext(PanelContext);
  if (!ctx) throw new Error("Panel sub-components must be used within <Panel>");
  return ctx;
}
