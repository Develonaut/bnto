import { describe, expect, it } from "vitest";
import { ConvexError } from "convex/values";
import { enforceQuota } from "./_helpers/quota";
import { FREE_RUN_LIMIT, FREE_PLAN_RUN_LIMIT } from "./_test_helpers";

describe("enforceQuota", () => {
  describe("anonymous users", () => {
    it("allows anonymous user under their run limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: true, runsUsed: 2, runLimit: FREE_RUN_LIMIT }),
      ).not.toThrow();
    });

    it("throws ANONYMOUS_QUOTA_EXCEEDED at the run limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: true, runsUsed: FREE_RUN_LIMIT, runLimit: FREE_RUN_LIMIT }),
      ).toThrow(ConvexError);

      try {
        enforceQuota({ isAnonymous: true, runsUsed: FREE_RUN_LIMIT, runLimit: FREE_RUN_LIMIT });
      } catch (e) {
        expect(e).toBeInstanceOf(ConvexError);
        expect((e as ConvexError<{ code: string }>).data.code).toBe(
          "ANONYMOUS_QUOTA_EXCEEDED",
        );
      }
    });

    it("throws ANONYMOUS_QUOTA_EXCEEDED above the run limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: true, runsUsed: FREE_RUN_LIMIT + 2, runLimit: FREE_RUN_LIMIT }),
      ).toThrow(ConvexError);
    });
  });

  describe("real (non-anonymous) users", () => {
    it("allows user under their plan limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 10, runLimit: FREE_PLAN_RUN_LIMIT }),
      ).not.toThrow();
    });

    it("allows user with zero runs used", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 0, runLimit: FREE_PLAN_RUN_LIMIT }),
      ).not.toThrow();
    });

    it("throws RUN_LIMIT_REACHED at the plan limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: FREE_PLAN_RUN_LIMIT, runLimit: FREE_PLAN_RUN_LIMIT }),
      ).toThrow(ConvexError);

      try {
        enforceQuota({ isAnonymous: false, runsUsed: FREE_PLAN_RUN_LIMIT, runLimit: FREE_PLAN_RUN_LIMIT });
      } catch (e) {
        expect(e).toBeInstanceOf(ConvexError);
        expect((e as ConvexError<{ code: string }>).data.code).toBe(
          "RUN_LIMIT_REACHED",
        );
      }
    });

    it("throws RUN_LIMIT_REACHED above the plan limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: 30, runLimit: FREE_PLAN_RUN_LIMIT }),
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
      // User upgraded from anonymous. isAnonymous is now false,
      // runLimit bumped to FREE_PLAN_RUN_LIMIT by auth.ts upgrade path.
      // They had 3 runs as anonymous, now their plan limit governs.
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: FREE_RUN_LIMIT, runLimit: FREE_PLAN_RUN_LIMIT }),
      ).not.toThrow();
    });
  });

  describe("edge cases", () => {
    it("handles isAnonymous: undefined (legacy user)", () => {
      // isAnonymous is optional in schema — older users might not have it.
      // Should behave like a real user (RUN_LIMIT_REACHED, not ANONYMOUS_QUOTA_EXCEEDED).
      expect(() =>
        enforceQuota({ isAnonymous: undefined, runsUsed: 10, runLimit: FREE_PLAN_RUN_LIMIT }),
      ).not.toThrow();
    });

    it("handles runsUsed exactly one below the limit", () => {
      expect(() =>
        enforceQuota({ isAnonymous: false, runsUsed: FREE_PLAN_RUN_LIMIT - 1, runLimit: FREE_PLAN_RUN_LIMIT }),
      ).not.toThrow();
    });
  });
});
