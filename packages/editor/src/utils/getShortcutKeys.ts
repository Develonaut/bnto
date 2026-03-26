/**
 * Resolve platform-aware display keys for a keyboard shortcut.
 */

import type { ShortcutDef } from "./shortcuts";
import { isMacPlatform } from "./isMacPlatform";

/** Return the display keys for a shortcut based on the current platform. */
export function getShortcutKeys(def: ShortcutDef): string[] {
  return isMacPlatform() ? def.mac : def.win;
}
