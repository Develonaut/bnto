# Component Standards

## Start Inline, Extract When Earned

Write components naturally. Data fetching, state, handlers, derived values -- put it all in the component. When the file gets too big or the logic gets hard to follow, extract pieces. Not before.

**There is no mandatory hook for every component.** A hook is a tool for managing complexity, not a ritual. Use one when it helps.

### When to extract a hook

- The component is getting long (>80-100 lines of logic + JSX)
- You want to reuse the same logic in multiple components
- The logic is complex enough that it's hard to follow inline
- You want to test the logic independently from rendering

### When to keep it inline

- The component is straightforward -- fetch, check loading, render
- State and handlers are simple and obvious
- Extracting a hook would just move code to another file with no clarity gain

```tsx
// This is fine. No hook needed.
"use client";

export function WorkflowTitle({ workflowId }: { workflowId: string }) {
  const { data: workflow, isLoading } = core.workflows.useWorkflowById(workflowId);
  if (isLoading || !workflow) return <Skeleton className="h-7 w-48" />;
  return (
    <Row className="gap-2">
      <Heading level={1}>{workflow.name}</Heading>
      {workflow.status === "draft" && <Badge variant="secondary">Draft</Badge>}
    </Row>
  );
}
```

```tsx
// This has earned a hook -- complex state, multiple concerns, 100+ lines.
"use client";

export function WorkflowEditor({ workflowId, onSuccess }: WorkflowEditorProps) {
  const { slots } = useWorkflowEditor({ workflowId, onSuccess });
  // ...
}
```

---

## Self-Fetching Components

Components fetch their own data. Pass IDs, not data. Multiple components fetching the same entity hit the cache -- no redundant network calls.

```tsx
// Page composes self-fetching leaves
<Sidebar.Content>
  <WorkflowTitle workflowId={workflowId} />
  <WorkflowDescription workflowId={workflowId} />
  <WorkflowMeta workflowId={workflowId} />
</Sidebar.Content>
```

### Loading states

Co-locate the skeleton in the same file. Keep it simple.

```tsx
export function WorkflowTitle({ workflowId }: { workflowId: string }) {
  const { data: workflow, isLoading } = core.workflows.useWorkflowById(workflowId);
  if (isLoading || !workflow) return <Skeleton className="h-7 w-48" />;
  return <Heading level={1}>{workflow.name}</Heading>;
}
```

If the skeleton is complex (10+ lines, multiple elements), extract it to a named function in the same file. Only extract to a separate file if it's shared.

---

## When to Graduate to Separate Files

Start with a single file. Break out when complexity **earns** it -- not preemptively.

| Signal | Action |
|---|---|
| File exceeds ~250 lines | Time to split |
| Logic is complex and you want to test it independently | Extract to a hook |
| Multiple components need the same logic | Extract to a shared hook |
| Skeleton is complex and shared | Extract to its own file |

When a component graduates to a folder:

```
Feature/
|-- FeatureRoot.tsx              # Main component (Root suffix internally)
|-- FeatureSkeleton.tsx          # Loading state (if complex)
|-- useFeature.ts                # Hook (if earned)
|-- context.ts                   # React context (if needed)
|-- utils/
|   +-- computeValue.ts          # Pure functions
+-- index.ts                     # Re-export with Object.assign (strips Root)
```

**The default is a single file. Folders are for when you've outgrown it.**

---

## Hook Conventions (When You Do Extract)

When a component earns a hook, follow these conventions:

**Naming:** Hook names match the component with a `use` prefix. No "Props" suffix.
- `WorkflowCard` -> `useWorkflowCard`
- `ExecutionForm` -> `useExecutionForm`

**Return shape:** Return what the component needs. No mandatory structure. For simple hooks, return values directly. For complex hooks with multiple elements, a slots object is fine.

```tsx
// Simple -- return values directly
function useWorkflowCard(workflow: Workflow) {
  const { canEdit } = useWorkflowPermissions(workflow);
  const handleEdit = () => openEditDialog(workflow._id);
  return { canEdit, handleEdit };
}

// Complex multi-element -- slots are useful here
function useWorkflowEditor({ workflowId, onSuccess }: WorkflowEditorProps) {
  // lots of state, handlers, validation...
  return {
    slots: {
      form: { props: { onSubmit: handleSubmit } },
      nameInput: { props: { value: name, onChange: setName } },
      submitButton: { props: { disabled: !isValid } },
    },
  };
}
```

**Hook decomposition:** If a hook grows past ~50 lines or does multiple unrelated things, break it into smaller hooks. But don't preemptively decompose a 20-line hook into three 7-line hooks.

---

## Layout & Typography: `cn` + `createCn`

Layout and typography components use `cn()` for static class merging and `createCn()` for variant-to-class mapping.

**Use `createCn()` for variant resolution.** It wraps `tailwind-variants`' `tv()` so we can swap the underlying library without touching consumers.

```tsx
// Variants via createCn()
const containerCn = createCn({
  base: "mx-auto w-full",
  variants: { size: containerSizeMap, padX: { true: "px-6" } },
  defaultVariants: { padX: true },
});

const Container = forwardRef<HTMLDivElement, ContainerProps>(
  ({ size, padX, className, ...rest }, ref) => (
    <div ref={ref} className={containerCn({ size, padX }, className)} {...rest} />
  ),
);
```

