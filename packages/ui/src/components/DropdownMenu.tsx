import {
  DropdownMenu as PrimitiveDropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuGroup,
} from "../primitives/dropdown-menu";
import { type ComponentProps } from "react";

function DropdownMenuRoot(
  props: ComponentProps<typeof PrimitiveDropdownMenu>,
) {
  return <PrimitiveDropdownMenu {...props} />;
}

export const DropdownMenu = Object.assign(DropdownMenuRoot, {
  Root: DropdownMenuRoot,
  Trigger: DropdownMenuTrigger,
  Content: DropdownMenuContent,
  Item: DropdownMenuItem,
  Label: DropdownMenuLabel,
  Separator: DropdownMenuSeparator,
  Group: DropdownMenuGroup,
});
