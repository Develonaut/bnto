"use client";

import { LayerPanelRoot } from "./LayerPanelRoot";
import { LayerPanelTrigger } from "./LayerPanelTrigger";

export const LayerPanel = Object.assign(LayerPanelRoot, {
  Root: LayerPanelRoot,
  Trigger: LayerPanelTrigger,
});
