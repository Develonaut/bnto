/** Returns the UI field config map for a node type, if one exists. */

import type { FieldConfigMap } from "./types";
import { NODE_FIELD_CONFIGS } from "./registry";

function getNodeFields(typeName: string): FieldConfigMap | undefined {
  return NODE_FIELD_CONFIGS[typeName as keyof typeof NODE_FIELD_CONFIGS];
}

export { getNodeFields };
