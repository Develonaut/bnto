"use client";

import * as React from "react";

import { ThemeProvider as NextThemesProvider } from "next-themes";

import { THEME_STORE_KEY, useThemeStore } from "@/lib/stores/themeStore";

/**
 * Blocking script that reads the persisted light angle from localStorage
 * and applies it as a CSS variable BEFORE React hydrates. This prevents
 * a flash of the wrong shadow direction on first paint.
 *
 * Rendered inline by ThemeProvider so consumers don't need to manually
 * add anything to their <head>. Same pattern as next-themes' ThemeScript.
 */
const FOUC_SCRIPT = `try{var t=JSON.parse(localStorage.getItem("${THEME_STORE_KEY}"));if(t&&t.state&&t.state.lightAngle!=null){document.documentElement.style.setProperty("--light-angle",t.state.lightAngle+"deg")}}catch(e){}`;

/** Syncs the Zustand theme store (light angle) to a CSS variable on :root. */
function ThemeStoreSync() {
  const lightAngle = useThemeStore((s) => s.lightAngle);

  React.useEffect(() => {
    document.documentElement.style.setProperty(
      "--light-angle",
      `${lightAngle}deg`,
    );
  }, [lightAngle]);

  return null;
}

export function ThemeProvider({
  children,
  ...props
}: React.ComponentProps<typeof NextThemesProvider>) {
  return (
    <NextThemesProvider {...props}>
      <script dangerouslySetInnerHTML={{ __html: FOUC_SCRIPT }} />
      <ThemeStoreSync />
      {children}
    </NextThemesProvider>
  );
}
