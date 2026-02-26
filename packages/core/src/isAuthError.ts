/** Returns true if the error looks like an authentication failure. */
export function isAuthError(error: unknown): boolean {
  if (error instanceof Error) {
    const msg = error.message.toLowerCase();
    return (
      msg.includes("not authenticated") ||
      msg.includes("unauthenticated") ||
      msg.includes("unauthorized")
    );
  }
  return false;
}
