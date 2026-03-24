/**
 * treeNavigation — pure functions for arrow-key tree traversal.
 *
 * Given a flat traversal order and the current selection,
 * computes the next selection target. No React, no store.
 */

type TreeNavAction = "up" | "down" | "first" | "last";

/** Returns the next node ID to select, or null if movement is blocked. */
function resolveTreeNav(
  flatIds: string[],
  selectedId: string | null,
  action: TreeNavAction,
): string | null {
  if (flatIds.length === 0) return null;

  if (action === "first") return flatIds[0]!;
  if (action === "last") return flatIds[flatIds.length - 1]!;

  if (selectedId === null) {
    return action === "down" ? flatIds[0]! : flatIds[flatIds.length - 1]!;
  }

  const index = flatIds.indexOf(selectedId);
  if (index === -1) return flatIds[0]!;

  if (action === "down") {
    return index < flatIds.length - 1 ? flatIds[index + 1]! : null;
  }
  // action === "up"
  return index > 0 ? flatIds[index - 1]! : null;
}

export { resolveTreeNav };
export type { TreeNavAction };
