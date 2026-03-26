/** Translate backend error strings into user-friendly messages. */
export function friendlyExecutionError(raw: string): string {
  if (raw.includes("file transit not configured"))
    return "The file processing server isn't fully configured. Please try again later.";
  if (raw.includes("timed out") || raw.includes("polling limit"))
    return "The execution took too long and was stopped. Try with fewer or smaller files.";
  return raw;
}
