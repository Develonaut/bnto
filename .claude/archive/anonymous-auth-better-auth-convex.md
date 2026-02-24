# Better Auth + Convex anonymous sessions: a complete technical assessment

**Date:** February 2026
**Status:** Research complete — informs anonymous-first access model decision

---

Better Auth ships a first-party anonymous authentication plugin that creates real database users, supports cookie-based sessions, and provides an `onLinkAccount` callback for migrating data when users upgrade to full accounts. The Convex adapter officially lists anonymous auth as supported, but the integration is in early alpha with at least one open bug. For our use case — anonymous workflow execution with quota enforcement and seamless account upgrade — this approach works but demands careful implementation.

---

## The anonymous plugin creates real users with a clear upgrade path

Better Auth's `anonymous` plugin (imported from `better-auth/plugins`) is a built-in, first-party feature. When a visitor calls `authClient.signIn.anonymous()`, the system creates a genuine user record in the database with `isAnonymous: true` and a generated temporary email in the format `temp-{id}@example.com`. A session cookie is set immediately, making the anonymous user indistinguishable from an authenticated user in terms of session management.

The upgrade flow requires no special "link" method. When an anonymous user calls any standard sign-in or sign-up method (email/password, OAuth, magic link), Better Auth automatically triggers the `onLinkAccount` callback with both the old anonymous user and the new authenticated user. **A critical architectural detail: the anonymous user receives a new user ID upon upgrade** — the old anonymous record is deleted by default, and a fresh user record is created for the authenticated identity. This differs from Firebase, which preserves the UID during upgrade.

Configuration is straightforward on both server and client:

```typescript
// Server
anonymous({
  onLinkAccount: async ({ anonymousUser, newUser }) => {
    // Migrate data: old ID → new ID
    await db.exec(sql`UPDATE workflow_runs SET "userId" = ${newUser.userId} WHERE "userId" = ${anonymousUser.userId}`);
  },
  disableDeleteAnonymousUser: true, // Recommended for Convex
})

// Client
const user = await authClient.signIn.anonymous();
// Later, to upgrade:
await authClient.signIn.email({ email: "user@example.com", password: "..." });
```

Additional plugin options include `generateRandomEmail` for custom email formats, `generateName` for display names, and `emailDomainName` to set the domain on generated emails.

---

## Convex support is official but carries alpha-stage caveats

The official integration package is **`@convex-dev/better-auth`** (published by the Convex team). It operates as a Convex Component that manages four core tables (`user`, `session`, `account`, `verification`) and translates Better Auth's database operations into Convex queries and mutations. The anonymous plugin is explicitly listed among the officially supported plugins that work without schema changes in the default installation mode.

However, three significant bugs have been documented in this integration:

- **Issue #3658** (fixed): The anonymous plugin's `after` hook incorrectly rejected valid first-time anonymous sign-ins by detecting the just-created session cookie as a pre-existing anonymous session, returning a 400 error.
- **Issue #5824** (fixed): The plugin's cleanup logic ran during the sign-in request itself, deleting the newly created anonymous user milliseconds after creation. The workaround — setting `disableDeleteAnonymousUser: true` — remains recommended.
- **Issue #180** (still open): An `ArgumentValidationError` occurs during anonymous sign-in, likely because the default component schema doesn't include the `isAnonymous` boolean field the plugin writes to the user table.

The docs explicitly state the component is in **"early alpha development."** Better Auth must be pinned to version **1.4.9** with `--save-exact` for Convex compatibility. For maximum reliability with anonymous auth, the **Local Install** approach (where the component lives in `convex/betterAuth/` with a developer-controlled schema) is recommended over the default NPM component install, since it ensures the generated schema includes all plugin-required fields.

---

## Session persistence uses standard cookies with a 7-day default

Better Auth manages anonymous sessions identically to authenticated sessions. The session token is stored in an **httpOnly, secure cookie** named `{prefix}.session_token` (default prefix: `better-auth`). Sessions expire after **7 days** by default, with the expiry refreshed whenever the `updateAge` threshold is reached (default: 1 day of activity). These values are configurable:

```typescript
session: {
  expiresIn: 60 * 60 * 24 * 7, // 7 days in seconds
  updateAge: 60 * 60 * 24,      // refresh every 1 day
}
```

**The anonymous user record persists in the database indefinitely**, even after the session cookie expires. However, once a session expires and the cookie is gone, the user cannot recover that specific anonymous identity — they'd need to sign in anonymously again, which creates an entirely new user record. This means data associated with the expired anonymous user becomes orphaned unless you implement a cleanup strategy.

For the Convex integration specifically, sessions follow a **two-step authentication model**: Better Auth creates a session record in Convex's database, then the Convex plugin generates a JWT (default 15-minute expiry) stored as a cookie. Convex validates WebSocket connections using this JWT via JWKS verification, with the session record serving as the authoritative source of truth. Static JWKS is recommended over dynamic to avoid 100–400ms latency per request on page load.

