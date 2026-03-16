/**
 * Draft types — shape of a persisted editor draft.
 *
 * A Draft captures the minimum needed to restore the editor:
 * the exported Definition, recipe metadata, and a timestamp.
 */

import type { Definition } from "@bnto/nodes";
import type { RecipeMetadata } from "../store/types";

interface Draft {
  definition: Definition;
  metadata: RecipeMetadata;
  savedAt: number;
  /** When last synced to Convex. null = never synced. */
  syncedAt: number | null;
}

export type { Draft };
