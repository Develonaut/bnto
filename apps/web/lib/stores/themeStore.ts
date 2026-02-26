"use client";

import { create } from "zustand";
import { persist } from "zustand/middleware";

/** localStorage key used by Zustand persist. Shared with ThemeProvider's FOUC script. */
export const THEME_STORE_KEY = "bnto-theme";

/** Default light angle (degrees). NW direction. */
export const THEME_STORE_DEFAULT_ANGLE = 135;

interface ThemeStore {
  lightAngle: number;
  setLightAngle: (angle: number) => void;
}

export const useThemeStore = create<ThemeStore>()(
  persist(
    (set) => ({
      lightAngle: THEME_STORE_DEFAULT_ANGLE,
      setLightAngle: (angle) => set({ lightAngle: angle }),
    }),
    {
      name: THEME_STORE_KEY,
    },
  ),
);
