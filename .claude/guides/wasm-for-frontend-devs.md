# WebAssembly for Frontend Developers — The Bnto Edition

A plain-English guide to understanding WebAssembly (WASM) for someone who's spent their career building with React and JavaScript. No jargon, lots of analogies, written like you're explaining it to a friend over coffee.

---

## What Is WASM, in One Sentence?

**WebAssembly is a way to run code written in other languages (like Rust, C, Go) inside the browser, at near-native speed, right alongside your JavaScript.**

That's it. That's the whole thing.

---

## The Playground Analogy

Imagine the browser is a playground. For 25 years, JavaScript was the only kid allowed to play on it. Every game, every toy, every activity on that playground — JavaScript handled it.

JavaScript is great at a lot of things. It's friendly, it's flexible, it picks up new tricks fast. But some activities — like compressing a 20MB image, or crunching through a million rows of CSV data — are like asking that kid to carry a refrigerator up a hill. It'll get there eventually, but it's going to be slow and sweaty.

**WASM is a second kid on the playground.** This kid grew up in a different neighborhood (Rust, C, C++) where they learned to be incredibly fast and efficient at heavy lifting. The browser said: "Hey, you can come play here too, as long as you follow my rules."

**JavaScript and WASM play together.** JS is still the social coordinator — it handles the UI, the DOM, the click events, the React components. But when there's heavy lifting to do? JS hands the work to WASM, waits for the result, and presents it to the user. Best of both worlds.

---

## Why Does WASM Exist?

### The Problem

JavaScript is an *interpreted* language. Your browser reads your JS code line by line and figures out what to do on the fly. Modern JS engines (V8 in Chrome, SpiderMonkey in Firefox) are incredibly smart about this — they do something called JIT (Just-In-Time) compilation to speed things up. But there's a ceiling.

Some tasks are fundamentally compute-heavy:
- **Image processing** — decoding pixels, applying compression algorithms, resizing
- **Data crunching** — parsing and transforming large CSV files, JSON blobs
- **Cryptography** — encryption, hashing
- **Games and 3D** — physics engines, rendering
- **Video/audio** — encoding, decoding, filtering

For these, JavaScript hits a wall. Not because JS is bad — but because it was designed for making web pages interactive, not for being a number-crunching powerhouse.

### The Solution

What if you could take code written in a language *designed* for performance (like C or Rust), compile it into a format the browser understands, and run it alongside JavaScript?

That's WebAssembly. It's a **compilation target** — a format that languages compile *down to*, like how TypeScript compiles down to JavaScript. But instead of compiling to human-readable JS, these languages compile to a compact binary format (`.wasm` files) that the browser can execute at near-native speed.

```
Your React code:    TypeScript  -->  JavaScript  -->  Browser interprets it
Your WASM code:     Rust        -->  .wasm binary -->  Browser executes it (fast!)
```

---

## How Fast Is "Near-Native Speed"?

When people say WASM runs at "near-native speed," they mean it runs almost as fast as if you'd downloaded and run a native app on your computer.

**Some real numbers from bnto:**

| Task | Pure JavaScript | Rust via WASM | Speedup |
|---|---|---|---|
| Compress a 5MB JPEG to 60% quality | ~800ms | ~120ms | ~6-7x faster |
| Parse + clean a 50K row CSV | ~400ms | ~60ms | ~6-7x faster |
| Rename 200 files (string processing) | ~15ms | ~3ms | ~5x faster |

The gap gets wider as the data gets bigger. For a 20MB image, you might see 10-15x improvements. For truly compute-heavy tasks (video encoding, physics simulation), it can be 20-50x.

**Why the speed difference?**

