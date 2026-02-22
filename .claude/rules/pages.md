# Page Composition

Pages are the composition layer. Reading a page file should tell you exactly what that page shows -- no digging into child components to understand the structure.

## Principle: Pages Are Readable Blueprints

A page file is a blueprint. When you open it, you should immediately see every section, every piece of content, every interaction point. The page composes small, single-purpose leaf components with shared layout primitives. Nothing is hidden behind opaque wrappers.

```tsx
// GOOD -- every piece visible, reads like a table of contents
<PageLayout>
  <PageLayout.Header>
    <WorkflowTitle workflowId={workflowId} />
    <WorkflowActions workflowId={workflowId} />
  </PageLayout.Header>
  <PageLayout.Content>
    <WorkflowEditor workflowId={workflowId} />
    <ExecutionHistory workflowId={workflowId} />
  </PageLayout.Content>
  <PageLayout.Sidebar>
    <WorkflowMeta workflowId={workflowId} />
    <NodePalette />
  </PageLayout.Sidebar>
</PageLayout>

// BAD -- opaque, have to open WorkflowPage to understand anything
<WorkflowPageContent workflowId={workflowId} />
```

## Rules

### 1. Leaf components, not grouped namespaces

Don't group domain components under a namespace (`WorkflowSidebar.Details`, `WorkflowSidebar.Actions`). Each piece of the UI is its own leaf component -- `WorkflowTitle`, `WorkflowDescription`, `WorkflowMeta`. The page composes them directly.

```tsx
// GOOD -- flat leaf components, page shows the structure
<WorkflowTitle workflowId={workflowId} />
<WorkflowDescription workflowId={workflowId} />
<WorkflowMeta workflowId={workflowId} />

// BAD -- namespace hides what's actually rendered
<WorkflowSidebar.Details workflowId={workflowId} />
```

### 2. Generic shell + domain leaves

Layout structure is generic. Domain content is specific. Separate them cleanly.

**Generic shell components** (`PageLayout.Header`, `PageLayout.Content`, `PageLayout.Sidebar`) handle consistent layout, spacing, and structure across all content types. They live in shared `components/`.

**Domain leaf components** (`WorkflowTitle`, `ExecutionStatus`) handle domain-specific rendering. They live in the route's `_components/` folder.

### 3. Every leaf fetches its own data

Each leaf component is self-fetching. Pass IDs, not data. Multiple leaves fetching the same entity (e.g., the same workflow) will hit the cache -- no redundant network calls.

```tsx
// GOOD -- each leaf fetches what it needs (cached)
<WorkflowTitle workflowId={workflowId} />       // useWorkflow(workflowId) -> renders title
<WorkflowDescription workflowId={workflowId} /> // useWorkflow(workflowId) -> renders description
<WorkflowMeta workflowId={workflowId} />        // useWorkflow(workflowId) -> renders meta

// BAD -- parent fetches, children receive data props
const { workflow } = useWorkflow(workflowId);
<WorkflowTitle title={workflow.title} />
<WorkflowDescription description={workflow.description} />
```

### 4. Compose in the page, not inside wrapper components

The page file itself shows the arrangement of sections. Don't delegate composition to a single child component -- that moves the structure one level deeper and makes the page useless as documentation.

### 5. Prefer page.tsx over layout.tsx for page-specific composition

Use `layout.tsx` only for chrome shared across sibling routes (e.g., a tab bar across sub-pages). If the layout is specific to one page, put it in `page.tsx`. Splitting composition across layout + page makes neither file tell the full story.

```tsx
// When layout.tsx makes sense -- shared chrome across siblings
app/(app)/settings/
  layout.tsx          // Settings nav shared by all settings pages
  profile/page.tsx
  account/page.tsx

// When layout.tsx is wrong -- only one page uses it
app/(app)/workflows/[workflowId]/
  layout.tsx          // Only serves page.tsx
  page.tsx            // Opaque because layout has the shell
```

### 6. Extract repeated UI into small named components

Header elements, back links, navigation controls -- extract into named leaf components so the page reads like a table of contents, not a wall of JSX.

```tsx
// GOOD -- each piece has a name
<PageLayout.Header>
  <BackToDashboardLink />
  <WorkflowNavigation workflowId={workflowId} />
</PageLayout.Header>

// BAD -- raw JSX obscures the structure
<PageLayout.Header>
  <Link href="/dashboard" className="inline-flex items-center gap-1.5 ...">
    <ArrowLeft className="size-4" />
    Back to dashboard
  </Link>
  ...
</PageLayout.Header>
```

---

## SEO Pages

See [seo.md](seo.md) for the full SEO and URL strategy -- slug registry, static generation, metadata, sitemap, middleware integration, and the checklist for shipping new bnto tool pages.
