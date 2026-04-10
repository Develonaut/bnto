# Core Design & Development Principles

**Created:** February 2026
**Updated:** March 2026
**Status:** Foundational — applies to every decision, always

These are not guidelines. They are the DNA of Bnto. Every line of code, every UI decision, every API design gets evaluated against these five. If something conflicts, go back to the drawing board.

---

## 1. TDD Red — Tests Are the Design Phase

Tests are not verification. Tests are **design.** Before writing a single line of implementation, write failing tests that describe what the feature should do. This is the Red phase of TDD — and it's the most important step in the entire development process.

**Why Red tests come first:**

- **They force you to think about the API before building it.** What does the caller pass in? What comes back? What happens on bad input? You answer these questions in test assertions, not in implementation code.
- **They define acceptance criteria in executable form.** When all Red tests turn Green, the feature is done. No ambiguity, no "I think it works."
- **They catch design mistakes early.** If a test is hard to write, the API is wrong. Refactor the interface before you've built anything behind it.
- **They prevent scope creep.** You only build what the tests require. No speculative features, no "while I'm here" additions.

**The Red-Green-Refactor cycle:**

```
1. RED    — Write a failing test that defines one behavior
2. GREEN  — Write the minimum code to make it pass
3. REFACTOR — Clean up while tests stay green
4. REPEAT — Next behavior, next Red test
```

The engine is discrete, testable, deterministic. Every node type ships with fixtures. Every predefined Bnto has a test. If you can't test it, you can't ship it.

Each layer owns its domain and is independently verifiable. Breaking changes are caught at the engine level before they reach a user. Agents verify their own work by running tests, not by claiming completion.

**The test suite IS the specification.** Someone reading your tests should understand exactly what the feature does — its contracts, its edge cases, its error paths — without reading a single line of implementation.

## 2. Go With the Grain

Every platform, language, and tool has a natural direction. Use Go idioms in Go, React patterns in React, Convex's real-time model instead of polling. If you're fighting a framework, reconsider the approach.

This is where "it just works" comes from. When software goes with the grain of its environment, the tool disappears and the task remains.

## 3. Modularity is Our Bread and Butter

One compartment is a bento box. Ninety-nine compartments is still a bento box. The mental model never changes.

- Every node does one thing well. Nodes compose into flows. Flows compose into larger flows.
- Single responsibility everywhere. No grab bags. Composable pieces over integrated blobs.
- The simplest Bnto is one node. The most complex is just more nodes, not a different product.

Think small, build small. A focused thing built well compounds.

## 4. Abstraction is the Goal

The user should never see complexity they didn't ask for. The developer should never see complexity from another layer.

**For users:** Every feature gets evaluated against _"did we make this easier?"_ The person who starts by compressing images and the person who builds a 20-node pipeline are using the same product. The mental model scales without changing.

**For developers:** Multi-tiered APIs where each layer is a clean abstraction over the one below. If changing one layer requires changing another, the abstraction is leaking -- fix the boundary. Can a new developer understand a layer without reading the layer below it?

## 5. Config as Code

Bnto is open source. Every piece of configuration — feature flags, recipe definitions, node schemas, rollout rules — should be representable in the repo. External dashboards (PostHog, Vercel, Convex) are runtime overrides, not the source of truth.

**Why this matters:**

- **Self-hosters can fork and run.** If config lives in a third-party dashboard, a fork is broken on day one. If it lives in the repo, `git clone` gives you a working product.
- **Contributors can see the full surface.** A contributor can't review, test, or modify what they can't see. Code-defined config is reviewable in PRs.
- **Git is the audit log.** Who changed what, when, and why — for free.
- **No vendor lock-in on configuration.** The repo works without any specific SaaS. External services enhance (targeting, analytics, rollout) but never gatekeep.

**The test:** If a new developer clones the repo, can they understand and modify every behavior without access to any external dashboard? If not, the config needs a code-defined default.

---

## How They Reinforce Each Other

Modularity makes testing easy. Going with the grain produces natural abstractions. Good abstractions enable composition. Config as code keeps the repo self-contained and forkable. TDD validates all of it continuously.

---

## For Claude Code

- **Write Red tests first.** Before implementing anything, write failing tests that define what the feature should do. The tests ARE the design. If you find yourself writing code without a failing test, stop and write the test first
- Write tests for every new node type and every predefined Bnto fixture before shipping
- Use each framework the way it wants to be used -- don't fight it
- Build small, composable pieces -- no monolithic functions or components
- Every API boundary should be a clean abstraction -- callers shouldn't need to know what's below
- When something feels awkward to build, that's a signal the design needs revisiting
- "Did we make the UX/DX easier?" is always a valid question to stop and ask
- Prefer code-defined defaults over dashboard-only config — the repo should be self-contained for self-hosters and contributors

---

## Trust Commitments

Public promises that constrain every build decision.

1. **Free tier never gets worse.** Current limits are the floor, not the ceiling.
2. **Desktop is free forever.** No "desktop Pro." Local execution is always unlimited.
3. **MIT license stays MIT.** The engine is always open. Users can run it themselves.
4. **No dark patterns.** No fake urgency, hidden limits, or surprise charges.
5. **No overpromising.** Ship it or don't announce it.
6. **If bnto shuts down, the engine stays open.** No lock-in, ever.

---

_Related: [code-standards.md](../rules/code-standards.md) covers the code-level application of modularity and single responsibility (the Bento Box Principle)._
