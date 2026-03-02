# Web App — User Journey Test Matrix

**Domain:** Next.js app on Vercel — pages, navigation, SEO, tool pages, responsive behavior
**Status:** Landing pages live. Tool pages live. E2E tests exist. Execution flow blocked by auth.
**Last Updated:** February 23, 2026

---

## Why This Matters

The web app is the front door. Users arrive from Google, land on a tool page, and either get value (run a bnto) or bounce. Every millisecond of load time, every confusing interaction, every broken layout costs users. These journeys verify the web experience from the user's perspective — what they see, what they can do, and what happens when things go wrong.

---

## Gate Map

Web app rendering and interaction checkpoints.

| Step | What Happens | Gate | Error Behavior |
|------|-------------|------|----------------|
| Route resolution | URL → page component | Next.js routing + `[bnto]` slug validation | Unknown slug → 404 via `notFound()` |
| Static generation | Build-time HTML for SEO pages | `generateStaticParams` + `generateMetadata` | Build failure if registry/metadata broken |
| Auth proxy | Server-side route protection | `proxy.ts` cookie-presence check | Unauth on private route → redirect `/signin` |
| Provider hydration | Client-side React hydration | `ConvexProvider` + auth providers | Hydration mismatch → client-only fallback |
| Data fetching | Convex subscriptions + queries | `useReady()` gate → Convex client | Loading skeleton until ready |
| File interaction | Drag-and-drop, file selection | Browser File API | Graceful fallback for unsupported browsers |

---

## Journey Matrix

### Anonymous Visitor (Public Pages)

The user who arrives from Google. No account, no auth. This is the majority of traffic.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **W1** | Home page loads | Navigate to `/` | Page renders. Bnto gallery visible with all Tier 1 tools. No loading spinner longer than 300ms. |
| **W2** | Tool page loads | Navigate to `/compress-images` | Page renders. H1 matches registry. File drop zone visible. Config controls visible. No layout shift. |
| **W3** | All Tier 1 slugs resolve | Navigate to each of the 6 Tier 1 URLs | All render. None 404. Each has unique H1, title, description. |
| **W4** | Unknown slug 404s | Navigate to `/nonexistent-slug` | 404 page renders (not blank page, not soft 404). HTTP status 404. |
| **W5** | Navigation between pages | Home → tool page → home → different tool page | No full-page reloads. Navigation feels instant. Back button works. |
| **W6** | Dark mode toggle | Click theme toggle | Theme switches. All pages render correctly in both modes. No flash of wrong theme. |

### SEO & Metadata

Verification that search engines see what they need.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **W10** | Title tag correct | View page source for each Tier 1 slug | `<title>` contains bnto name + " -- bnto". Present in HTML source (SSR), not client-rendered. |
| **W11** | Meta description correct | View page source for each Tier 1 slug | `<meta name="description">` present in HTML source. Matches registry entry. |
| **W12** | JSON-LD structured data | View page source for each Tier 1 slug | `<script type="application/ld+json">` present. Contains `WebApplication` type, price `"0"`, correct URL. |
| **W13** | Sitemap includes all slugs | Fetch `/sitemap.xml` | All 6 Tier 1 slugs present. Home page present. Valid XML. |
| **W14** | Canonical URL enforcement | Navigate to `/Compress-Images` (wrong case) | 301 redirect to `/compress-images`. No duplicate content. |
| **W15** | Open Graph tags | View page source | `og:title`, `og:description` present and correct for each slug. |

### Tool Page Interaction

The core product experience — selecting files and configuring a bnto.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **W20** | File drop zone accepts files | Drag file onto drop zone | File appears in selected files list. Size and type displayed. |
| **W21** | File selection via browse | Click browse button → select file | Same as W20. File appears in list. |
| **W22** | Multiple file selection | Select 3+ files | All files listed. Total count shown. |
| **W23** | File removal | Select files → click remove on one | File removed from list. Others remain. |
| **W24** | Config controls render | Load `/compress-images` | Quality slider visible with correct range. Default value set. |
| **W25** | Config controls for each bnto | Load each Tier 1 tool page | Each page shows context-appropriate controls (quality for images, format selector for convert, column mapping for CSV). |
| **W26** | Run button state | No files selected → select files → deselect all | Button disabled when no files. Enabled when files selected. Disabled again when deselected. |

### Protected Routes (Authenticated Users)

Routes that require sign-in.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **W30** | Unauth redirect on protected route | Navigate to `/workflows` without auth | Redirect to `/signin`. Never see `/workflows` content. |
| **W31** | Auth user sees protected route | Sign in → navigate to `/workflows` | Page renders. No redirect. Content visible. |
| **W32** | Auth user on `/signin` redirected | Sign in → navigate to `/signin` | Redirect to `/`. Don't see sign-in form when already authenticated. |
| **W33** | Sign-out redirects | Click sign out | Redirect to `/signin`. Immediate (no loading delay from server cleanup). |

### Error States

What happens when things go wrong.

| # | Journey | Steps | Pass Criteria |
|---|---------|-------|---------------|
| **W40** | Execution failure shows error | Run a bnto → execution fails | Error message visible. Not a crash/white screen. User can retry or go back. |
| **W41** | Network error handled | Lose network during page load | Graceful error state. Not a white screen. Can retry when network returns. |
| **W42** | Large file rejection | Drop a file exceeding size limit | Clear error message mentioning the limit. File not added to list. |

---

## Cross-Domain Dependencies

| Journey | Also Touches | Notes |
|---------|-------------|-------|
| W2, W20-W26 | Auth (anonymous session) | Tool pages trigger `useAnonymousSession()` on mount |
| W24-W25 | Engine (node schemas) | Config controls map to engine node parameters |
| W30-W33 | Auth (proxy, session) | Route protection handled by `proxy.ts` + auth providers |
| W40 | API (execution errors) | Error messages originate from API/engine, displayed by web |

---

## Implementation Notes

- E2E tests via Playwright in `apps/web/e2e/`
- **Page-level layout** verified via `toHaveScreenshot()` in `pages/` and `auth/` specs (`site-navigation.spec.ts`, `auth-lifecycle.spec.ts`)
- **Execution flows** verified programmatically via magic bytes, data attributes, file sizes, and download events in `journeys/browser/` specs
- Shared helpers in `apps/web/e2e/helpers.ts` (`uploadFiles`, `runAndComplete`, `downloadAndVerify`, `navigateToRecipe`, `assertBrowserExecution`)
- SEO tests (W10-W15) verify HTML source, not client-rendered content
- Use `test.use({ reducedMotion: "reduce" })` for deterministic rendering
- Import `{ test, expect }` from `./fixtures` (shared fixture captures console/page errors)
- `data-testid` attributes for reliable state detection in interaction tests
- Test tags: `@browser` (no Convex needed) and `@auth` (needs Convex) for selective runs via `--grep`
