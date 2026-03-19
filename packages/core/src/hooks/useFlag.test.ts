import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";

// Simulate a browser window for adapter functions
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

/**
 * useFlag delegates to isFlagEnabled + subscribeToFlags from the adapter.
 * The adapter is tested thoroughly in flagAdapter.test.ts.
 * Here we verify the adapter's core read + subscribe behavior
 * that useFlag depends on.
 */
describe("useFlag adapter dependencies", () => {
  it("isFlagEnabled returns undefined when PostHog is not initialized", async () => {
    const { isFlagEnabled } = await import("../adapters/posthog/flagAdapter");
    expect(isFlagEnabled("test-flag")).toBeUndefined();
  });

  it("isFlagEnabled returns boolean when PostHog is present", async () => {
    const ph = mockPostHog();
    ph.isFeatureEnabled.mockReturnValue(true);
    fakeWindow[PH_KEY] = ph;

    const { isFlagEnabled } = await import("../adapters/posthog/flagAdapter");
    expect(isFlagEnabled("test-flag")).toBe(true);
    expect(ph.isFeatureEnabled).toHaveBeenCalledWith("test-flag");
  });

  it("subscribeToFlags returns cleanup function", async () => {
    const ph = mockPostHog();
    const cleanupFn = vi.fn();
    ph.onFeatureFlags.mockReturnValue(cleanupFn);
    fakeWindow[PH_KEY] = ph;

    const { subscribeToFlags } = await import("../adapters/posthog/flagAdapter");
    const unsub = subscribeToFlags(() => {});
    expect(unsub).toBe(cleanupFn);
  });
});
