import { describe, it, expect } from "vitest";
import { toUser } from "./user";

describe("toUser", () => {
  it("maps core fields from raw doc", () => {
    const result = toUser({
      _id: "user_123",
      email: "jane@example.com",
      name: "Jane",
      image: "https://example.com/avatar.png",
      plan: "free",
      totalRuns: 0,
    });

    expect(result).toEqual({
      id: "user_123",
      email: "jane@example.com",
      name: "Jane",
      image: "https://example.com/avatar.png",
      plan: "free",
      totalRuns: 0,
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
      totalRuns: 0,
    });

    expect(result.plan).toBe("free");
  });

  it("maps 'pro' plan correctly", () => {
    const result = toUser({
      _id: "user_123",
      plan: "pro",
      totalRuns: 0,
    });

    expect(result.plan).toBe("pro");
  });

  it("handles null/undefined plan", () => {
    expect(toUser({ _id: "user_123", plan: null, totalRuns: 0 }).plan).toBeUndefined();
    expect(toUser({ _id: "user_123", totalRuns: 0 }).plan).toBeUndefined();
  });

  it("converts null fields to undefined", () => {
    const result = toUser({
      _id: "user_123",
      email: null,
      name: null,
      image: null,
      totalRuns: 0,
      lastRunAt: null,
    });

    expect(result.email).toBeUndefined();
    expect(result.name).toBeUndefined();
    expect(result.image).toBeUndefined();
    expect(result.totalRuns).toBe(0);
    expect(result.lastRunAt).toBeUndefined();
  });

});
