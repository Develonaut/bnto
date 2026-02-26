"use client";

/**
 * Re-export useTheme from next-themes.
 *
 * Per theming.md: import theme components from `@/components/`, never
 * from `next-themes` directly. This wrapper ensures we can swap the
 * underlying theme library without touching consumer code.
 */
export { useTheme } from "next-themes";
