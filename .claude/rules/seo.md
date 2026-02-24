# SEO & URL Strategy

Every predefined Bnto has its own dedicated, indexable URL. The URL IS the app -- not a landing page. Users land ready to drop files with zero navigation required.

---

## Routing: Root-Level Slugs

Bnto slugs live at the root: `/compress-images`, `/clean-csv`, `/rename-files`. Shorter URLs rank better and feel like native pages of the site.

**Route:** `app/[bnto]/page.tsx`

**Trade-off:** Root-level dynamic segments match every unknown path. This is manageable with a strict slug registry (see below) but requires discipline. Every new top-level app route (e.g., `/pricing`, `/about`, `/blog`) must be added as a static route that takes priority over the dynamic `[bnto]` segment. Next.js resolves static routes before dynamic ones, so collisions are prevented by convention, not configuration.

```
app/
|-- page.tsx                  # / -- home (static, wins over [bnto])
|-- [bnto]/
|   +-- page.tsx              # /compress-images, /clean-csv, etc.
|-- (app)/
|   |-- workflows/page.tsx    # /workflows (static, wins over [bnto])
|   +-- settings/page.tsx     # /settings (static, wins over [bnto])
|-- (auth)/
|   +-- signin/page.tsx       # /signin (static, wins over [bnto])
+-- pricing/page.tsx          # /pricing (static, wins over [bnto])
```

**Rule:** If you add a new top-level route, verify it doesn't collide with an existing bnto slug. Check `lib/bntoRegistry.ts`.

---

## Slug Registry

The slug registry is the single source of truth for all predefined Bnto URLs. It lives in code (not just docs) so it can drive `generateStaticParams`, `generateMetadata`, middleware, and sitemap generation from one place.

**File:** `apps/web/lib/bntoRegistry.ts`

```typescript
export interface BntoEntry {
  slug: string;
  title: string;           // Full page title: "Compress Images Online Free -- bnto"
  description: string;     // Meta description (one sentence, plain language)
  h1: string;              // Exact target search query from Notion
  fixture: string;         // Fixture filename in engine/examples/
}

export const BNTO_REGISTRY: BntoEntry[] = [
  {
    slug: "compress-images",
    title: "Compress Images Online Free -- bnto",
    description: "Compress PNG, JPEG, and WebP images instantly in your browser. No upload limits, no signup.",
    h1: "Compress Images Online Free",
    fixture: "compress-images.bnto.json",
  },
  // ... all registered bntos
] as const satisfies readonly BntoEntry[];

const slugSet = new Set(BNTO_REGISTRY.map((b) => b.slug));

/** Returns true if the slug maps to a registered Bnto. */
export function isValidBntoSlug(slug: string): boolean {
  return slugSet.has(slug);
}

/** Returns the registry entry for a slug, or undefined if not found. */
export function getBntoBySlug(slug: string) {
  return BNTO_REGISTRY.find((b) => b.slug === slug);
}
```

This is the contract between the docs (strategy/bntos.md) and the app. When you add a bnto to `bntos.md`, you add it here too. When the registry grows, it can be split into a separate data file imported by the module.

---

## Static Generation

SEO pages must be statically generated at build time. Static HTML loads faster than server-rendered pages, and Google rewards speed.

```typescript
// app/[bnto]/page.tsx

import { notFound } from "next/navigation";
import { BNTO_REGISTRY, getBntoBySlug } from "@/lib/bntoRegistry";

// Pre-render all registered slugs at build time
export function generateStaticParams() {
  return BNTO_REGISTRY.map((bnto) => ({ bnto: bnto.slug }));
}

// Per-slug metadata -- resolved at build time for static pages
export function generateMetadata({ params }: { params: { bnto: string } }) {
  const entry = getBntoBySlug(params.bnto);
  if (!entry) return {};
  return {
    title: entry.title,
    description: entry.description,
    openGraph: {
      title: entry.title,
      description: entry.description,
    },
  };
}

export default function BntoPage({ params }: { params: { bnto: string } }) {
  const entry = getBntoBySlug(params.bnto);
  if (!entry) notFound();        // Unknown slug -> 404, not a soft 404
  return <BntoApp entry={entry} />;
}
```

**Key:** `notFound()` returns a real 404 status code. Without this, unknown slugs render an empty page with a 200 -- Google treats that as a soft 404 and may penalize the site.

---

## 404 Handling

Every unknown slug must return a proper 404. No soft 404s.

