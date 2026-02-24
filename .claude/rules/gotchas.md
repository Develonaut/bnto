# Common Gotchas

Known pitfalls and their fixes. Check here before debugging something that feels familiar.

## SSG + Convex Hooks

Pages using Convex hooks (`useMutation`, `useQuery`) crash during static generation because there's no `ConvexProvider` at build time.

**Fix:** Extract the Convex-dependent code into a separate client component and load it via `next/dynamic` with `ssr: false`.

```tsx
// page.tsx -- must be "use client" to use ssr: false
"use client";
import dynamic from "next/dynamic";

const MyForm = dynamic(
  () => import("./my-form").then((m) => m.MyForm),
  { ssr: false },
);

export default function Page() {
  return <MyForm />;
}
```

Note: `ssr: false` requires `"use client"` on the importing file in Next.js 16.

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

## Object.assign + Shared Radix Primitives (Dot-Notation)

When two component wrappers use `Object.assign` on the **same Radix primitive** (e.g., Dialog and Sheet both use `DialogPrimitive.Root`), the last one to load overwrites the first's sub-components. `Dialog.Content` silently becomes `SheetContent`.

This happens because `Object.assign` **mutates its first argument**. If two wrappers target the same function reference, they stomp each other.

**Fix:** Create a wrapper function for each root so they have distinct object identities:

```tsx
// components/Dialog.tsx
import { Dialog as PrimitiveDialog, DialogContent, ... } from "../primitives/dialog";

// Distinct function -- Object.assign targets THIS, not the shared Radix primitive
function DialogRoot(props: ComponentProps<typeof PrimitiveDialog>) {
  return <PrimitiveDialog {...props} />;
}

export const Dialog = Object.assign(DialogRoot, {
  Root: DialogRoot,
  Content: DialogContent,
  ...
});
```

**Symptoms:** Component renders with wrong styling (e.g., Dialog slides in from the right like a Sheet). Check `data-slot` attributes in DevTools -- they'll reveal which primitive is actually rendering.

