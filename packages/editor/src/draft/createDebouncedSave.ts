/**
 * createDebouncedSave — pure JS debounce factory for auto-save.
 *
 * Returns schedule/flush/cancel/destroy methods.
 * schedule() resets the timer; flush() fires immediately;
 * cancel() clears the pending timer; destroy() cancels and
 * prevents future scheduling.
 */

interface DebouncedSave {
  schedule: (fn: () => void) => void;
  flush: () => void;
  cancel: () => void;
  destroy: () => void;
}

function createDebouncedSave(delayMs: number): DebouncedSave {
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

export { createDebouncedSave };
export type { DebouncedSave };
