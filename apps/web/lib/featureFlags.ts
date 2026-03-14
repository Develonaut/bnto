/**
 * Console-activated feature flags.
 *
 * Flags are backed by localStorage for persistence across refreshes.
 * Enable via browser console: `__bnto__.flags.set("flagName", true)`
 *
 * Add new flags to DEFAULTS below. All flags default to false.
 */

const STORAGE_KEY = "bnto:flags";
const CHANGE_EVENT = "bnto:flags-changed";

/** All flags default to false. Add new flags here as needed. */
const DEFAULTS: Record<string, boolean> = {};

export type FeatureFlag = string;
type FlagValues = Record<string, boolean>;

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
  return read()[flag] ?? false;
}

export function setFlag(flag: FeatureFlag, value: boolean) {
  const current = read();
  current[flag] = value;
  write(current);
}

export function listFlags(): FlagValues {
  return read();
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