- `app/[bnto]/page.tsx` calls `notFound()` when the slug isn't in the registry
- `app/not-found.tsx` renders a branded 404 page with navigation back to home
- Unknown paths like `/asdfghj` hit the dynamic segment, fail the registry lookup, and 404 cleanly

---

## Middleware Integration

The current middleware (`middleware.ts`) is private-by-default: any path not in `PUBLIC_PATHS` redirects unauthenticated users to `/signin`. Bnto slugs are public by design -- unauthenticated users must be able to access them.

**Solution:** Import the slug registry into the middleware and treat registered bnto slugs as public paths.

```typescript
// middleware.ts -- updated isPublicPath check
import { isValidBntoSlug } from "./lib/bntoRegistry";

// In the middleware function:
const isBntoSlug = isValidBntoSlug(pathname.slice(1)); // strip leading /
if (!isAuthenticated && !isPublicPath(pathname) && !isBntoSlug) {
  return NextResponse.redirect(new URL("/signin", request.url));
}
```

**Why not add all slugs to `PUBLIC_PATHS`?** `PUBLIC_PATHS` is a small, static list used by the auth system. Mixing in 20+ bnto slugs pollutes its purpose. Keep the two concerns separate: `PUBLIC_PATHS` for app routes, `isValidBntoSlug` for bnto tool pages.

---

## Canonical URLs & Normalization

Slugs are lowercase-hyphen only. Any non-canonical variant must redirect, not render.

| Request | Action |
|---|---|
| `/compress-images` | Render (canonical) |
| `/Compress-Images` | 301 redirect to `/compress-images` |
| `/compress_images` | 301 redirect to `/compress-images` |
| `/COMPRESS-IMAGES` | 301 redirect to `/compress-images` |
| `/compress-images/` | 301 redirect to `/compress-images` (no trailing slash) |

**Implementation:** Handle in middleware before the auth check. Normalize the pathname and redirect if it doesn't match.

```typescript
// middleware.ts -- canonical slug enforcement
const normalized = pathname.toLowerCase().replace(/_/g, "-").replace(/\/$/, "") || "/";
if (pathname !== normalized) {
  const url = request.nextUrl.clone();
  url.pathname = normalized;
  return NextResponse.redirect(url, 301);
}
```

**Trailing slashes:** Next.js config has `trailingSlash: false` by default. Enforce this -- don't leave it ambiguous.

---

## Metadata Strategy

### Default format

```
title:       "[Action] Online Free -- bnto"
description: One sentence. Plain language. Includes the action and "free".
og:title:    Same as title
og:description: Same as description
h1:          Exact target search query from Notion
```

### When the default format doesn't fit

The `"[Action] Online Free -- bnto"` template works for Tier 1 tool pages. As the catalog grows, some bntos won't fit this mold. The `BntoEntry` in the registry stores full strings, not template fragments -- so each entry can have whatever title makes sense. The format is a guideline for consistency, not a hard constraint.

```typescript
// Follows the template
{ title: "Compress Images Online Free -- bnto", ... }

// Diverges when the template is awkward
{ title: "Convert CSV to JSON -- bnto", ... }
{ title: "Fetch API to CSV -- bnto", ... }
```

**Rule:** Every title must end with ` -- bnto` for brand consistency. Everything before that is flexible.

### h1 and search query alignment

The `h1` on the page must match the target search query exactly. This is sourced from the Notion `Bnto Directory & Launch Plan` page. The h1 may differ from the page title -- titles are for browser tabs and SERP snippets, h1 is for on-page SEO.

---

## Sitemap

Generate a sitemap from the slug registry. This is how Google discovers bnto pages.

**File:** `apps/web/app/sitemap.ts`

```typescript
import { BNTO_REGISTRY } from "@/lib/bntoRegistry";
import type { MetadataRoute } from "next";

const BASE_URL = "https://bnto.dev"; // or whatever the production domain is

export default function sitemap(): MetadataRoute.Sitemap {
  const bntoPages = BNTO_REGISTRY.map((entry) => ({
    url: `${BASE_URL}/${entry.slug}`,
    lastModified: new Date(),
    changeFrequency: "monthly" as const,
    priority: 0.8,
  }));

  return [
    { url: BASE_URL, lastModified: new Date(), changeFrequency: "weekly", priority: 1.0 },
    ...bntoPages,
  ];
}
```

**Priority:** Home page at 1.0, bnto tool pages at 0.8, other pages (about, pricing) at 0.5.

---

## Structured Data (JSON-LD)

