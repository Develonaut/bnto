"use client";

import { Button, Container, SheetClose, SheetTitle, Stack, XIcon } from "@bnto/ui";
import { MobileNavActions } from "./MobileNavActions";
import { MobileNavAuth } from "./MobileNavAuth";

interface MobileNavSheetContentProps {
  onClose: () => void;
}

/** Inner content of the mobile nav sheet — title, close, actions, auth. */
export function MobileNavSheetContent({ onClose }: MobileNavSheetContentProps) {
  return (
    <div className="flex-1 overflow-y-auto">
      <Container className="pb-12">
        <div className="absolute -m-px h-px w-px overflow-hidden border-0 p-0">
          <SheetTitle className="text-primary">Navigation</SheetTitle>
        </div>
        <div className="flex justify-end pt-5">
          <SheetClose asChild>
            <Button variant="secondary" size="icon">
              <XIcon />
              <span className="sr-only">Close menu</span>
            </Button>
          </SheetClose>
        </div>
        <Stack className="h-full justify-between gap-20 pt-16">
          <MobileNavActions onClose={onClose} />
          <Stack className="gap-6">
            <div className="h-px bg-primary-foreground/20" />
            <MobileNavAuth onClose={onClose} />
          </Stack>
        </Stack>
      </Container>
    </div>
  );
}
