/** Processor lookup functions — stateless, reads from @bnto/nodes static exports. */

import { PROCESSORS } from "@bnto/nodes";
import type { ProcessorDef } from "@bnto/nodes";

/** Returns all engine processor definitions. */
export function getAllProcessors(): readonly ProcessorDef[] {
  return PROCESSORS;
}