Tool pages benefit from structured data. Google can display rich results for software applications and web tools.

```typescript
// app/[bnto]/page.tsx -- add JSON-LD to the page
function BntoJsonLd({ entry }: { entry: BntoEntry }) {
  const jsonLd = {
    "@context": "https://schema.org",
    "@type": "WebApplication",
    name: entry.h1,
    description: entry.description,
    url: `https://bnto.dev/${entry.slug}`,
    applicationCategory: "UtilityApplication",
    operatingSystem: "Any",
    offers: {
      "@type": "Offer",
      price: "0",
      priceCurrency: "USD",
    },
  };
  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
    />
  );
}
```

Keep this simple. Don't over-specify. `WebApplication` with a free price signal is the main win.

---

## LLM & AI Search Discovery

LLM-powered search (ChatGPT, Perplexity, Google AI Overviews, Gemini) is a major discovery channel. When someone asks "best free tool to compress images" or "how to batch rename files online", the answer comes from an LLM reading and synthesizing web content. This has different requirements than traditional SEO.

### What LLMs need to recommend you

LLMs don't read meta tags the way Google's crawler does. They read **page content** and evaluate whether it answers a user's query. To get recommended, bnto pages need to clearly and concisely communicate:

1. **What the tool does** -- in plain language, not marketing speak
2. **What it accepts** -- file types, input formats, limits
3. **What it outputs** -- what the user gets back
4. **What it costs** -- free, no signup, no upload limits
5. **How it compares** -- what makes it different (browser-based, no server upload, privacy)

### Page content structure for LLM readability

Every bnto tool page should include a short, factual description section that reads naturally to both humans and LLMs. This isn't a marketing block -- it's a clear, scannable summary.

```tsx
// Below the h1, above the tool UI
<section>
  <p>
    Compress PNG, JPEG, and WebP images directly in your browser.
    No files are uploaded to a server -- processing happens locally.
    Free, no signup required, no file size limits.
  </p>
  <dl>
    <dt>Supported formats</dt>
    <dd>PNG, JPEG, WebP, GIF</dd>
    <dt>Max files</dt>
    <dd>Unlimited (processed one at a time)</dd>
    <dt>Privacy</dt>
    <dd>Files never leave your browser</dd>
  </dl>
</section>
```

**Key:** Use semantic HTML (`<dl>`, `<dt>`, `<dd>`) for structured facts. LLMs and search engines extract structured content more reliably than paragraphs.

### llms.txt

`llms.txt` is an emerging convention (similar to `robots.txt`) that tells AI systems what your site is and what it offers. Place it at the site root.

**File:** `apps/web/public/llms.txt`

```
# bnto

> bnto is a free online tool for common file tasks -- compress images, clean CSVs, rename files, convert formats. All processing happens in the browser. No signup, no upload limits, no server processing.

## Tools