**Import pattern:** `createCn` from `../create-cn`, `cn` from `../cn`.

**Never** import `tv` directly from `tailwind-variants` -- use `createCn`.
**Never** use inline `cn()` for variant resolution -- use `createCn()` instead.

---

## Compound Composition & Dot-Notation

**ALL multi-part components use dot-notation namespacing -- no exceptions.** This includes shadcn/ui primitives. Flat named exports (`Card`, `CardHeader`, `CardTitle`) are legacy.

```tsx
// The only way
<Dialog.Root open={open} onOpenChange={setOpen}>
  <Dialog.Trigger asChild>{children}</Dialog.Trigger>
  <Dialog.Content>
    <Dialog.Title>New Workflow</Dialog.Title>
  </Dialog.Content>
</Dialog.Root>
```

**Implementation:** Assembled via `Object.assign`. Include a `Root:` self-reference so consumers can use either `<Feature>` or `<Feature.Root>`.

**The main component function uses a `Root` suffix internally.** The barrel (`index.ts` or single-file assembly) strips the suffix for the public export.

For **primitives** (thin wrappers around shadcn/Radix), use a single file:

```tsx
// Card.tsx -- imports primitives, assembles namespace
import { Card as PrimitiveCard, CardHeader, CardTitle, ... } from "../primitives/card";

function CardRoot(props: ComponentProps<typeof PrimitiveCard>) {
  return <PrimitiveCard {...props} />;
}

export const Card = Object.assign(CardRoot, {
  Root: CardRoot,
  Header: CardHeader,
  Title: CardTitle,
});
```

For **business components** with Bnto-specific logic, each sub-component in its own file, assembled in `index.ts`:

```tsx
// FeatureRoot.tsx -- exports with Root suffix
export function FeatureRoot({ ... }: FeatureProps) { ... }

// index.ts -- strips Root suffix for public API
import { FeatureRoot } from "./FeatureRoot";
import { FeatureItem } from "./FeatureItem";

export const Feature = Object.assign(FeatureRoot, {
  Root: FeatureRoot,
  Item: FeatureItem,
});
```

**Migration rule:** When you touch a file with flat primitive imports (`DialogTitle`, `CardHeader`), migrate to dot-notation in the same PR.

### Primitives as Base

**When a shadcn primitive exists for a component, the Bnto wrapper MUST use it as its base** -- import the primitive and extend it with Bnto-specific props and behavior.

```tsx
// GOOD -- wrapper owns styling, delegates to primitive for the raw element
import { Button as PrimitiveButton } from "../primitives/button";

const buttonCn = createCn({
  base: "inline-flex items-center ...",
  variants: { variant: variantMap, size: sizeMap },
});

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ variant, size, className, ...rest }, ref) => (
    <PrimitiveButton ref={ref} className={buttonCn({ variant, size }, className)} {...rest} />
  ),
);

// BAD -- copy-pasting the primitive and bypassing it entirely
const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, ...props }, ref) => (
    <button className={cn("inline-flex ...", className)} ref={ref} {...props} />
  ),
);
```

### Primitives vs Business Components

| Type | Location | Domain knowledge | File sharing |
|---|---|---|---|
| **Primitives** | `primitives/` | None -- generic, reusable | shadcn compound wrappers can share a file |
| **Business** | `components/` or `apps/web/` | Domain-specific | One component per file, always |

---

## CSS-First Interaction States

**Never use JavaScript for visual states that CSS can handle.** Every `useState` for a visual concern is a re-render the browser didn't need. CSS pseudo-classes, data attributes, and ARIA selectors are free.

### Pseudo-Classes for Interaction States

```tsx
// GOOD -- CSS handles the visual state
<span className="opacity-0 hover:opacity-100 focus-within:opacity-100" />

// BAD -- JS re-renders for a visual hover effect
const [hovered, setHovered] = useState(false);
<span
  onMouseEnter={() => setHovered(true)}
  onMouseLeave={() => setHovered(false)}
  style={{ opacity: hovered ? 1 : 0 }}
/>
```

### Data Attributes Over Ternary ClassNames

When a component has state that drives visual changes, **reflect state as a data attribute and style with Tailwind selectors** instead of computing classes in JS with ternaries.

```tsx
// GOOD -- data attribute drives styling, zero JS class logic
<Button
  data-active={isActive}
  className="data-[active=true]:bg-background"
/>

// BAD -- JS ternary for class selection
<button className={isActive ? "bg-blue-500 text-white" : "bg-gray-500 text-gray-300"} />
```

### When JS State Is Legitimate

- **Coordinating sibling state** -- CSS can't express "style siblings based on one element's state"
- **Triggering side effects** (data prefetch on hover, analytics) -- not a visual concern
- **Complex multi-step interactions** (drag-and-drop sequences, gesture tracking) -- CSS doesn't have the expressiveness

### Hover/Focus Parity

Every `group-hover:` that reveals interactive controls **MUST** also have `group-focus-within:` so keyboard users see the same UI.

```tsx
// GOOD -- keyboard users see the same controls
className="opacity-0 group-hover:opacity-100 group-focus-within:opacity-100"

// BAD -- invisible to keyboard users
className="opacity-0 group-hover:opacity-100"
```

