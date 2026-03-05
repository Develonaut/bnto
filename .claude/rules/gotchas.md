# Common Gotchas

Known pitfalls and their fixes. Check here before debugging something that feels familiar.

## `convexQuery` + Empty IDs = Silent Subscription Failures

`@convex-dev/react-query`'s `convexQuery()` creates a real Convex WebSocket subscription unless args is the literal string `"skip"`. Passing `{ id: "" }` creates an active subscription that hits the Convex validator and fails with `ArgumentValidationError`. React Query's `enabled: false` does NOT prevent this — it only prevents React Query from surfacing the data, but the subscription still fires server-side.

**Symptoms:** 100% failure rate spikes on `executions:get` (or any query) in the Convex dashboard. Intermittent 500 errors from Next.js dev server under parallel E2E load. Cache hit rate drops to 0%.

**Fix:** Always use `"skip"` in adapter functions when the ID is falsy:

```typescript
export function getExecutionQuery(id: string) {
  return convexQuery(api.executions.get, id ? { id: id as Id<"executions"> } : "skip");
}
```

See [convex.md](convex.md#convexquery-skip-guard-critical) for the full rule.

## SSG + Convex Hooks

Pages using Convex hooks (`useMutation`, `useQuery`) crash during static generation because there's no `ConvexProvider` at build time.

**Fix:** Push `"use client"` to the smallest leaf component that needs Convex hooks. The page stays a server component and renders static content (headings, descriptions) on the server. Client leaves handle their own loading state.

```tsx
// page.tsx -- server component, renders heading on the server
import { MyForm } from "./_components/MyForm";
import { Heading } from "@/components/ui/Heading";

export default function Page() {
  return (
    <>
      <Heading level={1}>Page Title</Heading> {/* server-rendered */}
      <MyForm /> {/* client leaf */}
    </>
  );
}

// _components/MyForm.tsx -- client boundary at the leaf
("use client");
export function MyForm() {
  const { data, isLoading } = core.recipes.useRecipes();
  if (isLoading) return <Skeleton />;
  return <form>...</form>;
}
```

**Anti-pattern:** `"use client"` on a page + `dynamic(() => ..., { ssr: false })` for Convex components. This puts the SSR bypass at the trunk level — the page renders nothing on the server, and the dynamic import adds an extra loading waterfall. `ssr: false` belongs at the leaf, not the trunk.

**When `ssr: false` IS correct:** Leaf components that require browser-only APIs with no SSR fallback — ReactFlow (`@xyflow/react`), Canvas, WebGL. These genuinely cannot render on the server. Use `dynamic` + `ssr: false` in the parent component that imports the leaf, keeping the bypass as close to the browser-only code as possible.

## Convex Module Filenames -- No Hyphens

Convex doesn't allow hyphens in module filenames. Path components can only contain alphanumeric characters, underscores, or periods. A file like `enrich-with-data.ts` will fail on deploy with `InvalidConfig`.

**Fix:** Use underscores instead of hyphens for all files inside the `convex/` directory.

```
# BAD -- deploy will fail
convex/_helpers/enrich-with-data.ts

# GOOD
convex/_helpers/enrich_with_data.ts
```

## Convex Schema Migration (Production)

Convex validates all existing documents against the new schema on deploy. If you rename or change a field type, the deploy will fail.

**Fix:**

1. Temporarily relax the schema (make old and new fields optional)
2. Deploy: `npx convex deploy --yes`
3. Run a cleanup mutation: `npx convex run --prod <function>`
4. Restore the strict schema
5. Deploy again

## Tailwind v4 + Monorepo

Tailwind skips `node_modules` by default. When shared packages are extracted (e.g., future `@bnto/ui`), classes defined in those packages won't generate CSS.

**Fix:** Add a `@source` directive in the app's `globals.css`:

```css
@source "../../node_modules/@bnto/ui";
```

Path is relative to the CSS file. Currently UI is co-located in `apps/web/` so this isn't needed yet.

## pnpm 10 Native Dependencies

pnpm 10 requires explicit opt-in for native dependency builds.

**Fix:** Add to root `package.json`:

```json
{
  "pnpm": {
    "onlyBuiltDependencies": ["esbuild", "sharp", "unrs-resolver"]
  }
}
```

## Turbopack + Node.js Subpath Imports on Vercel

Turbopack on Vercel does NOT support Node.js subpath imports (`#components/*` via `package.json` `imports` field). Builds fail with `Module not found: Can't resolve '#components/...'`. Setting `turbopack.root` to the monorepo root makes it worse — breaks the `imports` field resolution entirely. `resolveAlias` workarounds also fail.

**Fix:** Use standard `@/` path aliases via `tsconfig.json` `paths` instead of subpath imports. Do NOT set `turbopack.root` in `next.config.ts` — `transpilePackages` handles monorepo resolution.

## Git Case-Sensitivity on macOS (Vercel Deploys)

macOS filesystems are case-insensitive by default. Renaming `container.tsx` to `Container.tsx` on macOS won't be detected by git — it keeps tracking the old lowercase name. On Vercel (Linux, case-sensitive), imports for `@/components/ui/Container` fail because git checked out `container.tsx`.

**Fix:** Use a two-step `git mv` to force git to recognize the case change:

```bash
git mv components/ui/container.tsx components/ui/container_tmp.tsx
git mv components/ui/container_tmp.tsx components/ui/Container.tsx
```

**Detection:** Run this to find case mismatches:

```bash
git ls-files apps/web/components/ | while read gitpath; do
  dir=$(dirname "$gitpath"); base=$(basename "$gitpath")
  actual=$(ls -1 "$dir" 2>/dev/null | grep -ix "$base")
  [ -n "$actual" ] && [ "$actual" != "$base" ] && echo "MISMATCH: git=$gitpath disk=$dir/$actual"
done
```

## Playwright Screenshot Updates

`--update-snapshots` won't overwrite existing screenshots that already match. If you need to force full regeneration, delete the `__screenshots__/` directory first.

## Stale Symlinks After Moving Packages

After renaming or moving packages in the monorepo, old `node_modules` symlinks can persist and cause confusing import errors.

**Fix:** Delete `node_modules` in the affected packages and run `pnpm install`.

## No Object.assign Compound Components (RSC Incompatible)

`Object.assign` dot-notation (`<Dialog.Content>`, `<Card.Header>`) does NOT work in React Server Components. Server components get "client reference" objects — sub-properties assigned via `Object.assign` are undefined at render time.

**Fix (already applied repo-wide):** Use flat named exports with prefixed names instead:

```tsx
// GOOD -- flat exports, RSC compatible
import { Dialog, DialogContent, DialogTitle } from "@bnto/ui";

<Dialog open={open}>
  <DialogContent>
    <DialogTitle>Title</DialogTitle>
  </DialogContent>
</Dialog>

// BAD -- Object.assign dot-notation, breaks in RSC
<Dialog.Content>
  <Dialog.Title>Title</Dialog.Title>
</Dialog.Content>
```

**If you see `Object.assign` compound patterns anywhere in the codebase, convert them to flat exports.**
