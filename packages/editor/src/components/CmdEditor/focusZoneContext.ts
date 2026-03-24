/**
 * focusZoneContext — internal context for CmdEditor focus zones.
 *
 * CmdEditorShell provides the refs and handler; CmdEditorTree and
 * CmdEditorCommand consume them to attach zone refs automatically.
 */

"use client";

import { createContext, useContext } from "react";
import type { RefObject } from "react";

interface FocusZoneContextValue {
  treeRef: RefObject<HTMLDivElement | null>;
  commandRef: RefObject<HTMLDivElement | null>;
}

const FocusZoneContext = createContext<FocusZoneContextValue | null>(null);

function useFocusZoneContext(): FocusZoneContextValue | null {
  return useContext(FocusZoneContext);
}

export { FocusZoneContext, useFocusZoneContext };
export type { FocusZoneContextValue };
