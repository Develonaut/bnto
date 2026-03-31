"use client";

import { DialogShell, DialogBody, useDialog, Button, SlidersHorizontalIcon } from "@bnto/ui";
import { RecipeStepperConfigDialogContent } from "./RecipeStepperConfigDialogContent";

/** Icon button that opens the recipe config dialog. */
export function RecipeStepperConfigButton() {
  const { open, onOpenChange, openDialog } = useDialog();

  return (
    <>
      <Button
        variant="outline"
        size="icon"
        onClick={openDialog}
        aria-label="Configure recipe"
        data-testid="config-button"
      >
        <SlidersHorizontalIcon className="size-4" />
      </Button>
      <DialogShell
        open={open}
        onOpenChange={onOpenChange}
        title="Configure"
        description="Adjust recipe parameters before running"
        size="sm"
      >
        <DialogBody disablePadding className="-mx-8 min-h-0 overflow-y-auto px-8 py-4">
          <RecipeStepperConfigDialogContent />
        </DialogBody>
      </DialogShell>
    </>
  );
}
