/**
 * Node client — composes nodeService for the public API.
 *
 * selectNode already handles cross-domain atomicity (writes both
 * node selection AND panel auto-open/close in a single store action).
 * The client delegates directly — no additional composition needed.
 */

import type { NodeService, NodeClient } from "../editorTypes";

function createNodeClient(nodeService: NodeService): NodeClient {
  return { ...nodeService };
}

export { createNodeClient };