1. **Pre-compiled** — WASM arrives at the browser already compiled. JavaScript has to be parsed, optimized, and compiled on the fly
2. **Typed memory** — WASM works with fixed-size integers and floats (like `u8`, `f32`). JavaScript's `number` type is always a 64-bit float, even when you're counting to 5
3. **No garbage collector pauses** — JavaScript periodically pauses to clean up unused memory (garbage collection). Rust/WASM manages memory manually, so there are zero GC pauses
4. **Predictable performance** — JavaScript's JIT compiler makes optimistic guesses that sometimes turn out wrong, causing "deoptimizations." WASM's performance is consistent

---

## How Bnto Uses WASM

Here's the full picture of how a user compresses an image on bnto:

```
User drops a file onto the page
    |
    v
React component receives the File object
    |
    v
JavaScript reads the file into a Uint8Array (raw bytes)
    |
    v
JS sends those bytes to a Web Worker (background thread)
    |
    v
The Web Worker loads the .wasm file (one time, cached after that)
    |
    v
Worker calls the Rust function: compress_image(bytes, quality)
    |
    |   Inside WASM (Rust code running in the browser):
    |   1. Decode the image bytes into pixels
    |   2. Apply compression algorithm
    |   3. Encode pixels back to compressed bytes
    |   4. Return the result
    |
    v
Worker sends compressed bytes back to the main thread
    |
    v
React component creates a download link
    |
    v
User downloads the compressed image
```

**The files never leave the user's machine.** There's no server involved. The Rust code runs entirely in the browser — it just runs really, really fast.

### The Architecture in Bnto's Codebase

```
engine/                          <-- Rust code (compiles to WASM)
  crates/
    bnto-wasm/                   <-- The "front door" — JS calls these functions
    bnto-core/                   <-- Shared types and traits
    bnto-image/                  <-- Image compression/resize/convert logic
    bnto-csv/                    <-- CSV cleaning logic
    bnto-file/                   <-- File renaming logic

apps/web/
  public/wasm/
    bnto_wasm.js                 <-- Auto-generated JS "glue" code
    bnto_wasm_bg.wasm            <-- The compiled WASM binary (~2MB)
  lib/wasm/
    BntoWorker.ts                <-- TypeScript wrapper (main thread side)
    bnto.worker.ts               <-- Web Worker script (loads + calls WASM)
    types.ts                     <-- Message types for Worker communication

  components/                    <-- React components (your familiar territory)
    (these call BntoWorker, never touch WASM directly)
```

**The key insight:** React components never talk to WASM directly. They talk to `BntoWorker.ts` (a TypeScript class), which sends messages to a Web Worker, which calls the Rust functions. Your React code stays 100% normal React code.

---

## Where Does Each Piece Live? (The Thread Map)

This is the question that trips people up. There are **two threads** running in the browser, and different code lives on each one. Here's the definitive map:

