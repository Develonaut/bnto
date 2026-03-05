"use client";

import { useContext } from "react";

import { BentoItemContext } from "./bentoGridContext";
import type { CellLayout } from "./bentoGridContext";

/** Access the current item's bento layout inside a `<BentoGrid>` child. */
export function useBentoItem(): CellLayout {
  const ctx = useContext(BentoItemContext);
  if (!ctx) throw new Error("useBentoItem must be used inside <BentoGrid>");
  return ctx;
}

export type { CellLayout };
