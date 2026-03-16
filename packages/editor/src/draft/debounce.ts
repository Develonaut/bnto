/**
 * debounce — generalized debounce factory.
 *
 * Returns schedule/flush/cancel/destroy methods.
 * schedule() resets the timer with a new callback;
 * flush() fires immediately; cancel() clears the pending timer;
 * destroy() cancels and prevents future scheduling.
 */

interface Debounced {
  schedule: (fn: () => void) => void;
  flush: () => void;
  cancel: () => void;
  destroy: () => void;
}

function debounce(delayMs: number): Debounced {
  let timerId: ReturnType<typeof setTimeout> | null = null;
  let pending: (() => void) | null = null;
  let destroyed = false;

  function cancel() {
    if (timerId !== null) {
      clearTimeout(timerId);
      timerId = null;
    }
    pending = null;
  }

  function flush() {
    if (pending) {
      const fn = pending;
      cancel();
      fn();
    }
  }

  function schedule(fn: () => void) {
    if (destroyed) return;
    cancel();
    pending = fn;
    timerId = setTimeout(() => {
      const toRun = pending;
      timerId = null;
      pending = null;
      toRun?.();
    }, delayMs);
  }

  function destroy() {
    cancel();
    destroyed = true;
  }

  return { schedule, flush, cancel, destroy };
}

export { debounce };
export type { Debounced };
