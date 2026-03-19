/** Category lookup functions — stateless, reads from @bnto/nodes static exports. */

import { CATEGORIES } from "@bnto/nodes";
import type { CategoryInfo } from "@bnto/nodes";

/** Returns all node categories. */
export function getAllCategories(): readonly CategoryInfo[] {
  return CATEGORIES;
}
