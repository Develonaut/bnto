"use client";

import { useMemo } from "react";
import { Kbd, KbdGroup } from "@bnto/ui";
import { SHORTCUT_MAP } from "../utils/shortcuts";
import { getShortcutKeys } from "../utils/getShortcutKeys";

interface ShortcutHintProps {
  /** Shortcut id from SHORTCUTS definitions. */
  shortcutId: string;
}

/**
 * Right-aligned keyboard shortcut hint for menu items.
 *
 * Renders platform-aware Kbd keys (⌘ on Mac, Ctrl on Windows).
 * Uses `ml-auto` to push itself to the right edge of the flex parent.
 */
export function ShortcutHint({ shortcutId }: ShortcutHintProps) {
  const def = SHORTCUT_MAP.get(shortcutId);

  const keys = useMemo(() => {
    if (!def) return [];
    return getShortcutKeys(def);
  }, [def]);

  if (!keys.length) return null;

  return (
    <KbdGroup className="ml-auto">
      {keys.map((key) => (
        <Kbd key={key}>{key}</Kbd>
      ))}
    </KbdGroup>
  );
}
