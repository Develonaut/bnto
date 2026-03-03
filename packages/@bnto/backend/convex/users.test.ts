import { describe, expect, it } from "vitest";
import { convexTest } from "convex-test";
import schema from "./schema";

const modules = import.meta.glob("./**/*.ts");

describe("getMe (schema shape)", () => {
  it("returns user with all expected fields", async () => {
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

    expect(user).toMatchObject({
      name: "Jane Doe",
      email: "jane@example.com",
      plan: "free",
    });
    expect(user!._id).toBe(userId);
    expect(user!._creationTime).toBeDefined();
  });
});
