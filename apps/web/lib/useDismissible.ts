"use client";

import { useCallback, useState } from "react";

/**
 * Persistent dismiss state backed by localStorage.
 *
 * Returns whether the item has been dismissed and a function to dismiss it.
 * SSR-safe — defaults to dismissed (hidden) on the server to avoid flash.
 */
export function useDismissible(key: string) {
  const [dismissed, setDismissed] = useState(() => {
    if (typeof window === "undefined") return true;
    return localStorage.getItem(key) === "true";
  });

  const dismiss = useCallback(() => {
    localStorage.setItem(key, "true");
    setDismissed(true);
  }, [key]);

  return { dismissed, dismiss } as const;
}
