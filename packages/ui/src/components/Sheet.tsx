import {
  Sheet as PrimitiveSheet,
  SheetTrigger,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
  SheetClose,
} from "../primitives/sheet";
import { type ComponentProps } from "react";

function SheetRoot(props: ComponentProps<typeof PrimitiveSheet>) {
  return <PrimitiveSheet {...props} />;
}

export const Sheet = Object.assign(SheetRoot, {
  Root: SheetRoot,
  Trigger: SheetTrigger,
  Content: SheetContent,
  Header: SheetHeader,
  Title: SheetTitle,
  Description: SheetDescription,
  Close: SheetClose,
});