---

## The recommended data-merging pattern centers on `onLinkAccount`

For our scenario — anonymous user runs workflows, then creates an account — the implementation pattern has four steps:

**Step 1: Associate all data with the anonymous user ID.** Every workflow run, uploaded file, and quota counter references `userId` as a foreign key, indexed for efficient querying. Anonymous users are real database users, so this works identically to authenticated user data.

**Step 2: Enforce quotas against the anonymous user.** In Convex mutation handlers, query the run count for the current user and check against anonymous-tier limits:

```typescript
const user = await ctx.db.get(userId);
if (user.isAnonymous) {
  const runs = await ctx.db.query("workflowRuns")
    .withIndex("by_user", q => q.eq("userId", userId)).collect();
  if (runs.length >= ANONYMOUS_RUN_LIMIT) throw new Error("Sign up for more runs!");
}
```

**Step 3: Migrate data in `onLinkAccount`.** When the user upgrades, batch-update all foreign keys from the old anonymous user ID to the new authenticated user ID. This callback fires before the anonymous user is deleted, giving the migration window.

**Step 4: Clean up.** Implement scheduled Convex functions to periodically delete orphaned anonymous user records older than 30 days and their associated R2 output files.

---

## Alternative: Convex Auth's native Anonymous provider

Convex's own auth library (`@convex-dev/auth`) offers a potentially cleaner approach via its `Anonymous` provider combined with the `createOrUpdateUser` callback. This can keep the **same user ID** when upgrading, eliminating the need for data migration entirely — the anonymous user is simply patched with the new email and name, and `isAnonymous` is set to `false`.

This is architecturally superior if seamless ID preservation matters more than Better Auth's broader plugin ecosystem. Given we're already committed to Better Auth for the full auth stack, this would be a significant departure — but worth flagging as the migration-free alternative.

---

## Fingerprinting as a secondary abuse-detection signal

Browser fingerprinting (FingerprintJS or Fingerprint Pro) and anonymous auth sessions solve fundamentally different problems. For quota enforcement and run tracking with seamless account upgrade, anonymous sessions are decisively better as the primary system:

- **Deterministic identity**: Every user gets a unique database record. Open-source FingerprintJS achieves only 40–60% accuracy — identical corporate laptops produce identical fingerprints.
- **Native data association**: Run history and quotas are linked via foreign keys. No separate user management layer needed.
- **Cost**: Better Auth's anonymous plugin is free. Fingerprint Pro starts at $99/month.
- **Legal**: GDPR classifies fingerprints as personal data requiring explicit consent.

Where fingerprinting adds genuine value is as a **secondary abuse-detection layer** — store a fingerprint hash alongside the anonymous user record. When someone clears cookies to reset their quota, check the fingerprint against existing users and carry forward the quota if matched. The open-source FingerprintJS library (free) is sufficient for this heuristic purpose.

---

## Key implementation decisions for Bnto

| Decision | Recommendation | Reason |
|----------|---------------|--------|
| Anonymous plugin source | Better Auth `anonymous` plugin | First-party, maintained |
| Convex install mode | Local Install (`convex/betterAuth/`) | Avoid schema mismatch bug #180 |
| `disableDeleteAnonymousUser` | `true` | Prevents premature deletion race condition |
| Better Auth version | Pin to `1.4.9` with `--save-exact` | Required for Convex compatibility |
| Data association | Foreign key on `userId` | Same pattern as authenticated users |
| Upgrade migration | `onLinkAccount` batch update | Atomic migration window |
| Abuse prevention | Secondary fingerprint hash | Free, heuristic, non-blocking |
| Orphan cleanup | Convex cron (30-day TTL) | Prevents database bloat |
| CAPTCHA on anonymous sign-in | Cloudflare Turnstile | Prevents database-bloat attacks |

---

## References

- Better Auth anonymous plugin docs: https://www.better-auth.com/docs/plugins/anonymous
- Convex + Better Auth supported plugins: https://labs.convex.dev/better-auth/supported-plugins
- Convex + Better Auth local install: https://labs.convex.dev/better-auth/features/local-install
- Convex Auth anonymous users: https://labs.convex.dev/auth/config/anonymous
- GitHub issue #3658 (fixed): anonymous plugin after hook bug
- GitHub issue #5824 (fixed): anonymous user deleted on creation
- GitHub issue #180 (open): ArgumentValidationError on anonymous sign-in
- Convex Stack blog: Anonymous Users via Sessions — https://stack.convex.dev/anonymous-users-via-sessions
- Convex Discord: Upgrading anonymous accounts to real accounts — https://discord-questions.convex.dev/m/1366162915035447416
