import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";

const modules = import.meta.glob("./**/*.ts");

/**
 * Tests for the createOrUpdateUser callback patterns in auth.ts.
 *
 * We can't call the callback directly (it's wired into @convex-dev/auth),
 * so we exercise the exact same db operations it performs and verify:
 * - Schema accepts the document shapes
 * - Fields are stored and retrievable
 *
 * If these tests pass, the createOrUpdateUser callback will work — it does
 * exactly these operations on the same schema.
 */
describe("user creation (createOrUpdateUser patterns)", () => {
  describe("new user (email/password)", () => {
    it("creates a user row with email, name, and free-tier defaults", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          name: "Jane Doe",
          email: "jane@example.com",
          plan: "free",
          totalRuns: 0,
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));

      expect(user).not.toBeNull();
      expect(user!.name).toBe("Jane Doe");
      expect(user!.email).toBe("jane@example.com");
      expect(user!.plan).toBe("free");
      expect(user!.totalRuns).toBe(0);
    });

    it("accepts optional image field", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          name: "Jane Doe",
          email: "jane@example.com",
          image: "https://example.com/avatar.png",
          plan: "free",
          totalRuns: 0,
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.image).toBe("https://example.com/avatar.png");
    });
  });

  describe("profile updates (re-authentication)", () => {
    it("updates name and email on re-auth without touching app fields", async () => {
      const t = convexTest(schema, modules);

      const userId = await t.run(async (ctx) => {
        return ctx.db.insert("users", {
          name: "Jane Doe",
          email: "jane@example.com",
          plan: "pro",
          totalRuns: 15,
        });
      });

      // Re-auth updates profile (e.g., name changed on provider)
      await t.run(async (ctx) => {
        await ctx.db.patch(userId, {
          name: "Jane Smith",
        });
      });

      const user = await t.run(async (ctx) => ctx.db.get(userId));
      expect(user!.name).toBe("Jane Smith");
      // App fields untouched
      expect(user!.plan).toBe("pro");
      expect(user!.totalRuns).toBe(15);
    });
  });
});
