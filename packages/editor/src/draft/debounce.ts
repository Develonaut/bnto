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

/** Mutable timer state shared between debounce methods. */
interface TimerState {
  timerId: ReturnType<typeof setTimeout> | null;
  pending: (() => void) | null;
  destroyed: boolean;
}

function cancelTimer(state: TimerState) {
  if (state.timerId !== null) {
    clearTimeout(state.timerId);
    state.timerId = null;
  }
  state.pending = null;
}

function flushTimer(state: TimerState) {
  if (state.pending) {
    const fn = state.pending;
    cancelTimer(state);
    fn();
  }
}

function scheduleTimer(state: TimerState, fn: () => void, delayMs: number) {
  if (state.destroyed) return;
  cancelTimer(state);
  state.pending = fn;
  state.timerId = setTimeout(() => {
    const toRun = state.pending;
    state.timerId = null;
    state.pending = null;
    toRun?.();
  }, delayMs);
}

function debounce(delayMs: number): Debounced {
  const state: TimerState = { timerId: null, pending: null, destroyed: false };

  return {
    schedule: (fn) => scheduleTimer(state, fn, delayMs),
    flush: () => flushTimer(state),
    cancel: () => cancelTimer(state),
    destroy: () => {
      cancelTimer(state);
      state.destroyed = true;
    },
  };
}

export { debounce };
export type { Debounced };
