"use client";

import { Button, DialogBody, DialogClose, DialogFooter, Input, Label } from "@bnto/ui";

interface RenameRecipeFormProps {
  nameInputId: string;
  name: string;
  onNameChange: (value: string) => void;
  hasChanges: boolean;
  onSubmit: (e: React.FormEvent) => void;
}

function RenameFormFooter({ hasChanges }: { hasChanges: boolean }) {
  return (
    <DialogFooter>
      <DialogClose asChild>
        <Button variant="ghost">Cancel</Button>
      </DialogClose>
      <Button type="submit" disabled={!hasChanges}>
        Save
      </Button>
    </DialogFooter>
  );
}

/** Inner form for the rename dialog — name input + cancel/save buttons. */
export function RenameRecipeForm({
  nameInputId,
  name,
  onNameChange,
  hasChanges,
  onSubmit,
}: RenameRecipeFormProps) {
  return (
    <form onSubmit={onSubmit}>
      <DialogBody>
        <fieldset className="space-y-1.5">
          <Label htmlFor={nameInputId}>Name</Label>
          <Input
            id={nameInputId}
            value={name}
            onChange={(e) => onNameChange(e.target.value)}
            placeholder="Recipe name"
            autoFocus
          />
        </fieldset>
      </DialogBody>
      <RenameFormFooter hasChanges={hasChanges} />
    </form>
  );
}