```
┌─────────────────────────────────────────────────────────────────────┐
│                        MAIN THREAD                                  │
│                  (where React lives)                                │
│                                                                     │
│   ┌──────────────────────┐    ┌───────────────────────────────┐    │
│   │   React Components   │    │   @bnto/core                  │    │
│   │                      │    │                               │    │
│   │   RecipePage.tsx     │───>│   useWasmExecution() hook     │    │
│   │   FileDropZone.tsx   │    │   wasmExecutionService.ts     │    │
│   │   ResultsPanel.tsx   │    │   wasmExecutionStore (Zustand)│    │
│   │                      │    │                               │    │
│   └──────────────────────┘    └───────────┬───────────────────┘    │
│                                           │                        │
│                                           v                        │
│                               ┌───────────────────────┐           │
│                               │   BntoWorker.ts       │           │
│                               │   (TypeScript class)   │           │
│                               │                       │           │
│                               │   - Creates Worker    │           │
│                               │   - Sends messages    │           │
│                               │   - Correlates IDs    │           │
│                               │   - Returns Promises  │           │
│                               └───────────┬───────────┘           │
│                                           │                        │
│ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│
│                                    postMessage()                    │
│                               (ArrayBuffer transfer)               │
│ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─│
│                                           │                        │
│                        WORKER THREAD      │                        │
│                  (where WASM lives)       v                        │
│                                                                     │
│                               ┌───────────────────────┐           │
│                               │   bnto.worker.ts      │           │
│                               │   (Worker script, JS)  │           │
│                               │                       │           │
│                               │   - Receives messages │           │
│                               │   - Loads WASM module │           │
│                               │   - Calls Rust fns    │           │
│                               │   - Sends results back│           │
│                               └───────────┬───────────┘           │
│                                           │                        │
│                                           v                        │
│                               ┌───────────────────────┐           │
│                               │   bnto_wasm.js        │           │
│                               │   (auto-generated     │           │
│                               │    JS glue code)      │           │
│                               │                       │           │
│                               │   - Type conversion   │           │
│                               │   - Memory management │           │
│                               └───────────┬───────────┘           │
│                                           │                        │
│                                           v                        │
│                               ┌───────────────────────┐           │
│                               │   bnto_wasm_bg.wasm   │           │
│                               │   (compiled Rust)      │           │
│                               │                       │           │
│                               │   compress_image()    │           │
│                               │   clean_csv()         │           │
│                               │   rename_file()       │           │
│                               └───────────────────────┘           │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

### The Two Boundaries

There are actually **two boundaries** in this system, and they're different:

```
Main Thread          │  Worker Thread          │  WASM Sandbox
(JavaScript)         │  (JavaScript + WASM)    │  (Rust bytecode)
                     │                         │
React components     │  bnto.worker.ts         │  bnto_wasm_bg.wasm
BntoWorker.ts        │  bnto_wasm.js (glue)    │
Zustand stores       │                         │
                     │                         │
    ─── Boundary 1 ──┤─── ── ── Boundary 2 ───┤
    postMessage()    │  JS function calls      │
    (async, copies)  │  (sync, fast)           │
```

**Boundary 1: Main Thread ↔ Worker Thread.** This is the `postMessage()` boundary. Data crosses as serialized messages. `ArrayBuffer`s can be *transferred* (zero-copy ownership handoff) but everything else gets copied. This is **asynchronous** — you send a message and get a response later.

**Boundary 2: Worker JS ↔ WASM.** This is a function call boundary *within the same thread*. The worker's JavaScript calls exported Rust functions through the auto-generated glue code. This is **synchronous** — when JS calls `compress_image_combined()`, it blocks until Rust returns. (That's fine because we're already on a background thread — blocking here doesn't freeze the UI.)

### The Key Mental Model

**The main thread never touches WASM.** Not the binary, not the glue code, not the Rust functions. The main thread has zero WASM involvement. It only talks to `BntoWorker.ts`, which is a plain TypeScript class that wraps `postMessage()` calls.

**The worker thread has both JS and WASM.** The worker script (`bnto.worker.ts`) is JavaScript. It loads the WASM module, calls Rust functions, and marshals data. From the worker's perspective, calling a Rust function feels like calling any other JavaScript function — the glue code hides the boundary.

```
Main Thread:    JS ──postMessage──> JS ──function call──> WASM
                ^                   ^                     ^
                |                   |                     |
           React/Core         Worker script          Rust binary
           (your world)       (the bridge)           (the engine)
```

Think of it like ordering food through a delivery app:
- **You (main thread)** tap "Order" in the app. You don't walk to the restaurant.
- **The delivery driver (worker script)** picks up the order and brings it to the kitchen.
- **The chef (WASM/Rust)** cooks the food. The chef never talks to you directly.
- **The driver brings the food back** to your door. You eat it (render the result).

### The Full Message Flow

Here's exactly what happens when a user processes a file, with the thread boundary marked:

```
MAIN THREAD                          ║  WORKER THREAD
                                     ║
1. User drops a file                 ║
      │                              ║
2. React reads File → ArrayBuffer    ║
      │                              ║
