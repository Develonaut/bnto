"use client";

import { useCallback } from "react";

import { useTheme } from "@/components/useTheme";

import { Menu, MenuTrigger, MenuContent, SunIcon } from "@bnto/ui";

import { THEME_STORE_DEFAULT_ANGLE, useThemeStore } from "@/lib/stores/themeStore";
import { NavThemeMenuContent } from "./NavThemeMenuContent";

/* ── Helpers ──────────────────────────────────────────────────── */

const THEME_NAMES: Record<string, string> = {
  light: "Los Angeles",
  sunset: "Monaco",
  dark: "Tokyo",
};

/* ── NavThemeMenu ─────────────────────────────────────────────── */

export function NavThemeMenu() {
  const lightAngle = useThemeStore((s) => s.lightAngle);
  const setLightAngle = useThemeStore((s) => s.setLightAngle);
  const { setTheme, resolvedTheme } = useTheme();

  const isDefault = lightAngle === THEME_STORE_DEFAULT_ANGLE && resolvedTheme === "light";
  const themeName = THEME_NAMES[resolvedTheme ?? "light"] ?? "Los Angeles";

  const handleReset = useCallback(() => {
    setLightAngle(THEME_STORE_DEFAULT_ANGLE);
    setTheme("light");
  }, [setLightAngle, setTheme]);

  return (
    <Menu>
      <MenuTrigger variant="outline" size="icon" elevation="sm">
        <SunIcon />
        <span className="sr-only">Theme settings</span>
      </MenuTrigger>
      <MenuContent className="w-auto p-4" offset="lg">
        <NavThemeMenuContent
          lightAngle={lightAngle}
          setLightAngle={setLightAngle}
          themeName={themeName}
          isDefault={isDefault}
          onReset={handleReset}
        />
      </MenuContent>
    </Menu>
  );
}
