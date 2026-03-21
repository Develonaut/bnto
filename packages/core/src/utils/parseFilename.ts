/** Parse a filename into base name and extension. */

interface ParsedFilename {
  /** Filename without extension (e.g. "photo" from "photo.jpg"). */
  base: string;
  /** Lowercase extension without dot (e.g. "jpg"), or null if none. */
  extension: string | null;
  /** Extension with dot (e.g. ".jpg"), or empty string if none. */
  dotExtension: string;
}

function parseFilename(filename: string): ParsedFilename {
  const dot = filename.lastIndexOf(".");
  if (dot <= 0 || dot === filename.length - 1) {
    return { base: filename, extension: null, dotExtension: "" };
  }
  return {
    base: filename.slice(0, dot),
    extension: filename.slice(dot + 1).toLowerCase(),
    dotExtension: filename.slice(dot),
  };
}

export { parseFilename };
export type { ParsedFilename };
