/**
 * Console-activated feature flags.
 *
 * Flags are backed by localStorage for persistence across refreshes.
 * Enable via browser console: `__bnto__.flags.set("editor", true)`
 */

const STORAGE_KEY = "bnto:flags";
const CHANGE_EVENT = "bnto:flags-changed";

/** All flags default to false. Add new flags here. */
const DEFAULTS = { editor: false } as const;

export type FeatureFlag = keyof typeof DEFAULTS;
type FlagValues = Record<FeatureFlag, boolean>;

function read(): FlagValues {
  if (typeof window === "undefined") return { ...DEFAULTS };
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return { ...DEFAULTS, ...(raw ? JSON.parse(raw) : {}) };
  } catch {
    return { ...DEFAULTS };
  }
}

function write(values: FlagValues) {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(values));
  window.dispatchEvent(new Event(CHANGE_EVENT));
}

export function getFlag(flag: FeatureFlag): boolean {
  return read()[flag];
}

export function setFlag(flag: FeatureFlag, value: boolean) {
  const current = read();
  current[flag] = value;
  write(current);
}

export function listFlags(): FlagValues {
  return read();
}

// --- React hook: reactive rendering via useSyncExternalStore ---

import { useSyncExternalStore } from "react";

export function useFeatureFlag(flag: FeatureFlag): boolean {
  return useSyncExternalStore(
    (cb) => {
      window.addEventListener(CHANGE_EVENT, cb);
      window.addEventListener("storage", cb);
      return () => {
        window.removeEventListener(CHANGE_EVENT, cb);
        window.removeEventListener("storage", cb);
      };
    },
    () => getFlag(flag),
    () => DEFAULTS[flag],
  );
}

// --- Console API: window.__bnto__ ---

declare global {
  interface Window {
    __bnto__?: {
      flags: { get: typeof getFlag; set: typeof setFlag; list: typeof listFlags };
    };
  }
}

if (typeof window !== "undefined") {
  window.__bnto__ = {
    flags: { get: getFlag, set: setFlag, list: listFlags },
  };
}
