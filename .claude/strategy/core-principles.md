# Core Design & Development Principles

**Created:** February 2026
**Updated:** March 2026
**Status:** Foundational — applies to every decision, always

These are not guidelines. They are the DNA of Bnto. Every line of code, every UI decision, every API design gets evaluated against these four. If something conflicts, go back to the drawing board.

---

## 1. TDD is the Core of Our Success

The engine is discrete, testable, deterministic. Every node type ships with fixtures. Every predefined Bnto has a test. If you can't test it, you can't ship it.

Each layer owns its domain and is independently verifiable. Breaking changes are caught at the engine level before they reach a user. Agents verify their own work by running tests, not by claiming completion.

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

**For users:** Every feature gets evaluated against *"did we make this easier?"* The person who starts by compressing images and the person who builds a 20-node pipeline are using the same product. The mental model scales without changing.

**For developers:** Multi-tiered APIs where each layer is a clean abstraction over the one below. If changing one layer requires changing another, the abstraction is leaking -- fix the boundary. Can a new developer understand a layer without reading the layer below it?

---

## How They Reinforce Each Other

Modularity makes testing easy. Going with the grain produces natural abstractions. Good abstractions enable composition. TDD validates all of it continuously.

---

## For Claude Code

- Write tests for every new node type and every predefined Bnto fixture before shipping
- Use each framework the way it wants to be used -- don't fight it
- Build small, composable pieces -- no monolithic functions or components
- Every API boundary should be a clean abstraction -- callers shouldn't need to know what's below
- When something feels awkward to build, that's a signal the design needs revisiting
- "Did we make the UX/DX easier?" is always a valid question to stop and ask

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

*Related: [code-standards.md](../rules/code-standards.md) covers the code-level application of modularity and single responsibility (the Bento Box Principle).*
