# Core Design & Development Principles

**Created:** February 2026
**Status:** Foundational — applies to every decision, always

---

These are not guidelines. They are the DNA of Bnto. Every line of code, every UI decision, every API design, every feature prioritization gets evaluated against these. If something conflicts with them, we go back to the drawing board.

When a technical or product decision feels unclear, run it through all four. Usually one of them is being violated.

---

## 1. TDD is the Core of Our Success

The Go engine is the heart of Bnto. It is a CLI — discrete, testable, deterministic. That is not a limitation, it is a superpower.

We maintain a suite of predefined Bntos that run as integration tests against the engine. Every node type, every flow pattern, every edge case has a corresponding `.bnto.json` fixture that we execute and validate. Success is binary and immediate — it either produces the correct output or it doesn't.

**Why this matters:**

- The CLI is the stable API. The web app, desktop app, and cloud execution are all UIs on top of it. If the CLI is rock solid, everything built on top of it is rock solid.
- Agentic coding (Claude Code) can run the test suite and know immediately if a change broke something. No guessing, no manual testing, no "it worked on my machine."
- n8n and similar platforms didn't start with this discipline. Retrofitting test coverage onto a complex system is painful and has made things rocky for them. We bake it in from day one.
- Breaking changes are caught at the engine level before they ever reach a user.

**The rule:** Every new node type ships with fixtures. Every predefined Bnto has a test. If you can't test it, you can't ship it.

**Each layer owns its domain:**

- Go engine: does the work, fully tested in isolation
- Web app: owns its UI domain, trusts the engine
- Desktop app: owns local execution, trusts the engine
- Cloud API: owns scheduling and delivery, trusts the engine

Layers don't leak into each other. Breaking changes are isolated. The engine doesn't care about the web. The web doesn't care about the desktop. Each layer is independently verifiable.

---

## 2. Go With the Grain

Don't fight your environment. Every platform, language, and tool has a natural direction — the way it wants to be used. Intuitive software is software that goes with that grain.

**In practice:**

- Use Go idioms in Go code. Don't write JavaScript-style Go.
- Use React patterns in React code. Don't fight the component model.
- Use shadcn the way shadcn is designed to be used. Don't override it to do something it wasn't built for.
- Use Convex's real-time model. Don't bolt polling onto a system built for subscriptions.
- The `.bnto.json` format should feel natural to read and write. If it feels awkward, the design is wrong.

**This is where "it just works" comes from.** When software goes with the grain of its environment, users feel it even if they can't name it. Things land where expected. Interactions feel obvious. The tool disappears and the task remains.

If you find yourself fighting a framework, a language, or a pattern — stop. That's a signal to reconsider the approach, not to push harder.

---

## 3. Modularity is Our Bread and Butter

One compartment is a bento box. Ninety-nine compartments is still a bento box. The mental model never changes. The complexity just scales.

This is not just a metaphor — it is the literal architecture of the system.

**At the engine level:**

- Every node does one thing and does it well
- Nodes compose into flows
- Flows compose into larger flows
- Nothing is monolithic

**At the codebase level:**

- Single responsibility everywhere
- No grab bags — packages, modules, and components have one clear purpose
- Composable pieces over integrated blobs

**At the product level:**

- The simplest Bnto is one node
- The most complex Bnto is just more nodes, not a different product
- Users never hit a wall where the tool stops working the way they expect

**The discipline:** Think small, build small. A focused thing built well compounds. A large thing built loosely collapses under its own weight. Every time we're tempted to build something big, ask: what are the small composable pieces this is made of?

---

## 4. Abstraction is the Goal

The user should never see complexity they didn't ask for. The developer should never see complexity that belongs to another layer.

**For users (UX):**

Every feature, every screen, every interaction gets evaluated against one question: *did we make this easier?* If the answer is no, we either have a very good reason or we go back to the drawing board. There is no neutral — every design decision either reduces friction or adds it.

The casual user should never know what a `.bnto.json` file is unless they want to. The power user should be able to inspect and edit one directly. The abstraction serves both without compromising either.

**The superpower framing:** Making easy tasks stupid simple is the same foundation that makes hard tasks easy. The person who starts by compressing a folder of images and the person who builds a 20-node pipeline calling external APIs are using the same product. The mental model never changes. The complexity just scales. This is the core of what makes Bnto different — it grows with the user without ever feeling like a different tool.

**For developers (DX / API design):**

Multi-tiered APIs with layers that don't care about anything but their own level. The web app calls the BntoService API. The BntoService API calls the engine. The engine executes nodes. Each layer is a clean abstraction over the one below it.

- Breaking changes are isolated to the layer where they originate
- APIs are stable contracts, not implementation details
- Callers never need to know how something works, only what it does
- If changing one layer requires changing another, the abstraction is leaking — fix the boundary

**The test:** Can a new developer understand what a layer does without reading the layer below it? If yes, the abstraction is working. If no, it needs more work.

---

## How These Principles Work Together

They reinforce each other. Modularity makes testing easy — small focused things are trivially testable. Going with the grain produces natural abstractions — you're not fighting the environment to hide complexity. Good abstractions enable modularity — clean boundaries make composition natural. And TDD validates all of it continuously.

```
TDD          → catches breaks immediately, enables confident iteration
Grain        → produces intuitive software, reduces fighting the environment
Modularity   → enables composition, keeps complexity manageable
Abstraction  → protects users and developers from complexity they don't need
```

---

## For Claude Code

These principles apply to everything you build in this repo:

- Write tests for every new node type and every predefined Bnto fixture before shipping
- Use Go, React, and each framework the way they want to be used — don't fight them
- Build small, composable pieces — no monolithic functions or components
- Every API boundary should be a clean abstraction — callers shouldn't need to know what's below
- When something feels awkward to build, that's a signal the design needs revisiting, not a reason to push through
- "Did we make the UX/DX easier?" is always a valid question to stop and ask

---

*Related: [code-standards.md](../rules/code-standards.md) covers the code-level application of modularity and single responsibility (the Bento Box Principle).*

---

## Trust Commitments

These are public promises. They constrain every build decision.

1. **Free tier never gets worse.** Current limits are the floor, not the ceiling.
2. **Desktop is free forever.** No "desktop Pro." Local execution is always unlimited.
3. **MIT license stays MIT.** The engine is always open. Users can run it themselves.
4. **No dark patterns.** No fake urgency, hidden limits, or surprise charges.
5. **No overpromising.** Ship it or don't announce it.
6. **If bnto shuts down, the engine stays open.** No lock-in, ever.
