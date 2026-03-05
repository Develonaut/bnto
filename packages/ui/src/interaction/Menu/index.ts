import { Popover } from "../../overlay/Popover";

import { MenuRootWrapper } from "./MenuRootWrapper";
import { MenuTrigger } from "./MenuTrigger";
import { MenuContent } from "./MenuContent";
import { MenuItem } from "./MenuItem";
import { MenuLabel } from "./MenuLabel";
import { MenuSeparator } from "./MenuSeparator";

/** Closes the menu when its child is clicked. Renders no wrapper element. */
const MenuClose = Popover.Close;

export const Menu = Object.assign(MenuRootWrapper, {
  Root: MenuRootWrapper,
  Trigger: MenuTrigger,
  Content: MenuContent,
  Close: MenuClose,
  Item: MenuItem,
  Label: MenuLabel,
  Separator: MenuSeparator,
});
