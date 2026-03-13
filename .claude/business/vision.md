# Vision & Problem Statement

**Created:** February 2026
**Status:** Foundational
**Previously:** Notion — "Vision & Problem Statement"

---

## Origin Story

This idea came from building and running automation workflows daily as a software engineer. Every time Ryan McHenry needed to orchestrate a repeatable task — batch resize images, fetch data from an API and transform it, process files in a pipeline — the options were the same:

- **Write a bash script.** Fast to create, impossible to maintain, breaks when you look at it wrong. No error handling, no progress feedback, no reusability.
- **Use a no-code tool (Zapier, Make).** Locked into a vendor, pay per execution, limited to their integrations, zero control over the runtime. Can't run locally. Can't see the workflow.
- **Use n8n or similar.** Better — self-hostable, visual editor — but still requires a running server, Docker knowledge, and constant maintenance. Overkill for "compress these PNGs and upload them."
- **Build a custom solution.** Write Go/Python, wire up concurrency, handle errors, add logging. Works great, takes hours for something that should take minutes.

The gap isn't just a developer problem — it's everybody's problem. A designer needs to compress 10 images. A small team needs to rename and organize files before a client delivery. A solo founder needs to process a CSV every week. None of them have a good answer today.

Bnto fills that gap.

---

## The Problem

Getting a simple, repeatable task done shouldn't require a platform subscription, a server, or a computer science degree. But today it does.

**If you're non-technical:** You go to TinyPNG to compress images. You go to CloudConvert for file conversion. You Google a new single-purpose tool every time you need something. There's no place that just... does the thing. No account required to start. No per-task billing. The desktop app is free and unlimited. The web app has a generous free tier.

**If you're technical:** You either over-engineer a script that becomes unmaintainable, or you overpay for a platform that locks you into their ecosystem and punishes you for automating too much.

The 1-5 person startup. The design team. The indie hacker. The solo operator. None of the existing tools were built for them.

---

## The Vision

**Bnto is the place small teams go to get things done.**

Not the place that tries to replace Zapier for enterprise workflows. Not the tool that requires a PhD in DevOps. The tool that makes your Tuesday easier.

Drop 10 images. Run your flow. Done. That's it.

And if you want to build something more complex — chain nodes, loop over files, call an API, transform data — that works too. The same simple building blocks compose into whatever you need. One compartment or ten, it's still a bento box. Stack boxes and it's still a bento box. The mental model never changes, the complexity just scales with you.

The core philosophy:

- **Start with one node.** The simplest case is the entry point. Compress images, rename files, fetch a URL. No setup, no friction, just works.
- **Grow without leaving.** As your needs grow, your flows grow. You never outgrow the tool because the tool is just building blocks.
- **Honest limits, never punitive.** The desktop app is unlimited forever. Browser execution on the web is free and unlimited — it costs us $0. Pro sells persistence, collaboration, and server-side compute for nodes that have real infrastructure cost. We never gate node types, never reduce what's free, and never charge per execution for browser tools.
- **Local-first, cloud-optional.** Run on your machine with zero infrastructure. Run in the cloud when you want to. Same flow, same result.
- **Workflows are yours.** A `.bnto.json` file is just a file. Version control it. Share it. Copy it. It's not locked in a platform.

---

## What Bnto Is (and Is Not)

**Bnto IS:**
- A tool anyone can use to automate a repeatable task — no technical background required
- A Rust engine that executes `.bnto.json` recipe files (browser WASM, desktop native, CLI, cloud)
- A web app for building and running flows in the browser
- A free desktop app for local execution with a visual interface
- A cloud service for running workflows without local setup (paid)
- An open-source engine (MIT) with built-in node types for common tasks
- A portable workflow format that works everywhere

**Bnto IS NOT:**
- Built primarily for enterprise or Fortune 500 teams
- A Zapier/Make replacement — we don't build 1,000 integrations
- An ETL tool or data pipeline framework
- A CI/CD system (though workflows can complement CI)
- A serverless function platform

---

## Core Principles

**It just works.** The first-time experience should feel obvious. Drop your files, run your flow, get your output. No documentation required for the simple case.

**Abstract the complexity, expose the power.** Non-technical users never see the internals. Technical users can go as deep as they want. The tool doesn't force a ceiling.

**Aim small, grow naturally.** We're not trying to win enterprise contracts on day one. We're trying to be the tool a 3-person design team has bookmarked.

**Workflows are data, not code.** JSON is the lingua franca. Inspectable, diffable, portable.

**Local-first, cloud-optional.** The desktop app runs workflows for free using local compute. Cloud execution is a convenience layer, not a requirement.

**Open source core.** The engine and all node types are MIT-licensed. Cloud sells hosting and managed infrastructure, not proprietary features.

**Cost-first architecture.** $0/month infrastructure until revenue. Free tiers everywhere. The user's browser and local machine are powerful computers — use them.

---

## The Bnto Metaphor

A bento box (弁当) is a Japanese lunch container with carefully organized compartments. Each section has one purpose, holds one thing, and fits together into a complete meal.

Bnto workflows work the same way. Each node does one thing well. They compose into pipelines. The `.bnto.json` file is the box — portable, organized, complete.

And boxes stack. A flow can contain other flows. One compartment is still a bento box. Ten compartments is still a bento box. The mental model is the same at every scale — that's the point.

---

## Products

**Bnto Engine** (open source, MIT)
- Rust engine compiled to WASM for browser, native for desktop/CLI/cloud
- Built-in node types: image, csv, file, transform (more planned)
- Single cdylib WASM binary (606KB gzipped, all nodes)

**Bnto Web** (delivered — M1)
- Next.js app on Vercel + Convex Cloud + `@convex-dev/auth`
- 6 Tier 1 recipes running 100% client-side via Rust WASM
- Visual bento-box editor for building custom recipes

**Bnto Desktop** (M3 — free)
- Tauri app — same React frontend, local Rust engine execution
- No account needed, no cloud connectivity
- Full local execution including all node types

**Bnto Cloud** (M4 — paid)
- Rust compiled service on Railway for server-side execution
- Run workflows without local setup
- Pro tier ($8/month or $69/year) for persistence, collaboration, and server-side compute

---

## Tech Stack

| Layer | Technology |
|---|---|
| Engine | Rust (WASM for browser, native for desktop/CLI/cloud) |
| Web Frontend | Next.js on Vercel |
| Desktop | Tauri (Rust-native) |
| Database | Convex Cloud |
| Cloud Execution | Rust service on Railway (M4) |
| Auth | `@convex-dev/auth` |
| Shared UI | shadcn/ui + Tailwind CSS (Motorway design system) |
| Client State | Zustand |
| Server State | React Query + Convex real-time |
| Build | Turborepo + pnpm + Taskfile.dev |

---

*This document captures why Bnto needs to exist. Return here when making any decision that requires reconnecting with the core purpose.*
