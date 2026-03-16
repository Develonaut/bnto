import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { debounce } from "./debounce";

describe("debounce", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("fires the callback after the delay", () => {
    const d = debounce(2000);
    const fn = vi.fn();

    d.schedule(fn);
    expect(fn).not.toHaveBeenCalled();

    vi.advanceTimersByTime(2000);
    expect(fn).toHaveBeenCalledOnce();
  });

  it("resets the timer on re-schedule", () => {
    const d = debounce(2000);
    const fn1 = vi.fn();
    const fn2 = vi.fn();

    d.schedule(fn1);
    vi.advanceTimersByTime(1500);
    d.schedule(fn2);
    vi.advanceTimersByTime(1500);

    expect(fn1).not.toHaveBeenCalled();
    expect(fn2).not.toHaveBeenCalled();

    vi.advanceTimersByTime(500);
    expect(fn2).toHaveBeenCalledOnce();
    expect(fn1).not.toHaveBeenCalled();
  });

  it("flush fires immediately and clears pending", () => {
    const d = debounce(2000);
    const fn = vi.fn();

    d.schedule(fn);
    d.flush();

    expect(fn).toHaveBeenCalledOnce();

    vi.advanceTimersByTime(2000);
    expect(fn).toHaveBeenCalledOnce();
  });

  it("flush does nothing when no pending callback", () => {
    const d = debounce(2000);
    d.flush();
  });

  it("cancel prevents the callback from firing", () => {
    const d = debounce(2000);
    const fn = vi.fn();

    d.schedule(fn);
    d.cancel();

    vi.advanceTimersByTime(3000);
    expect(fn).not.toHaveBeenCalled();
  });

  it("destroy prevents future scheduling", () => {
    const d = debounce(2000);
    const fn = vi.fn();

    d.destroy();
    d.schedule(fn);

    vi.advanceTimersByTime(3000);
    expect(fn).not.toHaveBeenCalled();
  });
});
