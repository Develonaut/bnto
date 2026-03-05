import { PopoverClose } from "../../overlay/Popover";

export { MenuRootWrapper as Menu } from "./MenuRootWrapper";
export { MenuTrigger } from "./MenuTrigger";
export { MenuContent } from "./MenuContent";
export { MenuItem } from "./MenuItem";
export { MenuLabel } from "./MenuLabel";
export { MenuSeparator } from "./MenuSeparator";

/** Closes the menu when its child is clicked. Renders no wrapper element. */
export const MenuClose = PopoverClose;
