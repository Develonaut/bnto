import { describe, it, expect } from "vitest";
import { toUser } from "./user";

describe("toUser", () => {
  it("maps core fields from raw doc", () => {
    const result = toUser({
      _id: "user_123",
      email: "jane@example.com",
      name: "Jane",
      image: "https://example.com/avatar.png",
      isAnonymous: false,
      plan: "free",
    });

    expect(result).toEqual({
      id: "user_123",
      email: "jane@example.com",
      name: "Jane",
      image: "https://example.com/avatar.png",
      isAnonymous: false,
      plan: "free",
      totalRuns: undefined,
      lastRunAt: undefined,
    });
  });

  it("maps analytics fields (totalRuns, lastRunAt)", () => {
    const result = toUser({
      _id: "user_123",
      plan: "free",
      totalRuns: 42,
      lastRunAt: 1700000000000,
    });

    expect(result.totalRuns).toBe(42);
    expect(result.lastRunAt).toBe(1700000000000);
  });

  it("maps legacy 'starter' plan to 'free'", () => {
    const result = toUser({
      _id: "user_123",
      plan: "starter",
    });

    expect(result.plan).toBe("free");
  });

  it("maps 'pro' plan correctly", () => {
    const result = toUser({
      _id: "user_123",
      plan: "pro",
    });

    expect(result.plan).toBe("pro");
  });

  it("handles null/undefined plan", () => {
    expect(toUser({ _id: "user_123", plan: null }).plan).toBeUndefined();
    expect(toUser({ _id: "user_123" }).plan).toBeUndefined();
  });

  it("converts null fields to undefined", () => {
    const result = toUser({
      _id: "user_123",
      email: null,
      name: null,
      image: null,
      isAnonymous: null,
      totalRuns: null,
      lastRunAt: null,
    });

    expect(result.email).toBeUndefined();
    expect(result.name).toBeUndefined();
    expect(result.image).toBeUndefined();
    expect(result.isAnonymous).toBeUndefined();
    expect(result.totalRuns).toBeUndefined();
    expect(result.lastRunAt).toBeUndefined();
  });

  it("no longer includes runsUsed/runLimit/runsResetAt on User type", () => {
    const result = toUser({
      _id: "user_123",
      plan: "free",
      runsUsed: 7,
      runLimit: 25,
      runsResetAt: 1700000000000,
    });

    // These quota fields are now in ServerQuota, not on User
    expect(result).not.toHaveProperty("runsUsed");
    expect(result).not.toHaveProperty("runLimit");
    expect(result).not.toHaveProperty("runsResetAt");
  });
});
