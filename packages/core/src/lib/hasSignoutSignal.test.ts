import { describe, it, expect, beforeEach, afterEach, vi } from "vitest";
import { hasSignoutSignal } from "./hasSignoutSignal";

describe("hasSignoutSignal", () => {
  let cookieValue = "";

  beforeEach(() => {
    cookieValue = "";
    // Mock document.cookie in Node.js environment
    vi.stubGlobal("document", {
      get cookie() {
        return cookieValue;
      },
    });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it("returns false when no cookies are set", () => {
    expect(hasSignoutSignal()).toBe(false);
  });

  it("returns true when bnto-signout cookie is present", () => {
    cookieValue = "bnto-signout=1";
    expect(hasSignoutSignal()).toBe(true);
  });

  it("returns true when bnto-signout is among other cookies", () => {
    cookieValue = "theme=dark; bnto-signout=1; lang=en";
    expect(hasSignoutSignal()).toBe(true);
  });

  it("returns false when cookie name is only a partial match", () => {
    cookieValue = "not-bnto-signout=1";
    expect(hasSignoutSignal()).toBe(false);
  });

  it("returns false during SSR (no document)", () => {
    vi.stubGlobal("document", undefined);
    expect(hasSignoutSignal()).toBe(false);
  });
});