3. BntoWorker.process()             ║
   creates ProcessRequest            ║
   {id: "abc", data: [bytes],        ║
    nodeType: "compress-images",     ║
    params: {quality: 75}}           ║
      │                              ║
4. postMessage(request, [data])  ════╬════════════════════>
   (ArrayBuffer transferred —        ║  5. onmessage receives request
    main thread loses access)        ║        │
      │                              ║  6. Worker looks up node type
      │  UI stays responsive         ║     in its local registry
      │  (React keeps rendering,     ║        │
      │   user can scroll, click)    ║  7. Worker calls WASM:
      │                              ║     compress_image_combined(
      │                              ║       data, filename,
      │                              ║       params, progressCb)
      │                              ║        │
      │                              ║  8. Rust compresses image...
      │                              ║     (100-500ms of CPU work)
      │                              ║        │
      │                              ║  9. Rust calls progressCb(50, "Encoding...")
      │                              ║        │
   <══╬═══════════════════════════════╬══ 10. Worker sends ProgressResponse
      │                              ║        │
11. BntoWorker fires onProgress     ║  12. Rust finishes, returns bytes
    → Zustand store updates          ║        │
    → React re-renders progress bar  ║  13. Worker sends ResultResponse
      │                              ║     postMessage(result, [data])
   <══╬═══════════════════════════════╬══     (ArrayBuffer transferred back)
      │                              ║
14. BntoWorker resolves Promise     ║
    → Store: processing → completed  ║
    → React renders download link    ║
      │                              ║
