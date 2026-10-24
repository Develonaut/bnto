# Evaluating Feature Requests

How bnto answers a request for new functionality, from a user, a consumer of the engine, or our own dogfooding. Read it before you design anything a request asks for.

---

## The Mindset

**bnto is a generalized workflow-node engine. It doesn't carry any one product's opinions.**

A request tells you about an **outcome** someone needs. The shape it suggests (a node, a param, a mode, a flag) is the requester's guess at a solution. It is not a spec.

Our job is to work out what bnto's standards say should exist, so that the requester, and everyone with a similar need, can get that outcome by composing generic pieces. Often the answer is that it already works, or that it is a bug.

This is [generic-building-block-nodes.md](../../decisions/generic-building-block-nodes.md) applied at intake. Read that decision first.

## The Steps

Work through these in order for each item in a request. Write down the answer to each step, because the evaluation record (below) is made of them.

### 1. Restate it as an outcome

One sentence, input → result. Drop:

- the requester's product nouns;
- their proposed shape;
- anything that only makes sense in their setup.

Keep their examples: they become the RED tests.

> "A field holding the first N `/`-separated segments of an item's relative name", not "a top-folder node for our index".

### 2. Check for a defect first

Ask whether bnto, today, gives a **wrong result silently**: it exits 0, writes somewhere it shouldn't, leaves a template as text, or `bnto check` passes something that can't work.

If so, it's a **defect**, not a feature:

- log it in [ISSUES.md](../../ISSUES.md) with a repro;
- fix it on the defect track (single-concern PR, RED test first);
- don't make it wait for a feature design.

### 3. Try it with today's nodes

Build the recipe and run it, sandboxed: a temp `HOME` and `BNTO_HOME`, and never the real `~/.bnto`. Don't argue from memory. For what nodes exist and what each one takes, use the generated docs (`packages/@bnto/nodes/docs/`) and `engine/catalog.snapshot.json`. `bnto list` and `bnto info` describe recipes, not nodes.

- **It works:** the answer is that recipe. Say so first in the reply. If it's useful beyond this requester, consider adding it as an example.
- **It works, but clumsily:** record the recipe and what makes it clumsy (too many steps, a side effect, a lossy edge case). Clumsiness is evidence for step 4, not proof that a new node is needed.
- **It doesn't work:** record exactly where it stops.
- **`shell-run` doesn't count.** It's a stopgap you can suggest, not "doable with nodes".

### 4. Find the smallest generic piece that's missing

Apply the four questions in [generic-building-block-nodes.md](../../decisions/generic-building-block-nodes.md#the-test) to each candidate. Work from the most general piece to the most specific:

1. **An engine feature** that every node inherits: templating, template filters, iteration, metadata flow, field scope, validation.
2. **A new param** on an existing node, if it is that node's one job done more fully, not a second job.
3. **A new node** that does one thing, named from [node-vocabulary.md](../rust/node-vocabulary.md) (verb, class, shared params).

**Check prior art.** Look at how n8n, Node-RED, NiFi, Zapier Formatter and Jinja or Liquid filters solve it. If they model it as a small composable step or a filter, so do we.

**Check what's already planned:**

- [PLAN.md's backlog](../../PLAN.md#backlog)
- [ISSUES.md](../../ISSUES.md)
- the active plans and trackers in `.claude/strategy/`

Fold the request into planned work instead of designing a duplicate.

### 5. Say what not to build

Name the literal shape the request implies, and why bnto won't build it, for example:

> No `top-folder` node: one use case, no prior art, and template filters cover it.

This is the part most likely to be lost later, so write it down.

### 6. Watch for these red flags

These mean a design has drifted into one product's opinions:

- a param or option named after a product, a customer or one use case;
- a **mode** param that turns a node into two nodes behind one name;
- a sidecar file format or a magic field only one recipe reads;
- a hard-coded opinion (a unit label, a locale, a folder layout) with no general param behind it;
- a node that only makes sense inside one recipe;
- a change that makes one interpolator, or one node, behave unlike the others.

### 7. Review, then plan

- **`/workflow-expert` review.** Every new node, param or verb gets one against `node-vocabulary.md` before code. Give the reviewer the live catalog, not the skill's own node table, which is stale until ISS-17 is fixed.
- **Plan if it's big.** Work that spans two or more PRs gets a plan per [feature-planning.md](feature-planning.md).
- **Tests.** The request's examples become the RED tests.
- **Reviews.** Every PR gets a cold review ([cold-review.md](../../decisions/cold-review.md)).

### 8. Record it and reply

Record the evaluation in `.claude/requests/YYYY-MM-DD-<slug>.md` (format below), and link it from any ISSUES.md entry or PLAN.md task it creates.

**The repo is public.** A record never names the requester, their company or product, or any private project. Describe the consumer instead ("a consumer that pins bnto by commit"), and restate their setup in generic terms.

Draft the reply to the requester, in this order:

1. what works today, with the recipe;
2. what's a bug, and the workaround until it's fixed;
3. what bnto will add, in its generic shape, and why that differs from what they asked for, when it does;
4. what bnto won't add, and why.

## The Evaluation Record

```markdown
# Request: <one-line outcome>

**Received:** <date>, from <a generic description, never a name>. **Evaluated:** <date>.
**Source:** <where the request lives>.

## <n>. <outcome, restated>

- **Verdict:** doable today | doable today, clumsily | defect | missing generic piece | declined (only one product's opinion; no generic piece behind it)
- **Today:** the recipe tried (or where it stops), with the command and output.
- **Defects found:** ISS-<n> (with link).
- **Generic piece:** what bnto should add, and its prior art.
- **Won't build:** the literal shape, and why.
- **Tracked in:** ISSUES.md / PLAN.md entries.

## Reply

<the reply drafted in step 8>
```

**Worked example:** [2026-10-24-folder-index.md](../../requests/2026-10-24-folder-index.md).