- [Compress Images](https://bnto.dev/compress-images): Compress PNG, JPEG, and WebP images in your browser. Free, no signup.
- [Resize Images](https://bnto.dev/resize-images): Resize images to exact dimensions or percentages. Free, no signup.
- [Clean CSV](https://bnto.dev/clean-csv): Remove empty rows, trim whitespace, deduplicate. Free, no signup.
- [Rename Files](https://bnto.dev/rename-files): Batch rename files with patterns. Free, no signup.
...
```

Update this file whenever a new bnto ships. Keep descriptions factual and short -- one sentence per tool.

**Also provide `llms-full.txt`** for LLMs that want deeper context. This can include supported formats, limitations, and comparison points.

### robots.txt: Don't block AI crawlers

AI search engines use their own crawlers. If these are blocked, your pages are invisible to LLM-powered search.

**File:** `apps/web/public/robots.txt`

```
User-agent: *
Allow: /

# Explicitly allow AI crawlers
User-agent: GPTBot
Allow: /

User-agent: ChatGPT-User
Allow: /

User-agent: anthropic-ai
Allow: /

User-agent: Google-Extended
Allow: /

User-agent: PerplexityBot
Allow: /

User-agent: ClaudeBot
Allow: /

Sitemap: https://bnto.dev/sitemap.xml
```

**Rule:** Never block AI crawlers unless there's a specific legal or business reason. Being discoverable by LLMs is a competitive advantage.

### Structured data extensions for AI

The JSON-LD section above covers the basics. For LLM discoverability, add `featureList` and `screenshot` properties to help AI systems understand capabilities:

```typescript
const jsonLd = {
  "@context": "https://schema.org",
  "@type": "WebApplication",
  name: entry.h1,
  description: entry.description,
  url: `https://bnto.dev/${entry.slug}`,
  applicationCategory: "UtilityApplication",
  operatingSystem: "Any",
  browserRequirements: "Requires a modern web browser",
  offers: {
    "@type": "Offer",
    price: "0",
    priceCurrency: "USD",
  },
  featureList: entry.features,  // ["PNG", "JPEG", "WebP", "No upload", "Browser-based"]
};
```

### Registry extension for LLM content

Add a `features` field to the `BntoEntry` type so each tool page can generate both the description list and the structured data features from a single source:

```typescript
export interface BntoEntry {
  slug: string;
  title: string;
  description: string;
  h1: string;
  fixture: string;
  features: string[];       // ["PNG", "JPEG", "WebP", "No upload", "Browser-based"]
  inputFormats?: string[];   // ["image/png", "image/jpeg", "image/webp"]
  outputFormats?: string[];  // ["image/png", "image/jpeg", "image/webp"]
}
```

### How LLMs decide what to recommend

When a user asks "best free tool to compress images", LLMs evaluate:

1. **Does the page clearly say it does this?** -- Not buried in marketing copy, stated plainly
2. **Is it free?** -- Explicit "free" and "$0" signals, not "freemium" ambiguity
3. **Does it work?** -- Structured data says it's a working web app, not a blog post about tools
4. **Is it trustworthy?** -- HTTPS, clear privacy policy, no dark patterns
5. **Is the content fresh?** -- Last modified dates, active sitemap

Bnto's advantage: browser-based processing (privacy), no signup (friction-free), free (no paywall). Every tool page should state all three clearly.

### Checklist addition for LLM discoverability

When shipping a new bnto, also verify:

- [ ] Page has a plain-language description section with what/accepts/outputs/costs
- [ ] `featureList` populated in JSON-LD
- [ ] Entry added to `llms.txt`
- [ ] Description uses natural query language ("compress images", not "image optimization pipeline")

---

## Slug Naming Conventions

- **Lowercase, hyphen-separated:** `/compress-images` not `/CompressImages`
- **Verb-first:** `/compress-images`, `/rename-files`, `/clean-csv`
- **Action-oriented:** `/compress-images` not `/image-compressor`
- **No internal bnto names:** `/compress-images` not `/run-image-compress`
- **No reserved words:** Don't use slugs that match current or likely future app routes (`settings`, `workflows`, `pricing`, `about`, `blog`, `docs`, `api`)

### Reserved paths (never use as bnto slugs)

These are current or anticipated app routes. The slug registry should validate against this list.

```typescript
const RESERVED_PATHS = [
  "signin", "signout", "signup", "waitlist",
  "workflows", "executions", "settings",
  "pricing", "about", "blog", "docs", "changelog",
  "api", "admin", "dashboard",
] as const;
```

---

## Future: User-Created Public Workflows

Today, only predefined bntos have public URLs. Eventually users may want shareable links for their own workflows. When that time comes:

- **User workflows live under a distinct namespace:** `/w/[id]` or `/u/[username]/[workflow]`
- **Never at root level.** Root-level slugs are reserved for predefined, curated bnto tools
- **This is a future concern.** Don't build the namespace now, but don't make decisions that block it

---

## Checklist: Shipping a New Bnto

### Engine & Fixture
1. Create or verify the fixture in `engine/examples/`
2. Verify fixture executes clean via `bnto run` with sample input

### Registry & Docs
3. Add to Notion (`Bnto Directory & Launch Plan`) with search volume and rationale
4. Add to `strategy/bntos.md` with slug, persona, and fixture status
5. Add to `lib/bntoRegistry.ts` with full metadata (title, description, h1, fixture, features)
6. Verify slug doesn't collide with reserved paths or existing app routes

### SEO & Metadata
7. Verify `h1` matches the exact target query from Notion
8. Page has a plain-language description section (what/accepts/outputs/costs)
9. JSON-LD `featureList` populated
10. Run `pnpm build` to confirm the static page generates without errors

### LLM Discoverability
11. Add entry to `public/llms.txt`
12. Description uses natural query language, not internal jargon

### Runtime
13. Verify execution increments the run counter in Convex

---

## Related Documents

- [strategy/bntos.md](../strategy/bntos.md) -- Bnto directory with slugs, tiers, and fixture status
- [rules/pages.md](pages.md) -- Page composition patterns
- [rules/auth-routing.md](auth-routing.md) -- Middleware and auth flow
