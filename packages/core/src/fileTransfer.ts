/**
 * Module-scoped file transfer slot.
 *
 * Bridges the gap between recipe flow pages and the editor — File objects
 * can't be serialized to storage, but module memory survives SPA navigation.
 * The flow page stashes files before router.push; the editor claims them on mount.
 */

const pending = new Map<string, File[]>();

/** Store files for a recipe ID (no-op if empty). */
export function stashFilesForTransfer(recipeId: string, files: File[]) {
  if (files.length > 0) pending.set(recipeId, files);
}

/** Return files for a recipe ID and clear the slot (one-shot). */
export function claimTransferredFiles(recipeId: string): File[] {
  const files = pending.get(recipeId) ?? [];
  pending.delete(recipeId);
  return files;
}
