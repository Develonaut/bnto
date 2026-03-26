/** Merge cloud recipes into the local recipes map, deduplicating by cloudId. */

import type { UserRecipe } from "../types/recipe";

export function mergeCloudRecipes(local: Record<string, UserRecipe>, incoming: UserRecipe[]): void {
  const existingCloudIds = new Set<string>();
  for (const recipe of Object.values(local)) {
    if (recipe.cloudId) existingCloudIds.add(recipe.cloudId);
  }

  for (const r of incoming) {
    if (r.cloudId && existingCloudIds.has(r.cloudId)) continue;

    const existing = local[r.id];
    if (!existing) {
      local[r.id] = r;
    } else if (!existing.cloudId && r.cloudId) {
      local[r.id] = { ...existing, cloudId: r.cloudId, syncedAt: r.syncedAt ?? Date.now() };
    }
  }
}
