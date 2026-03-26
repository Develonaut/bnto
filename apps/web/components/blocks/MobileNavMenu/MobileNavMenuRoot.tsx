"use client";

import { Sheet, SheetContent } from "@bnto/ui";
import { MobileNavSheetContent } from "./MobileNavSheetContent";

interface MobileNavMenuProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function MobileNavMenuRoot({ open, onOpenChange }: MobileNavMenuProps) {
  function handleClose() {
    onOpenChange(false);
  }

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent
        data-testid="mobile-nav-dialog"
        aria-describedby={undefined}
        side="top"
        className="inset-0 h-dvh w-full bg-primary text-primary-foreground [&>button]:hidden"
      >
        <MobileNavSheetContent onClose={handleClose} />
      </SheetContent>
    </Sheet>
  );
}
