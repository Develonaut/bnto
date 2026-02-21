# Skeleton Standards

Skeletons are the first thing users see while data loads. A well-matched skeleton prevents layout shift and creates the perception of instant loading. A mismatched skeleton is worse than no skeleton at all.

## Core Principle: Match the Loaded Layout

The skeleton must mirror the structure, dimensions, and position of the real content it replaces. When data arrives, the transition should feel like content "painting in" -- not elements jumping around.

```
User navigates to page
  -> Show skeleton (data is loading)
  -> Data arrives:
       -> Has content? -> Replace skeleton with content (in-place)
       -> No content?  -> Replace skeleton with empty state
```

## Shape Rules

| Content type | Skeleton shape | Sizing |
|---|---|---|
| **Headings** | Single rectangle | Match heading font-size height, 40-60% container width |
| **Body text** | 2-3 rectangles, last one shorter | Match line-height, 60-80% width, last line 40-60% |
| **Avatars** | Circle | Exact avatar dimensions |
| **Images/thumbnails** | Rectangle with `aspect-ratio` | Match expected aspect ratio and width |
| **Cards** | Card outline with interior skeleton elements | Match card outer dimensions |
| **Badges/tags** | Small rounded rectangles | Approximate badge dimensions |

**Design for the median case.** If a description is typically 2-3 lines, show 2-3 skeleton lines. Don't try to match exact content length -- skeletons communicate structure, not content.

**Err toward more skeleton than less.** A slight collapse (skeleton larger than content) is less jarring than a push-down (skeleton smaller than content).

## Layout Shift Prevention

### Reserve space with CSS

```tsx
// Images -- use aspect-ratio
<Skeleton className="aspect-video w-full" />

// Text containers -- use min-height for variable content
<div className="min-h-[3.5rem]">  {/* ~2 lines minimum */}
  {isLoading ? <Skeleton className="h-4 w-3/4" /> : <p>{description}</p>}
</div>

// Avatars -- exact dimensions
<Skeleton className="size-10 rounded-full" />
```

### Same container for both states

The skeleton and real content must occupy the same parent element. Never swap between two different containers -- replace content in-place.

```tsx
// GOOD -- same container, content swaps in-place
export function WorkflowTitle({ workflowId }: { workflowId: string }) {
  const { data, isLoading } = core.workflows.useWorkflowById(workflowId);
  if (isLoading || !data) return <Skeleton className="h-7 w-48" />;
  return <Heading level={1}>{data.name}</Heading>;
}

// BAD -- different wrappers for skeleton vs loaded
if (isLoading) return <div className="skeleton-wrapper"><Skeleton /></div>;
return <div className="loaded-wrapper"><Content /></div>;
```

## What to Skeleton (and What Not To)

### Do skeleton

- Dynamic content that depends on data fetching (workflow names, execution statuses, node outputs)
- Lists and grids (show 3-6 skeleton cards to fill the viewport)
- Metadata and stats

### Do NOT skeleton

- **Static text that's known at render time** -- page titles like "Workflows", "Settings", section headers. Render these as real text immediately.
- **Action components** -- buttons, inputs, toggles. Show them disabled or hidden until ready, never as skeleton shapes.
- **Content that loads under 1 second** -- a skeleton flash is worse than no loading indicator. Consider a 200-300ms delay before showing the skeleton if loads are typically fast.
- **Modals, toasts, dropdowns, popovers** -- these appear on user action and should load instantly or not appear until ready.

## Animation

Use `motion-safe:animate-pulse` on the `Skeleton` primitive. This ensures:

1. Shimmer animation runs for sighted users with no motion preference
2. Animation is disabled under `prefers-reduced-motion: reduce` (static gray boxes)
3. Playwright tests with `reducedMotion: "reduce"` capture deterministic frames

```tsx
// packages/ui/src/primitives/skeleton.tsx
className={cn("motion-safe:animate-pulse rounded-md bg-muted", className)}
```

## Variable-Length Content Strategy

The hardest problem: content could be 1 line or 10 lines.

| Strategy | When to use |
|---|---|
| **Median skeleton + min-height** | Descriptions, summaries -- show 2-3 skeleton lines, set `min-height` on container |
| **Fixed skeleton + grow** | Workflow configs -- show 3-4 representative lines, container grows naturally |
| **Aspect-ratio** | Images, previews -- known ratio, use `aspect-ratio` CSS |
| **Count-based** | Lists, grids -- show enough skeleton items to fill the viewport (3-6 cards) |

**The min-height pattern:**

```tsx
// Container has min-height for ~2 lines, grows for longer content
<div className="min-h-[3rem]">
  {isLoading ? (
    <Stack gap="2">
      <Skeleton className="h-4 w-full" />
      <Skeleton className="h-4 w-3/5" />
    </Stack>
  ) : (
    <p>{description}</p>
  )}
</div>
```

## Empty States vs Skeletons

Skeletons are for **loading**. Empty states are for **no data**. These are different states with different visual treatments.

- Transition from skeleton -> empty state should use the **same container dimensions** to prevent shift
- If you know at navigation time that content is empty (e.g., server-rendered), skip the skeleton and render the empty state directly

## Testing: Paired Screenshot Strategy

Every skeleton should have a paired visual regression test comparing the skeleton state to the loaded state.

**Pattern:**
1. Intercept data (WebSocket blocking)
2. Screenshot the skeleton state
3. Release data
4. Screenshot the loaded state
5. Compare the pair visually -- skeleton shapes should correspond to loaded content positions

**Test data must show real content, not empty states.** The loaded screenshot should have actual content so the skeleton shapes can be compared against real layout.
