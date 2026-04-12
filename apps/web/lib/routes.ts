/** Route definitions — single source of truth for all route paths. */

export const ROUTES = {
  home: "/",
  explore: "/explore",
  editor: "/editor",
} as const satisfies Record<string, string>;

/** Returns the editor URL for a saved recipe. */
export function editorUrl(recipeId: string): string {
  return `/editor?recipe=${recipeId}`;
}
