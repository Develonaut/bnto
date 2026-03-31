import type { Recipe } from "@bnto/registry";

/** A user recipe stored in sessionStorage. */
export interface UserRecipe extends Recipe {
  /** Timestamp when last saved. */
  savedAt: number;
}
