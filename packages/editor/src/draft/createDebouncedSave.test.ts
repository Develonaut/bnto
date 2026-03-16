import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { createDebouncedSave } from "./createDebouncedSave";

describe("createDebouncedSave", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("delegates to debounce — schedule fires after delay", () => {
    const save = createDebouncedSave(2000);
    const fn = vi.fn();

    save.schedule(fn);
    expect(fn).not.toHaveBeenCalled();

    vi.advanceTimersByTime(2000);
    expect(fn).toHaveBeenCalledOnce();
  });

  it("flush fires immediately", () => {
    const save = createDebouncedSave(2000);
    const fn = vi.fn();

    save.schedule(fn);
    save.flush();
    expect(fn).toHaveBeenCalledOnce();
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
