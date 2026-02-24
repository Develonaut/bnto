import { describe, expect, it } from "vitest";
import { ConvexError } from "convex/values";
import { enforceQuota } from "./_helpers/quota";

describe("enforceQuota", () => {
  describe("anonymous users", () => {
    it("allows anonymous user under the default limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: true, runsUsed: 2, runLimit: 25 }),
      ).not.toThrow();
    });

    it("throws ANONYMOUS_QUOTA_EXCEEDED at the default limit (3)", () => {
      expect(() =>
        enforceQuota({ isAnonymous: true, runsUsed: 3, runLimit: 25 }),
      ).toThrow(ConvexError);

      try {
        enforceQuota({ isAnonymous: true, runsUsed: 3, runLimit: 25 });
      } catch (e) {
        expect(e).toBeInstanceOf(ConvexError);
        expect((e as ConvexError<{ code: string }>).data.code).toBe(
          "ANONYMOUS_QUOTA_EXCEEDED",
        );
      }
    });

    it("throws ANONYMOUS_QUOTA_EXCEEDED above the default limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: true, runsUsed: 5, runLimit: 25 }),
      ).toThrow(ConvexError);
    });

    it("anonymous limit takes priority over plan limit when lower", () => {
      // Anonymous limit is 3, plan limit is 25. At 3 runs, anonymous
      // limit fires first even though plan has headroom.
      try {
        enforceQuota({ isAnonymous: true, runsUsed: 3, runLimit: 25 });
      } catch (e) {
        expect((e as ConvexError<{ code: string }>).data.code).toBe(
          "ANONYMOUS_QUOTA_EXCEEDED",
        );
      }
    });
  });

  describe("real (non-anonymous) users", () => {
    it("allows user under their plan limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 10, runLimit: 25 }),
      ).not.toThrow();
    });

    it("allows user with zero runs used", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 0, runLimit: 25 }),
      ).not.toThrow();
    });

    it("throws RUN_LIMIT_REACHED at the plan limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 25, runLimit: 25 }),
      ).toThrow(ConvexError);

      try {
        enforceQuota({ isAnonymous: false, runsUsed: 25, runLimit: 25 });
      } catch (e) {
        expect(e).toBeInstanceOf(ConvexError);
        expect((e as ConvexError<{ code: string }>).data.code).toBe(
          "RUN_LIMIT_REACHED",
        );
      }
    });

    it("throws RUN_LIMIT_REACHED above the plan limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 30, runLimit: 25 }),
      ).toThrow(ConvexError);
    });

    it("respects different plan limits", () => {
      // Free: 5 runs
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 5, runLimit: 5 }),
      ).toThrow(ConvexError);

      // Pro: 100 runs — same usage is fine
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 5, runLimit: 100 }),
      ).not.toThrow();
    });
  });

  describe("upgraded users (was anonymous, now real)", () => {
    it("uses plan limit, not anonymous limit, after upgrade", () => {
      // User upgraded from anonymous. isAnonymous is now false.
      // They had 3 runs as anonymous, now their plan limit governs.
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 3, runLimit: 25 }),
      ).not.toThrow();
    });
  });

  describe("edge cases", () => {
    it("handles isAnonymous: undefined (legacy user)", () => {
      // isAnonymous is optional in schema — older users might not have it.
      // Should behave like a real user (no anonymous limit).
      expect(() =>
        enforceQuota({ isAnonymous: undefined, runsUsed: 10, runLimit: 25 }),
      ).not.toThrow();
    });

    it("handles runsUsed exactly one below the limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 24, runLimit: 25 }),
      ).not.toThrow();
    });
  });
});
