/**
 * Race running work against a recipe's whole-run time limit. Past the limit,
 * `onExpire` stops the work and the result fails with the engine's own
 * timeout message, so a hard stop reads the same as the engine's checks.
 */

/** Matches `BntoError::TimedOut`'s message in the Rust engine. */
function timedOutMessage(secs: number): string {
  return `Run timed out after ${secs}s`;
}

export function withTimeLimit<T>(
  work: Promise<T>,
  limitSecs: number | undefined,
  onExpire: () => void,
): Promise<T> {
  if (!limitSecs) return work;

  return new Promise<T>((resolve, reject) => {
    const timer = setTimeout(() => {
      reject(new Error(timedOutMessage(limitSecs)));
      onExpire();
    }, limitSecs * 1000);
    work.then(resolve, reject).finally(() => clearTimeout(timer));
  });
}
