/**
 * Map a bnto slug to the file types it accepts.
 *
 * Used by the file drop zone to filter the file input and
 * validate dropped files. Returns `accept` (for <input accept>)
 * and `label` (human-readable description for the drop zone hint).
 */

interface AcceptedTypes {
  /** Value for the HTML <input accept="..."> attribute. */
  accept: string;
  /** Human-readable label shown in the drop zone. */
  label: string;
  /** MIME type prefixes to validate against (e.g. "image/"). */
  mimePrefix?: string;
}

const SLUG_ACCEPT_MAP: Record<string, AcceptedTypes> = {
  "compress-images": {
    accept: "image/jpeg,image/png,image/webp",
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  "resize-images": {
    accept: "image/jpeg,image/png,image/webp",
    label: "JPEG, PNG, or WebP images",
    mimePrefix: "image/",
  },
  "convert-image-format": {
    accept: "image/jpeg,image/png,image/webp,image/gif",
    label: "JPEG, PNG, WebP, or GIF images",
    mimePrefix: "image/",
  },
  "rename-files": {
    accept: "*/*",
    label: "any files",
  },
  "clean-csv": {
    accept: ".csv,text/csv",
    label: "CSV files",
  },
  "rename-csv-columns": {
    accept: ".csv,text/csv",
    label: "CSV files",
  },
};

const DEFAULT_ACCEPTED: AcceptedTypes = {
  accept: "*/*",
  label: "files",
};

export function getAcceptedTypes(slug: string): AcceptedTypes {
  return SLUG_ACCEPT_MAP[slug] ?? DEFAULT_ACCEPTED;
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
