# Decision: Form Library & Validation Strategy

**Date:** February 28, 2026
**Status:** Research complete, pending implementation
**Backlog item:** "UX: Standardize Forms with React Hook Form + Zod" in PLAN.md

---

## Context

Forms across the app use ad-hoc `useState` for each field with no validation library. As the product grows (settings, team management, billing, recipe editor), form state management will become increasingly important. This document captures the research, comparison, and recommendation.

## Current State

### What exists today

| Surface | Pattern | Validation | Complexity |
|---|---|---|---|
| **SignInForm** (auth) | Raw `useState` per field (name, email, password, error, loading) | Try/catch on submit, single error string | Low |
| **Recipe configs** (6 variants) | Controlled components with `value` + `onChange` props, parent Zustand store | None — engine validates at execution | Low |
| **recipeFlowStore** (Zustand) | Factory-created per page mount, typed per slug via `BntoConfigMap` | None | Medium |

### What's NOT installed

- No `react-hook-form`, `formik`, `zod`, `yup`, or any form/validation library
- Zustand is used for app state (not forms)
- React Query for server state
- Radix UI primitives for inputs (Input, Select, Slider, Switch, Checkbox, etc.)
- All input components have Motorway visual treatment and dot-notation composition

---

## Two Distinct Paradigms

The product has two fundamentally different "form" paradigms. The library choice only applies to one of them.

### 1. Traditional forms (library territory)

Submit-once forms with validation. Small in number, low-to-medium complexity.

| Surface | Sprint | Fields | Notes |
|---|---|---|---|
| Sign-in | Existing | email, password | Basic validation needed |
| Sign-up | Existing | name, email, password | Email format, password min length |
| Settings/Profile | Sprint 3+ | name, email, avatar? | Not yet built |
| Team invite | Sprint 7 | email, role | Simple CRUD |
| Billing management | Sprint 7 | — | Stripe handles heavy form work |

**Total: 4-5 traditional forms across the entire product lifecycle.**

### 2. Config/editor state (NOT library territory)

Continuous, reactive state that updates Zustand stores on every change. Validation is domain-specific, driven by `ParameterSchema` from `@bnto/nodes`. A traditional form library would be a hindrance here.

| Surface | Sprint | Notes |
|---|---|---|
| Recipe config components (6 types) | Existing | Controlled components, parent Zustand store |
| NodeConfigPanel (10 node types) | Sprint 4 Wave 3 | Auto-generated from `ParameterSchema` schemas |
| Code editor (JSON editing) | Sprint 4B | CodeMirror 6 owns its own state model |
| Per-file config overrides | Backlog | Zustand store extension |
| Desktop local preferences | Sprint 5-6 | Simple key-value settings |

**These surfaces use Zustand + pure validation functions from `@bnto/nodes`. Do NOT apply a form library here.**

---

## Library Comparison

### React Hook Form (RHF)

| Metric | Value |
|---|---|
| GitHub Stars | 44.5k |
| npm Downloads | ~7M weekly |
| Bundle Size | ~10 KB gzipped |
| Maintenance | Active (updated Feb 2026) |
| State Model | Uncontrolled (refs) — minimal re-renders |
| Zod Integration | First-class via `@hookform/resolvers/zod` |
| shadcn/ui | Official integration guide |
| TypeScript | Strong |

**Pros:** Dominant ecosystem, tiny bundle, proven performance model (uncontrolled inputs = fewer re-renders), shadcn/ui first-class support, every React developer knows it.

**Cons:** Uncontrolled model can be confusing at first, API surface is large for simple forms.

### TanStack Form

| Metric | Value |
|---|---|
| GitHub Stars | ~5k |
| npm Downloads | ~115k weekly |
| Bundle Size | Varies by framework adapter |
| Maintenance | Active (TanStack ecosystem) |
| State Model | Fine-grained reactivity |
| Zod Integration | Plugin-based validation |
| shadcn/ui | Newer official integration guide |
| TypeScript | Best-in-class type inference |

