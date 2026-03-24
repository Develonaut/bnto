"use client";

import { CommandItem, Kbd, KbdGroup, ChevronRightIcon } from "@bnto/ui";
import type { LucideIcon } from "@bnto/ui";
import { ICON_COMPONENTS } from "../../adapters/nodeIcons";
import type { Command } from "../../commands/types";

// ---------------------------------------------------------------------------
// Icon resolution — merges global + node-specific icons
// ---------------------------------------------------------------------------

import { PlayIcon, DownloadIcon, TrashIcon, PlusIcon } from "@bnto/ui";

const GLOBAL_ICONS: Record<string, LucideIcon> = {
  Play: PlayIcon,
  Download: DownloadIcon,
  Trash2: TrashIcon,
  Plus: PlusIcon,
};

function resolveIcon(icon: string | undefined): LucideIcon | undefined {
  if (!icon) return undefined;
  return GLOBAL_ICONS[icon] ?? ICON_COMPONENTS[icon];
}

// ---------------------------------------------------------------------------
// CommandItemRow
// ---------------------------------------------------------------------------

/** Parses a shortcut string like "⌘⇧Z" into individual key tokens. */
function parseShortcut(shortcut: string): string[] {
  const keys: string[] = [];
  for (const char of shortcut) {
    keys.push(char);
  }
  return keys;
}

function CommandItemRow({
  command,
  drillable,
  onSelect,
}: {
  command: Command;
  drillable: boolean;
  onSelect: () => void;
}) {
  const Icon = resolveIcon(command.icon);

  return (
    <CommandItem
      value={[command.label, ...(command.keywords ?? [])].join(" ")}
      disabled={command.disabled}
      onSelect={onSelect}
      data-testid={`cmd-${command.id}`}
    >
      {Icon && <Icon className="size-4 text-muted-foreground" />}
      <span className="flex-1">{command.label}</span>
      {command.shortcut && (
        <KbdGroup className="ml-auto">
          {parseShortcut(command.shortcut).map((key, i) => (
            <Kbd key={i}>{key}</Kbd>
          ))}
        </KbdGroup>
      )}
      {drillable && !command.shortcut && (
        <ChevronRightIcon className="ml-auto size-4 text-muted-foreground" />
      )}
    </CommandItem>
  );
}

export { CommandItemRow };
