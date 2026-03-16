/**
 * serializeDraft — create a Draft from the current editor state.
 *
 * Takes the exported Definition and recipe metadata,
 * stamps the current time, and returns a Draft.
 */

import type { Definition } from "@bnto/nodes";
import type { RecipeMetadata } from "../store/types";
import type { Draft } from "./draftTypes";

function serializeDraft(
  definition: Definition,
  metadata: RecipeMetadata,
  syncedAt?: number | null,
): Draft {
  return {
    definition,
    metadata,
    savedAt: Date.now(),
    syncedAt: syncedAt ?? null,
  };
}

export { serializeDraft };
