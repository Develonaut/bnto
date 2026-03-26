"use client";

import { useCallback, useRef } from "react";
import { useTheme } from "next-themes";

import { Button, type SpringMode } from "../interaction/Button";
import { animateThemeTransition } from "./viewTransition";
import { ThemeIcons } from "./ThemeIcons";
import { useMounted } from "../hooks/useMounted";

type ElevationOverride = boolean | "none" | "sm" | "md" | "lg";

type ThemeName = "light" | "sunset" | "dark";

const THEME_CYCLE: ThemeName[] = ["light", "sunset", "dark"];

function nextTheme(current: ThemeName): ThemeName {
  const idx = THEME_CYCLE.indexOf(current);
  return THEME_CYCLE[(idx + 1) % THEME_CYCLE.length];
}

export function AnimatedThemeToggle({
  elevation,
  spring,
}: { elevation?: ElevationOverride; spring?: SpringMode } = {}) {
  const { resolvedTheme, setTheme } = useTheme();
  const buttonRef = useRef<HTMLButtonElement>(null);
  const mounted = useMounted();
  const current = mounted ? ((resolvedTheme ?? "light") as ThemeName) : undefined;

  const toggleTheme = useCallback(() => {
    if (!current || !buttonRef.current) {
      if (current) setTheme(nextTheme(current));
      return;
    }
    animateThemeTransition(buttonRef.current, () => setTheme(nextTheme(current)));
  }, [current, setTheme]);

  return (
    <Button
      ref={buttonRef}
      variant="outline"
      size="icon"
      elevation={elevation}
      spring={spring}
      className="group"
      data-theme={current}
      onClick={toggleTheme}
    >
      <ThemeIcons />
    </Button>
  );
}
