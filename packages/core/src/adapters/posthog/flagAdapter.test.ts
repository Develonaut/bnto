import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

// Simulate a browser window for adapter functions that check typeof window
const fakeWindow: Record<string, unknown> = {};
beforeEach(() => {
  (globalThis as unknown as Record<string, unknown>).window = fakeWindow;
});
afterEach(() => {
  for (const key of Object.keys(fakeWindow)) delete fakeWindow[key];
  delete (globalThis as unknown as Record<string, unknown>).window;
});

function mockPostHog() {
  return {
    isFeatureEnabled: vi.fn(),
    getFeatureFlag: vi.fn(),
    getFeatureFlagPayload: vi.fn(),
    onFeatureFlags: vi.fn().mockReturnValue(() => {}),
    reloadFeatureFlags: vi.fn(),
  };
}

const PH_KEY = "__bnto_ph__";

describe("flagAdapter", () => {
  let isFlagEnabled: typeof import("./flagAdapter").isFlagEnabled;
  let getFlagVariant: typeof import("./flagAdapter").getFlagVariant;
  let getFlagResult: typeof import("./flagAdapter").getFlagResult;
  let subscribeToFlags: typeof import("./flagAdapter").subscribeToFlags;
  let reloadFlags: typeof import("./flagAdapter").reloadFlags;

  beforeEach(async () => {
    const mod = await import("./flagAdapter");
    isFlagEnabled = mod.isFlagEnabled;
    getFlagVariant = mod.getFlagVariant;
    getFlagResult = mod.getFlagResult;
    subscribeToFlags = mod.subscribeToFlags;
    reloadFlags = mod.reloadFlags;
  });

  describe("isFlagEnabled", () => {
    it("returns undefined when PostHog is not initialized", () => {
      expect(isFlagEnabled("my-flag")).toBeUndefined();
    });

    it("delegates to posthog.isFeatureEnabled", () => {
      const ph = mockPostHog();
      ph.isFeatureEnabled.mockReturnValue(true);
      fakeWindow[PH_KEY] = ph;

      expect(isFlagEnabled("my-flag")).toBe(true);
      expect(ph.isFeatureEnabled).toHaveBeenCalledWith("my-flag");
    });

    it("pushes evaluation to E2E test hook when present", () => {
      const ph = mockPostHog();
      ph.isFeatureEnabled.mockReturnValue(false);
      fakeWindow[PH_KEY] = ph;

      const testEvents: unknown[] = [];
      fakeWindow.__bnto_flags__ = testEvents;

      isFlagEnabled("pro-save");
      expect(testEvents).toEqual([{ key: "pro-save", value: false }]);
    });
  });

  describe("getFlagVariant", () => {
    it("returns undefined when PostHog is not initialized", () => {
      expect(getFlagVariant("cta-test")).toBeUndefined();
    });

    it("returns variant string from posthog", () => {
      const ph = mockPostHog();
      ph.getFeatureFlag.mockReturnValue("variant-b");
      fakeWindow[PH_KEY] = ph;

      expect(getFlagVariant("cta-test")).toBe("variant-b");
    });
  });

  describe("getFlagResult", () => {
    it("returns undefined when PostHog is not initialized", () => {
      expect(getFlagResult("my-flag")).toBeUndefined();
    });

    it("returns full result with payload", () => {
      const ph = mockPostHog();
      ph.getFeatureFlag.mockReturnValue("test-a");
      ph.getFeatureFlagPayload.mockReturnValue({ cta: "Try Pro" });
      ph.isFeatureEnabled.mockReturnValue(true);
      fakeWindow[PH_KEY] = ph;

      expect(getFlagResult("cta-test")).toEqual({
        key: "cta-test",
        enabled: true,
        variant: "test-a",
        payload: { cta: "Try Pro" },
      });
    });
  });

  describe("subscribeToFlags", () => {
    it("returns no-op unsubscribe when PostHog is not initialized", () => {
      const unsub = subscribeToFlags(() => {});
      expect(typeof unsub).toBe("function");
      unsub(); // should not throw
    });

    it("delegates to posthog.onFeatureFlags", () => {
      const ph = mockPostHog();
      const unsubFn = vi.fn();
      ph.onFeatureFlags.mockReturnValue(unsubFn);
      fakeWindow[PH_KEY] = ph;

      const callback = vi.fn();
      const unsub = subscribeToFlags(callback);
      expect(ph.onFeatureFlags).toHaveBeenCalledWith(callback);
      expect(unsub).toBe(unsubFn);
    });
  });

  describe("reloadFlags", () => {
    it("does not throw when PostHog is not initialized", () => {
      expect(() => reloadFlags()).not.toThrow();
    });

    it("delegates to posthog.reloadFeatureFlags", () => {
      const ph = mockPostHog();
      fakeWindow[PH_KEY] = ph;

      reloadFlags();
      expect(ph.reloadFeatureFlags).toHaveBeenCalled();
    });
  });
});
