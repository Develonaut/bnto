"use client";

import { useCallback, useRef, useSyncExternalStore } from "react";
import { MoonIcon, SunIcon, SunsetIcon } from "../icons";
import { useTheme } from "next-themes";

import { Button, type SpringMode } from "../interaction/Button";
import { animateThemeTransition } from "./viewTransition";

type ElevationOverride = boolean | "none" | "sm" | "md" | "lg";

type ThemeName = "light" | "sunset" | "dark";

const THEME_CYCLE: ThemeName[] = ["light", "sunset", "dark"];

function nextTheme(current: ThemeName): ThemeName {
  const idx = THEME_CYCLE.indexOf(current);
  return THEME_CYCLE[(idx + 1) % THEME_CYCLE.length];
}

const noop = () => () => {};

export function AnimatedThemeToggle({ elevation, spring }: { elevation?: ElevationOverride; spring?: SpringMode } = {}) {
  const { resolvedTheme, setTheme } = useTheme();
  const buttonRef = useRef<HTMLButtonElement>(null);
  const mounted = useSyncExternalStore(noop, () => true, () => false);

  const current = mounted
    ? ((resolvedTheme ?? "light") as ThemeName)
    : undefined;

  const toggleTheme = useCallback(() => {
    if (!current || !buttonRef.current) {
      if (current) setTheme(nextTheme(current));
      return;
    }
    const target = nextTheme(current);
    animateThemeTransition(buttonRef.current, () => setTheme(target));
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
      <SunIcon className="absolute size-4 scale-0 rotate-90 transition-all group-data-[theme=light]:scale-100 group-data-[theme=light]:rotate-0" />
      <SunsetIcon className="absolute size-4 scale-0 rotate-90 transition-all group-data-[theme=sunset]:scale-100 group-data-[theme=sunset]:rotate-0" />
      <MoonIcon className="absolute size-4 scale-0 -rotate-90 transition-all group-data-[theme=dark]:scale-100 group-data-[theme=dark]:rotate-0" />
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}
