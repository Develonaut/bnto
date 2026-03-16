/**
 * Keyboard shortcut definitions for the editor.
 *
 * Each shortcut has a unique id, human-readable label, description,
 * and platform-aware display keys (Mac vs Windows/Linux).
 */

export interface ShortcutDef {
  id: string;
  label: string;
  description: string;
  /** Mac display keys (e.g. ["⌘", "Z"]) */
  mac: string[];
  /** Windows/Linux display keys (e.g. ["Ctrl", "Z"]) */
  win: string[];
}

export const SHORTCUTS: ShortcutDef[] = [
  {
    id: "undo",
    label: "Undo",
    description: "Undo the last action",
    mac: ["⌘", "Z"],
    win: ["Ctrl", "Z"],
  },
  {
    id: "redo",
    label: "Redo",
    description: "Redo the last undone action",
    mac: ["⌘", "⇧", "Z"],
    win: ["Ctrl", "Shift", "Z"],
  },
  {
    id: "delete",
    label: "Delete node",
    description: "Delete the selected node",
    mac: ["⌫"],
    win: ["Del"],
  },
  {
    id: "run",
    label: "Run",
    description: "Run the recipe",
    mac: ["⌘", "↵"],
    win: ["Ctrl", "↵"],
  },
  {
    id: "export",
    label: "Export",
    description: "Download the recipe as .bnto.json",
    mac: ["⌘", "D"],
    win: ["Ctrl", "D"],
  },
  {
    id: "escape",
    label: "Escape",
    description: "Close panels / deselect node",
    mac: ["Esc"],
    win: ["Esc"],
  },
  {
    id: "help",
    label: "Help",
    description: "Open the help dialog",
    mac: ["⌘", "/"],
    win: ["Ctrl", "/"],
  },
];

/** Map from shortcut id to its definition for quick lookup. */
export const SHORTCUT_MAP = new Map(SHORTCUTS.map((s) => [s.id, s]));

/**
 * Detect whether the user is on macOS.
 *
 * Uses `navigator.platform` with a `navigator.userAgentData` fallback.
 * Returns false during SSR.
 */
export function isMacPlatform(): boolean {
  if (typeof navigator === "undefined") return false;

  // Modern API (Chromium 93+)
  const uaData = (navigator as Navigator & { userAgentData?: { platform?: string } }).userAgentData;
  if (uaData?.platform) return uaData.platform === "macOS";

  // Legacy fallback
  return /Mac|iPhone|iPad|iPod/.test(navigator.platform);
}

/** Return the display keys for a shortcut based on the current platform. */
export function getShortcutKeys(def: ShortcutDef): string[] {
  return isMacPlatform() ? def.mac : def.win;
}
