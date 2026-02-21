# Performance Standards

**Performance is a competitive advantage. Every millisecond matters.**

## Server Components First

Next.js App Router defaults to Server Components -- respect that default. Only add `"use client"` when the component genuinely needs browser APIs, state, or event handlers.

| Rule | Do | Don't |
|---|---|---|
| **Push client boundaries down** | `"use client"` on the smallest leaf that needs interactivity | `"use client"` on a page or layout because one child needs `useState` |
| **Server data fetching** | Fetch in Server Components, pass as props | Fetch everything client-side when server fetch is possible |
| **Compose server + client** | `<ServerLayout><ClientWidget /></ServerLayout>` | Mark the whole tree client because one piece needs a click handler |

```tsx
// GOOD -- Server Component page, client island for interactivity
export default function WorkflowPage({ params }: Props) {
  return (
    <WorkflowLayout>           {/* Server Component */}
      <WorkflowHeader />        {/* Server Component */}
      <RunWorkflowButton />     {/* "use client" -- leaf component */}
    </WorkflowLayout>
  );
}

// BAD -- entire page is client because RunWorkflowButton needs onClick
"use client";
export default function WorkflowPage({ params }: Props) { ... }
```

## Bundle Size

- **No barrel re-exports in client code** -- importing from `index.ts` in `"use client"` components can pull in the entire package. Import specific files directly
- **Lazy load heavy components** -- `next/dynamic` for modals, dialogs, below-fold content
- **Tree-shake aggressively** -- named exports, no side effects in module scope

## Core Web Vitals Targets

| Metric | Target | How |
|---|---|---|
| **LCP** | < 2.5s | Server Components, streaming, `next/image`, font preloading |
| **FID/INP** | < 100ms | Minimal client JS, no heavy computation in event handlers |
| **CLS** | < 0.1 | Explicit image dimensions, font `display: swap`, no layout shifts |

## PR Checklist

- [ ] Minimal `"use client"` -- only on leaf components that need interactivity
- [ ] No unnecessary client-side data fetching
- [ ] Images use `next/image`
- [ ] Heavy components lazy loaded (`next/dynamic`)
- [ ] No barrel imports in client components
