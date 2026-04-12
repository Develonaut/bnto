"use client";

import { useSyncExternalStore } from "react";

function subscribe(cb: () => void) {
  window.addEventListener("resize", cb);
  return () => window.removeEventListener("resize", cb);
}

function getSnapshot() {
  const w = window.innerWidth;
  if (w >= 1024) return 3;
  if (w >= 640) return 2;
  return 1;
}

function getServerSnapshot() {
  return 1;
}

/** Returns 1 (mobile), 2 (tablet), or 3 (desktop) based on window width. */
export function useColumnCount() {
  return useSyncExternalStore(subscribe, getSnapshot, getServerSnapshot);
}
