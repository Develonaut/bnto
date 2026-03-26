"use client";

import { Accordion, DialogShell, Text } from "@bnto/ui";
import type { PaletteGroup } from "../../hooks/useNodePalette";
import { SLOTS } from "../../adapters/bentoSlots";
import { PaletteCategoryGroup } from "./PaletteCategoryGroup";
import { PaletteSearchInput } from "./PaletteSearchInput";
import { useNodePaletteDialog } from "./useNodePaletteDialog";

interface NodePaletteDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

function NodePaletteDialogRoot({ open, onOpenChange }: NodePaletteDialogProps) {
  const { search, setSearch, isFull, filteredGroups, defaultExpanded, handleClose, handleAdd } =
    useNodePaletteDialog(onOpenChange);

  return (
    <DialogShell
      open={open}
      onOpenChange={handleClose}
      title="Add Node"
      description="Search and select a node type to add to the canvas."
      size="lg"
      contentProps={{ "data-testid": "node-palette-dialog" }}
      headerClassName="pb-2"
    >
      <PaletteDialogBody
        search={search}
        setSearch={setSearch}
        isFull={isFull}
        filteredGroups={filteredGroups}
        defaultExpanded={defaultExpanded}
        onAdd={handleAdd}
      />
    </DialogShell>
  );
}

interface PaletteDialogBodyProps {
  search: string;
  setSearch: (value: string) => void;
  isFull: boolean;
  filteredGroups: PaletteGroup[];
  defaultExpanded: string[];
  onAdd: (type: string) => void;
}

/** Accordion list of palette groups + empty search message. */
function PaletteGroupList({
  filteredGroups,
  defaultExpanded,
  isFull,
  onAdd,
  search,
}: {
  filteredGroups: PaletteGroup[];
  defaultExpanded: string[];
  isFull: boolean;
  onAdd: (type: string) => void;
  search: string;
}) {
  return (
    <>
      <Accordion
        type="multiple"
        defaultValue={defaultExpanded}
        className="-mr-8 min-h-0 flex-1 overflow-x-hidden overflow-y-auto border-t border-border pr-8 pt-3"
      >
        {filteredGroups.map((group) => (
          <PaletteCategoryGroup
            key={group.category.name}
            group={group}
            isFull={isFull}
            onAdd={onAdd}
          />
        ))}
      </Accordion>
      {filteredGroups.length === 0 && search.trim() && (
        <Text size="xs" color="muted" className="px-3 py-4 text-center">
          No nodes match &ldquo;{search}&rdquo;
        </Text>
      )}
    </>
  );
}

function PaletteDialogBody({
  search,
  setSearch,
  isFull,
  filteredGroups,
  defaultExpanded,
  onAdd,
}: PaletteDialogBodyProps) {
  return (
    <div className="flex h-[28rem] flex-col">
      <PaletteSearchInput value={search} onChange={setSearch} />
      {isFull && (
        <Text size="xs" color="muted" className="pb-2">
          Canvas is full ({SLOTS.length} nodes max).
        </Text>
      )}
      <PaletteGroupList
        filteredGroups={filteredGroups}
        defaultExpanded={defaultExpanded}
        isFull={isFull}
        onAdd={onAdd}
        search={search}
      />
    </div>
  );
}

export { NodePaletteDialogRoot };
