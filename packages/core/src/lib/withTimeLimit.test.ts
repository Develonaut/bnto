import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { withTimeLimit } from "./withTimeLimit";

/** A promise the test settles by hand, standing in for a running step. */
function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: Error) => void;
  const promise = new Promise<T>((res, rej) => {
    resolve = res;
    reject = rej;
  });
  return { promise, resolve, reject };
}

describe("withTimeLimit", () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("fails with the engine's message and calls onExpire once the limit passes", async () => {
    const onExpire = vi.fn();
    const work = deferred<string>();

    const limited = withTimeLimit(work.promise, 2, onExpire);
    const assertion = expect(limited).rejects.toThrow("Run timed out after 2s");
    await vi.advanceTimersByTimeAsync(2000);

    await assertion;
    expect(onExpire).toHaveBeenCalledOnce();
  });

  it("does not expire before the limit", async () => {
    const onExpire = vi.fn();
    withTimeLimit(deferred<string>().promise, 2, onExpire);

    await vi.advanceTimersByTimeAsync(1999);

    expect(onExpire).not.toHaveBeenCalled();
  });

  it("resolves with the work's result and clears its timer when the work finishes in time", async () => {
    const onExpire = vi.fn();
    const work = deferred<string>();

    const limited = withTimeLimit(work.promise, 2, onExpire);
    work.resolve("done");

    await expect(limited).resolves.toBe("done");
    expect(vi.getTimerCount()).toBe(0);
    await vi.advanceTimersByTimeAsync(5000);
    expect(onExpire).not.toHaveBeenCalled();
  });

  it("passes the work's own error through and clears its timer", async () => {
    const work = deferred<string>();

    const limited = withTimeLimit(work.promise, 2, vi.fn());
    work.reject(new Error("Corrupt JPEG"));

    await expect(limited).rejects.toThrow("Corrupt JPEG");
    expect(vi.getTimerCount()).toBe(0);
  });

  it("arms no timer when there is no limit", async () => {
    const onExpire = vi.fn();
    withTimeLimit(deferred<string>().promise, undefined, onExpire);

    expect(vi.getTimerCount()).toBe(0);
    await vi.advanceTimersByTimeAsync(60_000);
    expect(onExpire).not.toHaveBeenCalled();
  });
});