**Pros:** Zero dependencies, best TypeScript type safety, fine-grained field control, framework-agnostic core (could work with future Tauri/non-React consumer).

**Cons:** Much smaller community (40x fewer downloads than RHF), newer/less battle-tested, shadcn/ui integration is newer and less documented.

### Formik

| Metric | Value |
|---|---|
| GitHub Stars | 34.2k |
| npm Downloads | ~600k weekly (declining) |
| Bundle Size | ~44 KB gzipped |
| Maintenance | Slow (last major: 2021) |
| State Model | Controlled via Context — re-renders entire form tree |
| Zod Integration | Via Yup primarily |

**Not recommended.** Bundle is 4x RHF, maintenance is slow, re-render model is the worst of all options. Effectively legacy.

### Conform

| Metric | Value |
|---|---|
| GitHub Stars | ~2.5k |
| npm Downloads | ~50k weekly |
| Bundle Size | Small |
| Maintenance | Active (v1.17.0, Feb 2026) |
| State Model | Progressive enhancement, web standards |

**Interesting for SSR-heavy apps** but bnto's forms are all client-side (`"use client"`). Progressive enhancement doesn't add value here.

### Custom Zustand + Zod (no library)

**Pros:** Zero new dependencies, already familiar pattern in codebase, total control.

**Cons:** Re-invents field-level errors, touched/dirty tracking, async validation, focus management. For 4-5 forms, the ROI of building a custom form system is low.

---

## Recommendation: React Hook Form + Zod

**For traditional forms only.** Config/editor state continues using Zustand + `@bnto/nodes` validation.

### Why RHF + Zod

1. **shadcn/ui alignment.** shadcn/ui has first-class RHF integration. Our primitives (Input, Select, Switch) work with it out of the box.
2. **Bundle budget.** ~10 KB gzipped for RHF + ~13 KB for Zod = ~23 KB total. Reasonable given it replaces hand-rolled validation that would accumulate similar weight.
3. **Uncontrolled = performance.** RHF's ref-based model means form inputs don't trigger React re-renders on every keystroke. Aligns with the performance-first architecture.
4. **Zod schemas are reusable.** The same Zod schemas used for client-side form validation can be shared with Convex validators (via `zodToConvex` or manual alignment). One source of truth for validation rules.
5. **Community standard.** 44.5k stars, 7M weekly downloads. Every React hire knows it. Zero ramp-up time.
6. **Minimal surface area needed.** We only need: `useForm`, `zodResolver`, field registration, error display. We don't need RHF's advanced features (field arrays, multi-step wizards, etc.).

### Why NOT TanStack Form

TanStack Form has better TypeScript inference, but:
- 40x smaller community
- shadcn/ui integration is less mature
- We have 4-5 simple forms — the type safety advantage doesn't justify the ecosystem risk
- If TanStack Form becomes dominant in 2-3 years, migrating 4-5 forms is trivial

### Why NOT custom Zustand + Zod

Building a custom form system for 4-5 simple forms is over-engineering. You'd end up reimplementing what RHF provides: field registration, error mapping, touched/dirty state, submit handling, validation timing (onBlur vs onChange vs onSubmit). Not worth it.

---

## Implementation Pattern

### Package: `@bnto/form`

Form infrastructure lives in a dedicated package at `packages/@bnto/form/`. This follows the same naming-by-role convention as `@bnto/auth` and `@bnto/backend` — the package name describes what it does, not what library it uses internally. If RHF is ever swapped for TanStack Form or something custom, consumers don't change.

```
packages/@bnto/form/
├── package.json          # depends on react-hook-form, @hookform/resolvers, zod
├── src/
│   ├── index.ts          # public API — re-exports hooks, schemas, utilities
│   ├── schemas/
│   │   └── auth.ts       # Zod schemas for auth forms
│   ├── useSignInForm.ts  # sign-in form hook
│   └── useSignUpForm.ts  # sign-up form hook
└── tsconfig.json
```

