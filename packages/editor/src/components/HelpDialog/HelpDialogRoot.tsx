"use client";

import { useMemo } from "react";
import { DialogShell, Stack, Text } from "@bnto/ui";
import { isMacPlatform } from "../../utils/isMacPlatform";
import { ShortcutList } from "./ShortcutList";

interface HelpDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

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
        <ShortcutList isMac={isMac} />
      </Stack>
    </DialogShell>
  );
}

export { HelpDialogRoot };
