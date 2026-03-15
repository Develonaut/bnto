"use client";

import { core } from "@bnto/core";

/**
 * Invisible marker that self-fetches the current user and exposes
 * `data-user-id` for E2E test observability.
 *
 * Extracted from useRecipeFlow so the recipe orchestrator doesn't
 * need to fetch user data — each concern owns its own data.
 */
export function SessionMarker() {
  const { data: currentUser } = core.user.useCurrentUser();
  return <span data-user-id={currentUser?.id ?? ""} hidden />;
}
