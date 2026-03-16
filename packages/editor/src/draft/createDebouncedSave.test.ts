import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { createDebouncedSave } from "./createDebouncedSave";

describe("createDebouncedSave", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("fires the callback after the delay", () => {
    const save = createDebouncedSave(2000);
    const fn = vi.fn();

    save.schedule(fn);
    expect(fn).not.toHaveBeenCalled();

    vi.advanceTimersByTime(2000);
    expect(fn).toHaveBeenCalledOnce();
  });

  it("resets the timer on re-schedule", () => {
    const save = createDebouncedSave(2000);
    const fn1 = vi.fn();
    const fn2 = vi.fn();

    save.schedule(fn1);
    vi.advanceTimersByTime(1500);
    save.schedule(fn2);
    vi.advanceTimersByTime(1500);

    expect(fn1).not.toHaveBeenCalled();
    expect(fn2).not.toHaveBeenCalled();

    vi.advanceTimersByTime(500);
    expect(fn2).toHaveBeenCalledOnce();
    expect(fn1).not.toHaveBeenCalled();
  });

  it("flush fires immediately and clears pending", () => {
    const save = createDebouncedSave(2000);
    const fn = vi.fn();

    save.schedule(fn);
    save.flush();

    expect(fn).toHaveBeenCalledOnce();

    // Timer fires but nothing happens (already flushed)
    vi.advanceTimersByTime(2000);
    expect(fn).toHaveBeenCalledOnce();
  });

  it("flush does nothing when no pending save", () => {
    const save = createDebouncedSave(2000);
    // Should not throw
    save.flush();
  });

  it("cancel prevents the callback from firing", () => {
    const save = createDebouncedSave(2000);
    const fn = vi.fn();

    save.schedule(fn);
    save.cancel();

    vi.advanceTimersByTime(3000);
    expect(fn).not.toHaveBeenCalled();
  });

  it("destroy prevents future scheduling", () => {
    const save = createDebouncedSave(2000);
    const fn = vi.fn();

    save.destroy();
    save.schedule(fn);

    vi.advanceTimersByTime(3000);
    expect(fn).not.toHaveBeenCalled();
  });
});
