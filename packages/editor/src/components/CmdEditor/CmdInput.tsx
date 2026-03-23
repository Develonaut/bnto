"use client";

import {
  Command,
  CommandInput,
  CommandList,
  CommandEmpty,
  CommandGroup,
  CommandItem,
} from "@bnto/ui";

/**
 * CmdInput — command palette stub using cmdk.
 *
 * Phase 1: static shell with no real commands. Renders the input
 * and an empty list. Real command wiring happens in Phase 3.
 */
function CmdInput() {
  return (
    <Command data-testid="cmd-input">
      <CommandInput placeholder="Type a command..." />
      <CommandList>
        <CommandEmpty>No commands yet.</CommandEmpty>
        <CommandGroup heading="Actions">
          <CommandItem disabled>Commands will appear here...</CommandItem>
        </CommandGroup>
      </CommandList>
    </Command>
  );
}

export { CmdInput };
