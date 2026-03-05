"use client";

import { createContext } from "react";

export interface CellLayout {
  colSpan: 1 | 2 | 3;
  rowSpan: 1 | 2;
  featured: boolean;
}

export const BentoItemContext = createContext<CellLayout | null>(null);
