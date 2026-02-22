/**
 * Auth error event bus.
 *
 * Wired into React Query's query cache and mutation cache. When any query
 * or mutation fails with an auth error, listeners are notified. The app
 * wires this to redirect to /signin (core doesn't own navigation).
 */

type AuthErrorListener = () => void;
const listeners = new Set<AuthErrorListener>();

/** Subscribe to auth errors. Returns an unsubscribe function. */
export function onAuthError(listener: AuthErrorListener) {
  listeners.add(listener);
  return () => {
    listeners.delete(listener);
  };
}

/** Notify all listeners that an auth error occurred. */
export function emitAuthError() {
  listeners.forEach((fn) => fn());
}

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
