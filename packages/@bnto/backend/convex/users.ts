import { query } from "./_generated/server";
import { getAppUserId } from "./_helpers/auth";

/** Get the current authenticated user with Bnto fields. */
export const getMe = query({
  args: {},
  handler: async (ctx) => {
    const userId = await getAppUserId(ctx);
    if (userId === null) return null;
    return ctx.db.get(userId);
  },
});
