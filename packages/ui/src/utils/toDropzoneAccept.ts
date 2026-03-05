/**
 * Convert an accept string to react-dropzone's Accept format.
 *
 * Returns undefined for wildcard accept strings — react-dropzone
 * accepts all files when the accept prop is omitted.
 */
export function toDropzoneAccept(
  accept: string,
): Record<string, string[]> | undefined {
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