15. User clicks download             ║
```

Notice: steps 4 through 13 happen **concurrently**. The main thread is free to do other things (step 11 — update the UI with progress) while the worker crunches data. This is the whole point of the Worker architecture.

---

## The JS-WASM Boundary (The Border Crossing)

Think of the WASM boundary like a border crossing between two countries. Both countries are inside the browser, but they speak different languages and have different rules. Every time data crosses the border, it has to go through customs.

### What Crosses the Border Easily

| Data type | JS side | Rust/WASM side | Cost |
|---|---|---|---|
| Numbers | `number` | `u8`, `u32`, `f64`, etc. | Free (instant) |
| Booleans | `boolean` | `bool` | Free (instant) |
| Byte arrays | `Uint8Array` | `Vec<u8>` / `&[u8]` | Cheap (memory copy) |

### What's More Expensive to Cross

| Data type | JS side | Rust/WASM side | Cost |
|---|---|---|---|
| Strings | `string` | `String` / `&str` | Moderate (UTF-16 <-> UTF-8 conversion) |
| Objects | `{ name: "foo" }` | Not directly supported | Expensive (must serialize/deserialize) |
| Arrays of objects | `[{...}, {...}]` | Not directly supported | Expensive |

### What Can't Cross at All

| Thing | Why not |
|---|---|
| DOM elements | WASM has no access to the DOM |
| React components | WASM doesn't know React exists |
| `fetch()` / network | WASM can't make HTTP requests directly |
| File system | WASM can't read files from disk |
| `window`, `document` | WASM has no access to browser APIs |

**This is why the data flow matters.** JavaScript reads the file, converts it to bytes (`Uint8Array`), passes the bytes to WASM, gets bytes back, and turns them into a download. WASM never touches the DOM, never makes network requests, never reads files. It just crunches bytes.

### The Mental Model: WASM Is a Calculator

Think of WASM like a really fast calculator on your desk. You (JavaScript) type in numbers, press a button, and read the answer off the screen. The calculator doesn't know about your email, your browser tabs, or your file system. It just does math on the numbers you give it.

```
You (JS):        "Here are 5 million bytes. Compress them at quality 75."
Calculator (WASM): *crunches for 100ms*
Calculator (WASM): "Here are 1.2 million bytes back."
You (JS):        "Thanks!" *creates download link*
```

---

## What WASM Is Great At

### 1. CPU-Intensive Computation

Anything where you're crunching numbers, processing data, or running algorithms. This is WASM's home turf.

- Image compression, resizing, format conversion
- CSV/data parsing and transformation
- Hashing and encryption
- Video/audio processing
- Physics simulation, pathfinding, game logic
- Machine learning inference

### 2. Predictable, Consistent Performance

No garbage collector pauses. No JIT deoptimizations. When WASM says "this takes 100ms," it takes 100ms every time. JavaScript's performance can vary wildly depending on what the JIT compiler decides to do.

### 3. Reusing Existing Libraries

Decades of battle-tested C/C++/Rust libraries suddenly work in the browser. The `image` crate that bnto-image uses? It's the same library that runs in production servers processing millions of images. We didn't have to rewrite image compression in JavaScript — we just compiled the Rust version to WASM.

### 4. Security Through Isolation

WASM runs in a **sandbox**. It can't access the DOM, the network, the file system, or any browser API unless JavaScript explicitly passes data in. This is a security feature — a malicious WASM module can't steal cookies, make network requests, or modify the page. It can only operate on the data you give it.

### 5. Portable

A `.wasm` file runs the same way in Chrome, Firefox, Safari, and Edge. It also runs in Node.js, Deno, and standalone runtimes like Wasmtime. Write once, run everywhere — for real this time.

### 6. Small Binary Size

WASM binaries are compact. Bnto's entire engine (image compression, CSV processing, file renaming) compiles to about ~2MB. Compare that to shipping the equivalent JavaScript libraries, which would be larger *and* slower.

---

## What WASM Is NOT Great At

### 1. DOM Manipulation

WASM cannot touch the DOM. At all. If you need to update the UI, add elements, change styles, handle click events — that's JavaScript's job. React stays in charge of the UI. Always.

This is NOT a limitation for bnto — we *want* JS/React handling the UI. But it's why you'll never see "WASM replaces JavaScript" — they do fundamentally different things.

### 2. Calling Browser APIs

`fetch()`, `localStorage`, `WebSocket`, `navigator.clipboard`, `document.querySelector` — none of these are available from inside WASM. If WASM needs to make a network request, JavaScript has to do it and pass the result in.

### 3. Small, Quick Operations

If you're formatting a date, capitalizing a string, or filtering a 50-item array — JavaScript is actually faster. The overhead of crossing the JS-WASM boundary (copying data back and forth) makes WASM slower for tiny operations. WASM wins when the computation is heavy enough to justify the border crossing cost.

**Rule of thumb:** If the operation takes less than ~1ms in JavaScript, don't bother with WASM. If it takes more than ~50ms, WASM is probably worth it. In between, measure.

### 4. Startup Time

The `.wasm` file needs to be downloaded, compiled, and instantiated before it can run. For bnto, this takes about 200-400ms on first load (then it's cached). This is why we load WASM lazily — only when the user actually needs to process a file, not on page load.

### 5. Debugging

Debugging WASM is harder than debugging JavaScript. You can't just open DevTools and set a breakpoint in Rust code (well, technically you can with source maps, but it's clunky). When something goes wrong in WASM, you mostly see opaque error messages. This is why bnto's Rust code is heavily tested — we catch bugs in Rust unit tests, not in the browser.

### 6. String-Heavy Work

Every string that crosses the WASM boundary gets converted between UTF-16 (JavaScript) and UTF-8 (Rust). If your workload is "process a million tiny strings," the conversion overhead can eat into the performance gains. Byte arrays (`Uint8Array`) are the ideal data format for the boundary.

### 7. Multithreading (Sort Of)

WASM itself supports threads via `SharedArrayBuffer`, but browser support is spotty and the ergonomics are painful. Bnto sidesteps this by running WASM in a **Web Worker** (a separate thread from the main thread), which gives us the main benefit we need: the UI doesn't freeze while WASM is crunching data.

---

## Web Workers: WASM's Best Friend

You'll see Web Workers mentioned a lot alongside WASM. Here's why they're important:

**The problem:** JavaScript (and WASM) run on the browser's main thread by default. The main thread is also responsible for rendering the UI, handling click events, and running your React components. If WASM spends 500ms compressing an image *on the main thread*, the UI freezes for 500ms. Buttons don't respond, animations stutter, the page feels broken.

**The solution:** Web Workers. A Web Worker is a separate thread that runs in the background. It has its own JavaScript environment, separate from the main thread. You can send data to it, it processes the data, and sends the result back.

```
Main Thread (React lives here)          Web Worker (WASM lives here)
  |                                       |
  |-- "Here's a file, compress it" -----> |
  |                                       | (loads WASM, processes file)
  |   (UI stays responsive, user          | (this takes 500ms but
  |    can still scroll, click,           |  nobody notices because
  |    interact with the page)            |  it's on a different thread)
  |                                       |
  | <-- "Done! Here's the result" --------|
  |
  v
  Show download button
```

**In bnto, WASM always runs in a Web Worker.** The main thread sends bytes to the worker, the worker calls WASM, and sends bytes back. The React UI never blocks.

Think of it like a restaurant:
- **Main thread** = the front of house (waiter, host, ambiance). Handles customers (users)
- **Web Worker** = the kitchen. Does the heavy cooking (processing)
- **WASM** = the chef. Really fast and skilled at cooking (computation)
- **Messages** = the order tickets that go between front and back

The waiter (main thread) takes the order (file), passes the ticket to the kitchen (Worker), the chef (WASM) cooks it, and the waiter brings the finished plate (result) back to the customer (UI). The customer never waits at an empty table while the chef is cooking — the waiter keeps refilling their water.

### Why Not Load WASM on the Main Thread?

You *could* load the `.wasm` file directly on the main thread. It would work. But it would be a terrible user experience:

```
Main thread WITHOUT a Worker:          Main thread WITH a Worker:

User drops file                        User drops file
  │                                      │
  ▼                                      ▼
Load WASM (200ms)                      Send to Worker
  │  ← UI frozen                         │  ← UI free
  ▼                                      │
Compress image (300ms)                   │  User scrolls, clicks,
  │  ← UI frozen                         │  sees progress bar update
  ▼                                      │
Show result                              ▼
                                       Show result
Total: 500ms of frozen UI             Total: 0ms of frozen UI
```

The total processing time is the same. The difference is **who feels it**. With a Worker, the user's main thread stays free — progress bars animate, buttons respond, the app feels alive. Without a Worker, the page turns into a frozen screenshot for half a second.

**This is why bnto's architecture has a hard rule:** WASM is always loaded in, and called from, the Worker thread. The main thread only has TypeScript/JavaScript — `BntoWorker.ts`, React components, Zustand stores. No `.wasm` imports, no glue code, no Rust function calls. If you see WASM touching the main thread, it's a bug.

---

## WASM vs. Just Writing It in JavaScript

Fair question. Why not just write the image compression in JavaScript? Libraries like `browser-image-compression` exist.

| Factor | JavaScript library | Rust via WASM |
|---|---|---|
| **Speed** | Adequate for small files. Struggles with large ones | 5-15x faster on large files |
| **Consistency** | Performance varies (GC pauses, JIT mood) | Predictable, consistent |
| **Bundle size** | Often larger (JS is verbose) | Compact binary |
| **Ecosystem** | Pure-JS libraries exist but are limited | Access to the entire Rust/C ecosystem |
| **Quality** | JS image codecs are often simplified | Rust `image` crate = production-grade codecs |
| **Thread safety** | Single-threaded (or complex Worker setup) | Naturally isolates in a Worker |
| **Development complexity** | Easy to write and debug | Harder to write, harder to debug |
| **Maintenance** | Easier to read and modify | Requires Rust knowledge |

**Bnto's bet:** For a tool that processes files, the user experience improvement from 5-15x faster processing justifies the added development complexity. When you drop a 15MB photo and get the compressed version back in 200ms instead of 2 seconds, that's the difference between "this is magic" and "this is loading."

---

## The WASM Ecosystem (Briefly)

### Who Uses WASM in Production?

- **Figma** — their design tool engine is C++ compiled to WASM. This is the poster child for WASM success
- **Google Earth** — the 3D globe rendering runs in WASM
- **Photoshop** (web version) — image processing and filters via WASM
- **Squoosh** (by Google) — image compression tool, very similar to what bnto does
- **AutoCAD** (web version) — CAD rendering engine in WASM
- **Blazor** (.NET in the browser via WASM)
- **FFmpeg.wasm** — video processing in the browser

### The Standard

WASM is a W3C standard (same body that standardizes HTML, CSS, and the DOM). It's not a Google project or a Facebook project — it's a cross-browser, cross-vendor standard supported by all major browsers since 2017.

### Browser Support

WASM works in every modern browser. Period.

| Browser | WASM support since |
|---|---|
| Chrome | March 2017 |
| Firefox | March 2017 |
| Safari | September 2017 |
| Edge | October 2017 |

Global browser support is over 96% (as of 2025). The remaining ~4% is mostly very old browsers that wouldn't run a modern React app either.

---

## How WASM Actually Works (Under the Hood)

You don't need to know this to use WASM, but it might satisfy your curiosity.

### The Compilation Pipeline

```
Rust source code (.rs files)
    |
    | rustc (Rust compiler) + wasm-pack
    v
WebAssembly binary (.wasm file)
    ~2MB of compact binary instructions
    |
    | wasm-bindgen (generates JS glue code)
    v
JavaScript glue file (.js)
    Thin wrapper that handles:
    - Loading the .wasm file
    - Converting JS types <-> WASM types
    - Memory management
    |
    | Browser downloads both files
    v
Browser compiles .wasm to native machine code
    This happens FAST — much faster than parsing
    equivalent JavaScript, because .wasm is already
    in a structured binary format
    |
    v
Native machine code runs on your CPU
    Almost as fast as a natively compiled app
```

### What's Inside a .wasm File?

A `.wasm` file is a binary format (not human-readable) that contains:

1. **Functions** — the actual code, in a stack-based instruction format
2. **Memory** — a linear block of bytes (like a big `ArrayBuffer`) that the WASM code reads and writes to
3. **Imports** — functions it expects JavaScript to provide
4. **Exports** — functions that JavaScript can call

The browser's WASM runtime compiles these instructions to native machine code for your CPU architecture (x86 on Intel/AMD, ARM on Apple Silicon/mobile). This is why WASM is fast — by the time it runs, it's actual machine instructions, not interpreted bytecode.

### The Memory Model

This is the biggest conceptual difference from JavaScript:

**JavaScript:** Objects live in a managed heap. The garbage collector automatically frees them when they're no longer referenced. You never think about memory.

**WASM:** Has a single, flat block of memory (a `WebAssembly.Memory` object, which is backed by an `ArrayBuffer`). Rust manages this memory manually — it knows exactly when to allocate and free. This is why there are no GC pauses.

When you pass a `Uint8Array` from JS to WASM:
1. The bytes are copied into WASM's linear memory
2. WASM processes them in-place (fast — just pointer arithmetic)
3. The result bytes are copied back to a new `Uint8Array` in JS

The "copy in, process, copy out" pattern is why WASM is fastest with large blobs of bytes and less ideal for many tiny string operations.

---

## Key Terms Glossary

| Term | Plain English |
|---|---|
| **WASM / WebAssembly** | A binary format that lets non-JS languages run in the browser |
| **`.wasm` file** | The compiled binary. Like a `.js` file but for WASM |
| **wasm-pack** | A build tool that compiles Rust to WASM (+ generates JS glue code) |
| **wasm-bindgen** | A Rust library that generates the JS <-> Rust bridge code |
| **JS glue code** | Auto-generated JavaScript that handles type conversion and memory |
| **Linear memory** | WASM's flat block of bytes — like one giant `ArrayBuffer` |
| **Host** | The environment running WASM (usually the browser, but can be Node.js) |
| **Sandbox** | The security boundary. WASM can't escape it — no DOM, no network, no filesystem |
| **cdylib** | A Rust "crate type" that produces a shared library (.wasm in our case) |
| **Web Worker** | A browser API that runs JS in a background thread (separate from UI) |
| **`Uint8Array`** | JavaScript's typed array for raw bytes — the lingua franca of the WASM boundary |
| **Instantiation** | The one-time setup cost of loading and compiling a .wasm module |

---

## Frequently Asked Questions

### "Will WASM replace JavaScript?"

No. They do different things. JavaScript is the language of the web platform — it owns the DOM, events, and the entire UI layer. WASM is a computation engine. Think of it like asking "will the engine replace the steering wheel?" You need both.

### "Do I need to learn Rust to use WASM?"

In bnto, no. The WASM boundary is wrapped in TypeScript (`BntoWorker.ts`). From a React component's perspective, you're calling a TypeScript class that returns a Promise. You'd only need Rust to *write new engine nodes* — not to use existing ones.

### "Is WASM safe? Can it access my files?"

WASM is sandboxed. It runs inside the browser's security model. It cannot access the filesystem, the network, cookies, localStorage, or any browser API. The only data it can see is what JavaScript explicitly passes to it. In bnto, that's the file bytes the user chose to upload — nothing else.

### "Why not just use a server?"

Three reasons:
1. **Privacy** — files never leave the user's machine. No upload, no server storage, no data in transit
2. **Cost** — server compute costs money per request. Browser compute is free (the user's CPU does the work)
3. **Speed** — no network round-trip. Drop a file, get the result instantly

This is bnto's entire philosophy: **your browser is a powerful computer — use it.**

### "Can I use WASM with React?"

Absolutely. WASM doesn't know or care about your UI framework. In bnto, React components call `BntoWorker.process()`, get back a `Uint8Array`, and render the result. WASM is just another async operation, like `fetch()` — you call it, await the result, update state. React never needs to know the computation happened in Rust.

### "How big is a .wasm file?"

Bnto's entire engine (image + CSV + file processing) is about ~2MB. It's cached by the browser like any other static asset. For comparison, a typical React app bundle is 300KB-1MB of JavaScript. The WASM file is bigger, but it only loads when needed and downloads in parallel with everything else.

### "What about mobile?"

WASM works on mobile browsers (iOS Safari, Chrome for Android). Performance is good — mobile CPUs are fast enough for most WASM workloads. The main constraint is memory — mobile browsers have lower memory limits, so processing very large files (50MB+) can be an issue.

---

## The One-Paragraph Summary

WebAssembly lets you run code written in fast languages (Rust, C++) inside the browser, at near-native speed, in a secure sandbox. It's not a replacement for JavaScript — it's a companion for heavy computation. JavaScript handles the UI, events, and browser APIs. WASM handles the number crunching. In bnto, this means users can compress images, clean CSVs, and rename files entirely in their browser, with no server involved, at speeds that feel instant. React components call a TypeScript wrapper, which talks to a Web Worker, which runs the Rust code compiled to WASM. From the React component's perspective, it's just another async function call.

---

## Further Reading

- [WebAssembly.org](https://webassembly.org/) — the official site, good high-level overview
- [MDN WebAssembly Guide](https://developer.mozilla.org/en-US/docs/WebAssembly) — practical browser-focused documentation
- [Lin Clark's Cartoon Intro to WASM](https://hacks.mozilla.org/2017/02/a-cartoon-intro-to-webassembly/) — the best visual explainer out there (seriously, read this one)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/) — how the Rust <-> JS bridge works
- [Squoosh](https://squoosh.app/) — Google's WASM-powered image compressor (very similar to bnto's image compression)
