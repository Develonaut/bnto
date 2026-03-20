/** Returns the UI field config map for a node type, if one exists. */

import type { NodeParamFields } from "./types";
import { NODE_PARAM_FIELDS } from "./registry";

function getNodeParamFields(typeName: string): NodeParamFields | undefined {
  return NODE_PARAM_FIELDS[typeName as keyof typeof NODE_PARAM_FIELDS];
}

export { getNodeParamFields };
