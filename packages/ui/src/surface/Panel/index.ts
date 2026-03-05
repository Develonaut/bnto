import { PanelRoot } from "./PanelRoot";
import { PanelHeader } from "./PanelHeader";
import { PanelTrigger } from "./PanelTrigger";
import { PanelDivider } from "./PanelDivider";
import { PanelContent } from "./PanelContent";
import { PanelFooter } from "./PanelFooter";

export const Panel = Object.assign(PanelRoot, {
  Root: PanelRoot,
  Header: PanelHeader,
  Trigger: PanelTrigger,
  Divider: PanelDivider,
  Content: PanelContent,
  Footer: PanelFooter,
});
