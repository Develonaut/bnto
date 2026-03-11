/**
 * Panel client — thin passthrough to panelService.
 */

import type { PanelService, PanelClient } from "../editorTypes";

function createPanelClient(panelService: PanelService): PanelClient {
  return { ...panelService };
}

export { createPanelClient };
