"use client";

import { useCallback } from "react";

import { Sheet, SheetContent } from "@bnto/ui";
import { MobileNavSheetContent } from "./MobileNavSheetContent";

interface MobileNavMenuProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function MobileNavMenuRoot({ open, onOpenChange }: MobileNavMenuProps) {
  const handleClose = useCallback(() => {
    onOpenChange(false);
  }, [onOpenChange]);

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