**Apps import from `@bnto/form`, never from `react-hook-form` directly.** Even if the package starts as thin wrappers, the boundary means we can swap the underlying library without touching any consumer code.

```bash
# Package owns the dependencies
cd packages/@bnto/form && pnpm add react-hook-form @hookform/resolvers zod

# Apps depend on the package
pnpm --filter @bnto/web add @bnto/form
```

### Hook pattern (one per form)

Each form gets a `useXxxForm` hook exported from `@bnto/form`. The component becomes a pure render shell.

```tsx
// packages/@bnto/form/src/useSignInForm.ts
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { signInSchema, type SignInFormData } from "./schemas/auth";

export function useSignInForm() {
  return useForm<SignInFormData>({
    resolver: zodResolver(signInSchema),
    defaultValues: { email: "", password: "" },
  });
}
```

### Component pattern (pure render shell)

```tsx
// apps/web — SignInForm.tsx
import { useSignInForm } from "@bnto/form";

export function SignInForm() {
  const { register, handleSubmit, formState: { errors, isSubmitting } } = useSignInForm();

  return (
    <form onSubmit={handleSubmit(handleSignIn)}>
      <Input {...register("email")} />
      {errors.email && <ErrorMessage>{errors.email.message}</ErrorMessage>}
      {/* ... */}
    </form>
  );
}
```

### What NOT to do

- Do NOT apply RHF to recipe config components — those stay as controlled components with Zustand
- Do NOT apply RHF to the NodeConfigPanel — that's schema-driven via `@bnto/nodes`
- Do NOT create a shared `<Form>` wrapper component — keep it simple, each form is self-contained
- Do NOT import `react-hook-form` or `zod` directly in `apps/web` — always go through `@bnto/form`

---

## Scope & Migration Order

### Phase 1: Foundation (backlog task)

1. Create `packages/@bnto/form` package with `react-hook-form`, `@hookform/resolvers`, `zod`
2. Create `schemas/auth.ts` — shared Zod schemas for auth
3. Create `useSignInForm.ts` hook
4. Create `useSignUpForm.ts` hook (shared fields + name)
5. Add `@bnto/form` as dependency in `apps/web`, refactor `SignInForm.tsx` to use the hooks
6. Field-level error display (per-field error messages, not a single error string)

### Phase 2: New forms use the pattern (Sprint 3+)

7. Settings/Profile form uses `useProfileForm` hook in `@bnto/form`
8. Any new form follows the `useXxxForm` + Zod schema pattern in `@bnto/form`

### Phase 3: Future forms (Sprint 7)

9. Team invite form uses `useTeamInviteForm` hook in `@bnto/form`
10. Any billing-adjacent forms (if Stripe doesn't handle it all)

---

## What This Does NOT Cover

- **Recipe config validation** — stays in `@bnto/nodes` `ParameterSchema` system
- **NodeConfigPanel** (Sprint 4) — auto-generated from schemas, not a form library concern
- **Editor store state** — Zustand, not forms
- **Server-side validation** — Convex validators handle this independently
- **Desktop forms** — desktop skips auth (free, no account), so no form needs until local preferences

---

## References

- [shadcn/ui RHF integration](https://ui.shadcn.com/docs/forms/react-hook-form)
- [shadcn/ui TanStack Form integration](https://ui.shadcn.com/docs/forms/tanstack-form)
- [React Hook Form](https://github.com/react-hook-form/react-hook-form) — 44.5k stars
- [TanStack Form comparison](https://tanstack.com/form/latest/docs/comparison)
- [Conform](https://github.com/edmundhung/conform) — 2.5k stars
- [shadcn/ui discussion on Form deprecation](https://github.com/shadcn-ui/ui/discussions/9505)
