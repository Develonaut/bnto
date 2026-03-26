"use client";

import { useMemo } from "react";
import { DialogShell, Kbd, KbdGroup, Row, Stack, Text } from "@bnto/ui";
import { SHORTCUTS } from "../../utils/shortcuts";
import { isMacPlatform } from "../../utils/isMacPlatform";
import { getShortcutKeys } from "../../utils/getShortcutKeys";

interface HelpDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

/**
 * Help dialog — lists keyboard shortcuts and general editor help.
 *
 * Shows platform-aware key combinations (⌘ on Mac, Ctrl on Windows).
 * Opened via the toolbar help button or ⌘/ shortcut.
 */
function HelpDialogRoot({ open, onOpenChange }: HelpDialogProps) {
  const isMac = useMemo(() => isMacPlatform(), []);

  return (
    <DialogShell
      open={open}
      onOpenChange={onOpenChange}
      title="Help"
      description="Keyboard shortcuts for the recipe editor."
      size="sm"
    >
      <Stack className="gap-1 py-2">
        <Text size="xs" className="px-4 pb-1 text-muted-foreground uppercase tracking-wider">
          Keyboard Shortcuts
        </Text>
        {SHORTCUTS.map((shortcut) => {
          const keys = isMac ? shortcut.mac : getShortcutKeys(shortcut);
          return (
            <Row key={shortcut.id} className="items-center justify-between px-4 py-2">
              <Text size="sm">{shortcut.label}</Text>
              <KbdGroup>
                {keys.map((key) => (
                  <Kbd key={key}>{key}</Kbd>
                ))}
              </KbdGroup>
            </Row>
          );
        })}
      </Stack>
    </DialogShell>
  );
}

export { HelpDialogRoot };
