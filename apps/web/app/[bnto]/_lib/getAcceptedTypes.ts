/**
 * Map a bnto slug to the file types it accepts.
 *
 * Reads from the engine-generated menu — no hardcoded slug map.
 * Used by the file drop zone to filter the file input and
 * validate dropped files.
 */

import { getRecipe } from "#lib/menu";

interface AcceptedTypes {
  /** Value for the HTML <input accept="..."> attribute. */
  accept: string;
  /** Human-readable label shown in the drop zone. */
  label: string;
  /** MIME type prefixes to validate against (e.g. "image/"). */
  mimePrefix?: string;
}

const DEFAULT_ACCEPTED: AcceptedTypes = {
  accept: "*/*",
  label: "files",
};

export function getAcceptedTypes(slug: string): AcceptedTypes {
  const recipe = getRecipe(slug);
  if (!recipe) return DEFAULT_ACCEPTED;

  const { mimeTypes, extensions, label, mimePrefix } = recipe.accept;

  // Wildcard — accept anything
  if (mimeTypes.length === 1 && mimeTypes[0] === "*/*") {
    return { accept: "*/*", label, mimePrefix };
  }

  // When a mimePrefix is set (e.g. "image/"), MIME types alone are sufficient
  // for filtering. Extensions are redundant and would bloat the accept string.
  // Include extensions only when they add value (e.g. ".csv" where MIME
  // detection by browsers is unreliable).
  const parts = mimePrefix ? mimeTypes : [...mimeTypes, ...extensions];
  const accept = parts.join(",");
  return { accept, label, mimePrefix };
}

/**
 * Check if a file matches the accepted types for a bnto slug.
 * Returns true if accepted, false if the file should be rejected.
 */
export function isFileAccepted(file: File, slug: string): boolean {
  const { accept, mimePrefix } = getAcceptedTypes(slug);
  if (accept === "*/*") return true;

  if (mimePrefix && file.type.startsWith(mimePrefix)) return true;

  const acceptedTypes = accept.split(",");
  for (const t of acceptedTypes) {
    if (t.startsWith(".") && file.name.toLowerCase().endsWith(t)) return true;
    if (file.type === t) return true;
  }

  return false;
}

/**
 * Convert a bnto slug's accepted types to react-dropzone's `Accept` format.
 *
 * Returns `undefined` for wildcard slugs (react-dropzone accepts all
 * files when `accept` is omitted).
 */
export function toDropzoneAccept(
  slug: string,
): Record<string, string[]> | undefined {
  const { accept } = getAcceptedTypes(slug);
  if (accept === "*/*") return undefined;

  const tokens = accept.split(",");
  const mimes: string[] = [];
  const extensions: string[] = [];

  for (const t of tokens) {
    if (t.startsWith(".")) extensions.push(t);
    else mimes.push(t);
  }

  const result: Record<string, string[]> = {};
  for (const mime of mimes) {
    result[mime] = [];
  }

  // Attach bare extensions to the first MIME key, or create a wildcard entry
  if (extensions.length > 0) {
    const firstMime = mimes[0];
    if (firstMime) {
      result[firstMime] = extensions;
    } else {
      result["application/octet-stream"] = extensions;
    }
  }

  return result;
}
