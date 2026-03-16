"use client";

/**
 * useDraftHydration — loads a saved draft from localStorage on mount.
 *
 * Returns the draft's Definition if one exists, undefined otherwise.
 * Skipped when URL params indicate a specific recipe source.
 */

import { useMemo } from "react";
import type { Definition } from "@bnto/nodes";
import { loadDraft } from "../draft/draftStorage";

interface UseDraftHydrationOptions {
  /** When true, skip hydration (e.g. URL params specify a recipe source). */
  skip?: boolean;
}

function useDraftHydration({ skip = false }: UseDraftHydrationOptions = {}):
  | Definition
  | undefined {
  return useMemo(() => {
    if (skip) return undefined;
    if (typeof window === "undefined") return undefined;
    const draft = loadDraft(localStorage);
    return draft?.definition;
  }, [skip]);
}

export { useDraftHydration };
