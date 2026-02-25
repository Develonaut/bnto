# Rust for Node/React Developers — The Bnto Edition

A reference guide for understanding Rust through the lens of Node/npm/React. Written for the upcoming M1 browser execution work (Rust→WASM via `wasm-pack`).

---

## The Big Picture: npm world vs Rust world

| Concept | Node/npm | Rust |
|---|---|---|
| **Package manager** | `pnpm` | `cargo` (built into Rust — no separate install) |
| **Package config** | `package.json` | `Cargo.toml` |
| **Lock file** | `pnpm-lock.yaml` | `Cargo.lock` |
| **A package** | A folder with `package.json` | A "crate" — a folder with `Cargo.toml` |
| **Scripts** | `"scripts"` in package.json | Built-in: `cargo build`, `cargo test`, `cargo run` |
| **node_modules** | Downloaded deps folder | `~/.cargo/registry` (global cache, invisible) |
| **Imports** | `import { thing } from "@bnto/core"` | `use image::DynamicImage;` |
| **Build output** | Still JS files, needs Node to run | Native binary (or `.wasm` for browser) |
| **Workspace** | `pnpm-workspace.yaml` | `[workspace]` in root `Cargo.toml` |
| **npmjs.com** | Package registry | [crates.io](https://crates.io) |

**The biggest shift:** Node runs your code as-is. Rust compiles it first. You write code, then `cargo build` turns it into a binary (or WASM). Compilation catches most bugs before your code ever runs — the compiler is your strictest code reviewer.

---

## Our Rust Setup (engine/)

```
engine/                          <- Rust WASM engine (primary)
├── Cargo.toml                   <- like package.json
├── src/
│   ├── lib.rs                   <- entry point (like index.ts)
│   ├── image/                   <- image processing nodes
│   │   ├── mod.rs               <- module declaration (like index.ts barrel)
│   │   ├── compress.rs          <- compress-images node
│   │   └── resize.rs            <- resize-images node
│   └── csv/                     <- csv processing nodes
│       ├── mod.rs
│       └── clean.rs
└── tests/                       <- integration tests (like __tests__/)
```

Build for WASM:
```bash
wasm-pack build --target web      # like: pnpm build, but outputs .wasm + JS glue
```

---

## Reading a Rust File — Annotated

```rust
// Every file is a module. Modules are like TypeScript files —
// each one has its own scope. You import things with `use`.

use image::{DynamicImage, ImageFormat};    // External crate (like npm package)
use std::io::Cursor;                       // Standard library (like Node built-ins)
use wasm_bindgen::prelude::*;              // WASM bridge (lets JS call Rust)

// Struct = your "class" (but no inheritance, ever — same as Go)
// Think: interface + data, no methods yet
pub struct CompressOptions {
    pub quality: u8,          // u8 = unsigned 8-bit integer (0-255)
    pub format: ImageFormat,  // an enum from the `image` crate
}

// `impl` block = where you add methods to a struct
// Think: class CompressOptions { ... methods here ... }
impl CompressOptions {
    // Associated function (no `self`) = static method / constructor
    // Think: CompressOptions.default() — called on the type, not an instance
    pub fn default() -> Self {
        Self {
            quality: 80,
            format: ImageFormat::Jpeg,
        }
    }
}

// #[wasm_bindgen] = decorator that makes this callable from JavaScript
// Think: export function compress(data: Uint8Array): Uint8Array
#[wasm_bindgen]
pub fn compress(data: &[u8], quality: u8) -> Result<Vec<u8>, JsValue> {
    //           ^^^^                        ^^^^^^^^^^^^^^^^^^^^^^
    //           borrowed slice               Result = can succeed or fail
    //           (read-only view              Ok(data) or Err(error)
    //            of bytes)                   This replaces try/catch

    // `?` at the end = "if this fails, return the error immediately"
    // It's like: const img = await loadImage(data) but the error propagates
    let img = image::load_from_memory(data)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    //   ^^^^^^^^ convert Rust error to JS error  ^^^ the magic ? operator

    let mut output = Vec::new();       // mut = mutable. Rust defaults to immutable
    //  ^^^ like: let output = []      // You must opt-in to mutation — opposite of JS

    img.write_to(&mut Cursor::new(&mut output), ImageFormat::Jpeg)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(output)    // Ok() wraps the success value. Like: return output
}
```

---

## Visibility: Almost the Same as Go

**`pub` = exported (public). No `pub` = private.**

```rust
pub struct Engine { ... }           // Exported — other modules can use it
struct InternalState { ... }        // Private — only this module sees it

pub fn compress() { ... }          // Exported — callable from outside
fn validate() { ... }              // Private — internal helper

pub mod image;                     // Exported module (like re-exporting a folder)
mod internal;                      // Private module
```

TS equivalent:
```typescript
export class Engine { ... }        // pub struct
class InternalState { ... }        // no pub

export function compress() { }     // pub fn
function validate() { }            // no pub
```

Go equivalent:
```go
type Engine struct { ... }         // PascalCase = exported
type internalState struct { ... }  // camelCase = unexported
```

**Struct fields are private by default too.** Each field needs its own `pub`:
```rust
pub struct Options {
    pub quality: u8,       // accessible from outside
    format: String,        // private — only this module can read it
}
```

---

## The Ownership System — The Big Rust Thing

This is the concept that doesn't exist in Node, Go, or any GC language. It's Rust's killer feature and its steepest learning curve.

### The Rule (just one rule, three consequences)

**Every value in Rust has exactly one owner. When the owner goes out of scope, the value is dropped (freed).**

```rust
fn main() {
    let name = String::from("bnto");    // `name` owns this string
    let other = name;                    // ownership MOVES to `other`
    // println!("{}", name);             // COMPILE ERROR — `name` no longer owns it
    println!("{}", other);               // works — `other` is the owner now
}
```

TS equivalent (conceptually):
```typescript
// Imagine if JavaScript did this:
let name = "bnto";
let other = name;     // In JS, both variables point to the same string. Fine.
console.log(name);    // In JS, this works. In Rust, it wouldn't.
```

**Why?** No garbage collector. Rust needs to know exactly when to free memory. One owner = one clear moment to free it. This is what makes Rust fast and safe without a GC.

### Borrowing: The Escape Hatch

You usually don't want to *move* values around. You want to *lend* them temporarily. That's borrowing.

```rust
fn count_bytes(data: &[u8]) -> usize {
    //                ^ the & means "I'm borrowing this, not taking ownership"
    data.len()
}   // data goes out of scope, but nothing is freed — we were just borrowing

fn main() {
    let bytes = vec![1, 2, 3, 4, 5];
    let count = count_bytes(&bytes);    // lend bytes to the function
    //                      ^ "borrow" — bytes is still ours after the call
    println!("bytes: {:?}, count: {}", bytes, count);  // bytes still usable
}
```

TS equivalent:
```typescript
// In TS, everything is always "borrowed" (passed by reference for objects).
// Rust makes you be explicit about it:
function countBytes(data: Uint8Array): number {  // TS: always a reference
    return data.length;
}
```

### Two Kinds of Borrowing

| Kind | Syntax | TS Equivalent | Rule |
|---|---|---|---|
| **Shared borrow** | `&data` | `readonly` / `Readonly<T>` | Many readers allowed, no writing |
| **Mutable borrow** | `&mut data` | Normal reference | Only ONE writer, no other readers |

```rust
let mut scores = vec![100, 200, 300];

// Multiple shared borrows — fine (many readers)
let first = &scores[0];
let second = &scores[1];
println!("{} {}", first, second);

// Mutable borrow — exclusive access (one writer)
scores.push(400);        // OK — no other borrows active
```

**The mental model:** Shared borrows (`&`) are like passing `Readonly<T>` in TypeScript. Mutable borrows (`&mut`) are like having an exclusive lock — nobody else can touch it while you're writing.

### When You'll Actually Hit This

In WASM node code, ownership matters most when:
1. **Passing data to functions** — use `&` to borrow instead of moving
2. **Returning data** — return owned values (`Vec<u8>`, `String`), not borrows
3. **Working with buffers** — `&[u8]` (borrowed slice) for input, `Vec<u8>` (owned) for output

```rust
// Common pattern for our WASM nodes:
// Take borrowed input (cheap), return owned output (caller takes ownership)
pub fn compress(input: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    //           ^^^^^ borrowed: we read it      ^^^^^^ owned: caller gets it
    let img = load(input)?;
    let output = encode(img, quality)?;
    Ok(output)    // ownership of output moves to the caller
}
```

---

## Error Handling: Result Instead of try/catch

Rust has no exceptions. Functions that can fail return `Result<Success, Error>`.

```rust
// Result is an enum with two variants:
//   Ok(value)    — success
//   Err(error)   — failure

fn parse_quality(input: &str) -> Result<u8, String> {
    let num: u8 = input.parse()
        .map_err(|_| format!("'{}' is not a valid quality (0-100)", input))?;
    //                                                                    ^ the ? operator

    if num > 100 {
        return Err(format!("quality {} is out of range (0-100)", num));
    }

    Ok(num)
}

// Using it:
fn main() {
    match parse_quality("85") {
        Ok(q)    => println!("Quality: {}", q),
        Err(msg) => println!("Error: {}", msg),
    }
}
```

TS equivalent:
```typescript
function parseQuality(input: string): number {
    const num = parseInt(input);
    if (isNaN(num)) throw new Error(`'${input}' is not a valid quality`);
    if (num > 100) throw new Error(`quality ${num} is out of range`);
    return num;
}

try {
    const q = parseQuality("85");
} catch (err) {
    console.error(err.message);
}
```

### The `?` Operator — Your Best Friend

The `?` at the end of an expression means: "if this is `Err`, return the error immediately. If it's `Ok`, unwrap the value and continue."

```rust
// Without ? — verbose, tedious
fn process(data: &[u8]) -> Result<Vec<u8>, Error> {
    let img = match image::load_from_memory(data) {
        Ok(img) => img,
        Err(e) => return Err(e.into()),
    };
    let resized = match resize(&img) {
        Ok(r) => r,
        Err(e) => return Err(e.into()),
    };
    Ok(resized)
}

// With ? — clean, reads top-to-bottom (like Go's if err != nil, but shorter)
fn process(data: &[u8]) -> Result<Vec<u8>, Error> {
    let img = image::load_from_memory(data)?;
    let resized = resize(&img)?;
    Ok(resized)
}
```

Compare to Go:
```go
func process(data []byte) ([]byte, error) {
    img, err := loadFromMemory(data)
    if err != nil { return nil, err }
    resized, err := resize(img)
    if err != nil { return nil, err }
    return resized, nil
}
```

**`?` is Go's `if err != nil { return err }` compressed into a single character.**

### `unwrap()` — The "I Promise This Won't Fail" Escape Hatch

```rust
let num: u8 = "85".parse().unwrap();    // panics (crashes) if it fails
```

**`unwrap()` is like `!` in TypeScript** — "trust me, this isn't null." Use it in tests and prototypes. Never in production WASM code — a panic in WASM crashes the entire Web Worker.

---

## Types: Rust vs TypeScript vs Go

| Concept | TypeScript | Go | Rust |
|---|---|---|---|
| **String (owned)** | `string` | `string` | `String` |
| **String (borrowed)** | `string` | `string` | `&str` |
| **Array (owned)** | `number[]` | `[]int` | `Vec<i32>` |
| **Array (borrowed)** | `readonly number[]` | `[]int` | `&[i32]` |
| **Byte array** | `Uint8Array` | `[]byte` | `Vec<u8>` / `&[u8]` |
| **Object/struct** | `interface Foo {}` | `type Foo struct {}` | `struct Foo {}` |
| **Enum** | `enum` or union types | `const` + `iota` | `enum` (much more powerful) |
| **Nullable** | `T \| null` | pointer (`*T`) | `Option<T>` |
| **Error result** | `throw` / `Promise` | `(T, error)` | `Result<T, E>` |
| **Void** | `void` | (no return) | `()` (unit type) |
| **Any** | `any` | `interface{}` | Doesn't exist. Use generics or trait objects |
| **Map** | `Map<K, V>` | `map[K]V` | `HashMap<K, V>` |

### Number Types (Rust Is Specific)

TS has `number` for everything. Rust makes you choose:

| Rust type | What it is | Range | When to use |
|---|---|---|---|
| `u8` | Unsigned 8-bit | 0 to 255 | Byte data, quality percentages |
| `u32` | Unsigned 32-bit | 0 to ~4 billion | Image dimensions, counts |
| `u64` | Unsigned 64-bit | 0 to very big | File sizes |
| `i32` | Signed 32-bit | -2B to +2B | General integers |
| `f32` | 32-bit float | Decimal | Ratios, scaling factors |
| `f64` | 64-bit float | Decimal (precise) | Math-heavy operations |
| `usize` | Pointer-sized | Platform-dependent | Array indexing, lengths |

**Rule of thumb:** `u8` for bytes, `u32` for dimensions/counts, `f32` for ratios, `usize` for lengths/indexes.

---

## Enums — Way More Powerful Than TS Enums

Rust enums can carry data. They're more like TypeScript discriminated unions.

```rust
// Rust enum with data — like a TS discriminated union
enum ImageFormat {
    Jpeg { quality: u8 },           // variant with named fields
    Png { compression: u8 },
    WebP { lossless: bool },
}

// Using it with match (exhaustive — compiler forces you to handle every case)
fn extension(format: &ImageFormat) -> &str {
    match format {
        ImageFormat::Jpeg { .. } => "jpg",
        ImageFormat::Png { .. } => "png",
        ImageFormat::WebP { .. } => "webp",
    }
}
```

TS equivalent:
```typescript
type ImageFormat =
    | { type: "jpeg"; quality: number }
    | { type: "png"; compression: number }
    | { type: "webp"; lossless: boolean };

function extension(format: ImageFormat): string {
    switch (format.type) {
        case "jpeg": return "jpg";
        case "png":  return "png";
        case "webp": return "webp";
    }
}
```

### Option and Result — The Two Enums You'll Use Constantly

```rust
// Option<T> = T | null in TypeScript
enum Option<T> {
    Some(T),    // has a value
    None,       // no value (null)
}

// Result<T, E> = success or failure
enum Result<T, E> {
    Ok(T),      // success
    Err(E),     // failure
}
```

```rust
// Using Option (like nullable)
fn find_user(id: u32) -> Option<User> {
    if id == 0 { None } else { Some(User { id }) }
}

// Handling it
match find_user(42) {
    Some(user) => println!("Found: {}", user.id),
    None       => println!("Not found"),
}

// Or with if-let (like optional chaining)
if let Some(user) = find_user(42) {
    println!("Found: {}", user.id);
}
```

---

## Traits — Interfaces, but Implicit (Like Go)

Traits are Rust's version of interfaces. Like Go, they're implemented implicitly — but with a bit more ceremony.

```rust
// Define a trait (like a TS interface)
trait Node {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, String>;
    fn name(&self) -> &str;
}

// Implement it for a type (you DO write `impl Trait for Type`, unlike Go)
struct CompressImage;

impl Node for CompressImage {
    fn execute(&self, input: &[u8]) -> Result<Vec<u8>, String> {
        // compress logic...
        Ok(vec![])
    }

    fn name(&self) -> &str {
        "compress-image"
    }
}
```

TS equivalent:
```typescript
interface Node {
    execute(input: Uint8Array): Uint8Array;
    name(): string;
}

class CompressImage implements Node {
    execute(input: Uint8Array): Uint8Array { ... }
    name(): string { return "compress-image"; }
}
```

Go equivalent:
```go
type Node interface {
    Execute(ctx context.Context, input []byte) ([]byte, error)
    Name() string
}

// Go: just have the methods, no "implements" keyword
// Rust: you write `impl Node for CompressImage` explicitly
```

---

## The Five Patterns You'll See Everywhere

### 1. Constructor + Methods (the "class" pattern)

```rust
pub struct Engine {
    quality: u8,
}

impl Engine {
    // Constructor — `new()` by convention (like Go's `New()`)
    pub fn new(quality: u8) -> Self {
        Self { quality }      // Self = the type we're implementing for
    }

    // Method — &self is "this" (borrowed, read-only)
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // self.quality is accessible here
        Ok(vec![])
    }

    // Mutable method — &mut self (can modify self)
    pub fn set_quality(&mut self, quality: u8) {
        self.quality = quality;
    }
}
```

TS equivalent:
```typescript
class Engine {
    constructor(private quality: number) {}

    compress(data: Uint8Array): Uint8Array { ... }

    setQuality(quality: number) {
        this.quality = quality;
    }
}
```

### 2. The `?` Error Chain (replaces try/catch)

```rust
pub fn process_image(data: &[u8], quality: u8) -> Result<Vec<u8>, String> {
    let img = load_image(data)?;          // fail? return error
    let resized = resize(&img, 800)?;     // fail? return error
    let compressed = compress(&resized, quality)?;  // fail? return error
    Ok(compressed)                        // all good, return success
}
```

TS equivalent:
```typescript
async function processImage(data: Uint8Array, quality: number): Promise<Uint8Array> {
    const img = await loadImage(data);        // throw on fail
    const resized = await resize(img, 800);   // throw on fail
    const compressed = await compress(resized, quality);
    return compressed;
}
```

### 3. Pattern Matching (replaces switch/if-else chains)

```rust
match format {
    "jpg" | "jpeg" => compress_jpeg(data, quality),
    "png"          => compress_png(data),
    "webp"         => compress_webp(data, quality),
    _              => Err(format!("unsupported format: {}", format)),
    //  ^ underscore = default case (like _ in Go switch)
}
```

TS equivalent:
```typescript
switch (format) {
    case "jpg": case "jpeg": return compressJpeg(data, quality);
    case "png":              return compressPng(data);
    case "webp":             return compressWebp(data, quality);
    default:                 throw new Error(`unsupported format: ${format}`);
}
```

**Match is exhaustive** — the compiler forces you to handle every possible case. This is one of Rust's best features. You can never forget a case.

### 4. Iterators + Closures (like .map/.filter/.reduce)

```rust
let sizes: Vec<u32> = files
    .iter()                          // like files[Symbol.iterator]()
    .filter(|f| f.size > 0)         // |f| is a closure (arrow function)
    .map(|f| f.size)                // transform each element
    .collect();                      // collect into a Vec (must be explicit)
```

TS equivalent:
```typescript
const sizes: number[] = files
    .filter(f => f.size > 0)
    .map(f => f.size);
    // no .collect() needed — JS arrays are eager
```

**Key difference:** Rust iterators are lazy (like generators). Nothing executes until `.collect()`, `.for_each()`, or another terminal method is called. TS array methods are eager — `.filter()` immediately produces a new array.

### 5. The WASM Boundary (Rust ↔ JavaScript)

```rust
use wasm_bindgen::prelude::*;

// #[wasm_bindgen] makes this function callable from JS
#[wasm_bindgen]
pub fn compress_image(data: &[u8], quality: u8) -> Result<Vec<u8>, JsValue> {
    //                      ^^^^                          ^^^^^^
    //                      JS passes Uint8Array          Rust returns Uint8Array
    //                      Rust sees it as &[u8]         JS gets it as Uint8Array

    let result = internal_compress(data, quality)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    //           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //           Convert Rust errors to JS-readable errors

    Ok(result)
}
```

JS side:
```typescript
import init, { compress_image } from "../engine/pkg";

await init();  // load the .wasm file (one-time)

const input = new Uint8Array(fileBuffer);
const output = compress_image(input, 85);  // calls Rust, gets Uint8Array back
```

**What crosses the WASM boundary:**

| Type in Rust | Type in JS | Notes |
|---|---|---|
| `&[u8]` / `Vec<u8>` | `Uint8Array` | Zero-copy for borrows, copy for owned |
| `String` / `&str` | `string` | Always copied (UTF-8 ↔ UTF-16) |
| `bool` | `boolean` | Trivial |
| `u8`, `u32`, `f64` etc. | `number` | Trivial |
| `Result<T, JsValue>` | Return or throw | `Err` becomes a JS exception |
| `JsValue` | `any` | Escape hatch for complex JS objects |

---

## Symbols Cheat Sheet

| Symbol | Meaning | TS Equivalent |
|---|---|---|
| `&` | Borrow (reference) | Passing an object (JS always borrows) |
| `&mut` | Mutable borrow | Normal JS reference (can modify) |
| `*` | Dereference (follow reference to value) | Not needed in JS |
| `::` | Path separator / associated function | `.` (like `Array.from()`) |
| `.` | Method call / field access | `.` (same as JS) |
| `?` | Propagate error (early return on Err) | `await` (kind of) + `throw` |
| `!` suffix | Macro invocation | No equivalent — think "special function" |
| `'a` | Lifetime annotation | No equivalent — tells compiler how long borrows live |
| `<T>` | Generic type parameter | `<T>` (same concept) |
| `_` | Ignore/discard | `_` in destructuring |
| `..` | Range OR struct spread | `...` (spread) |
| `=>` | Match arm result | `:` in switch case (sort of) |
| `\|x\|` | Closure parameter | `(x) =>` arrow function |
| `::<Type>` | Turbofish (explicit generic) | `fn<Type>()` |

---

## Mutability: Opt-In, Not Opt-Out

In JS, everything is mutable by default. In Rust, everything is **immutable** by default.

```rust
let x = 5;          // immutable — like: const x = 5
// x = 10;          // COMPILE ERROR

let mut y = 5;      // mutable — like: let y = 5
y = 10;             // OK

let mut vec = Vec::new();   // must be mut to push to it
vec.push(1);
vec.push(2);
```

**Why?** Immutability by default prevents accidental mutation. Combined with the borrow checker, it means data races are impossible. The compiler guarantees thread safety — which is why Rust WASM in Web Workers is rock solid.

---

## Strings: The Two Types You Need to Know

| Type | Ownership | TS Equivalent | When to Use |
|---|---|---|---|
| `String` | Owned (heap-allocated, growable) | `string` (that you built) | Return values, struct fields |
| `&str` | Borrowed (slice/view into a String) | `string` (that you received) | Function parameters, string literals |

```rust
let owned: String = String::from("hello");     // you own this
let borrowed: &str = "hello";                   // string literal — always &str
let also_borrowed: &str = &owned;               // borrow the owned string

fn greet(name: &str) {           // accept borrowed — most flexible
    println!("Hello, {}", name);
}

greet("world");                  // &str literal — works
greet(&owned);                   // borrow a String — works
```

**Rule of thumb:** Accept `&str` in function parameters. Return `String` from functions. Store `String` in structs.

---

## Closures vs Functions

```rust
// Named function
fn add(a: i32, b: i32) -> i32 { a + b }

// Closure (anonymous function) — pipes instead of parentheses for params
let add = |a: i32, b: i32| -> i32 { a + b };

// Short closure (type inference, single expression = no braces needed)
let double = |x| x * 2;

// Closures capture their environment (like JS arrow functions)
let factor = 3;
let multiply = |x| x * factor;    // captures `factor` from surrounding scope
```

TS equivalent:
```typescript
function add(a: number, b: number): number { return a + b; }
const add = (a: number, b: number): number => a + b;
const double = (x: number) => x * 2;
const factor = 3;
const multiply = (x: number) => x * factor;
```

---

## Modules: How Files Connect

Every `.rs` file is a module. You wire them together with `mod` and `use`.

```rust
// src/lib.rs — the crate root (like index.ts)
mod image;     // tells Rust: "there's a module called image"
               // looks for src/image.rs OR src/image/mod.rs
mod csv;

pub use image::compress_image;    // re-export (like: export { compressImage } from "./image")
pub use csv::clean_csv;
```

```rust
// src/image/mod.rs — the image module root (like image/index.ts)
mod compress;     // src/image/compress.rs
mod resize;       // src/image/resize.rs

pub use compress::compress_image;   // re-export public API
pub use resize::resize_image;
```

```rust
// src/image/compress.rs — a leaf module
use image::DynamicImage;              // external crate
use super::resize::resize_image;      // sibling module (super = parent)

pub fn compress_image(data: &[u8]) -> Result<Vec<u8>, String> {
    // ...
}
```

TS equivalent of the module tree:
```typescript
// src/index.ts
export { compressImage } from "./image";
export { cleanCsv } from "./csv";

// src/image/index.ts
export { compressImage } from "./compress";
export { resizeImage } from "./resize";
```

---

## Testing

```rust
// Tests live in the same file (like co-located test blocks)
#[cfg(test)]                    // only compiled when running tests
mod tests {
    use super::*;               // import everything from parent module

    #[test]                     // marks this as a test function
    fn test_compress_quality() {
        let input = include_bytes!("../fixtures/test.jpg");  // embed file at compile time
        let result = compress(input, 85).unwrap();           // unwrap OK in tests
        assert!(!result.is_empty());                          // assert! macro
        assert_eq!(result.len() < input.len(), true);        // assert_eq! for equality
    }

    #[test]
    fn test_invalid_input() {
        let result = compress(b"not an image", 85);
        assert!(result.is_err());                            // should fail
    }
}
```

Run:
```bash
cargo test                     # run all tests
cargo test compress            # run tests with "compress" in the name
cargo test -- --nocapture      # show println! output during tests
```

---

## Quick Comparison: Same Logic in TS, Go, Rust

**Task:** Compress a list of images, skip failures, return successes.

### TypeScript
```typescript
function compressAll(images: Uint8Array[], quality: number): Uint8Array[] {
    return images
        .map(img => compress(img, quality))
        .filter((result): result is Uint8Array => result !== null);
}
```

### Go
```go
func CompressAll(images [][]byte, quality int) [][]byte {
    var results [][]byte
    for _, img := range images {
        result, err := Compress(img, quality)
        if err != nil {
            continue
        }
        results = append(results, result)
    }
    return results
}
```

### Rust
```rust
fn compress_all(images: &[Vec<u8>], quality: u8) -> Vec<Vec<u8>> {
    images
        .iter()
        .filter_map(|img| compress(img, quality).ok())  // ok() converts Result to Option
        .collect()
}
```

---

## Common Rust-isms Explained

| You'll see this | What it means | TS comparison |
|---|---|---|
| `let x = 5;` | Immutable binding | `const x = 5;` |
| `let mut x = 5;` | Mutable binding | `let x = 5;` |
| `vec![1, 2, 3]` | Create a Vec (array) | `[1, 2, 3]` |
| `format!("hi {}", name)` | String interpolation | `` `hi ${name}` `` |
| `println!("debug: {:?}", val)` | Debug print | `console.log("debug:", val)` |
| `.clone()` | Deep copy | `structuredClone(obj)` |
| `.to_string()` | Convert to String | `.toString()` |
| `.into()` | Type conversion (inferred) | Implicit coercion |
| `.as_ref()` | Borrow from owned type | No equivalent (JS always borrows) |
| `impl From<A> for B` | Conversion trait | No equivalent (custom `fromA()` methods) |
| `#[derive(Debug, Clone)]` | Auto-generate trait impls | No equivalent (built-in in JS) |
| `todo!()` | Placeholder (compiles, panics at runtime) | `throw new Error("TODO")` |
| `unimplemented!()` | Same as todo! but for known-missing code | Same |

---

## The Cargo.toml Anatomy

```toml
[package]
name = "bnto-engine"
version = "0.1.0"
edition = "2021"               # Rust edition (like TS target version)

[lib]
crate-type = ["cdylib"]        # Required for wasm-pack — produces .wasm

[dependencies]
image = "0.25"                 # like: "image": "^0.25" in package.json
csv = "1.3"
serde = { version = "1", features = ["derive"] }   # with optional features enabled
wasm-bindgen = "0.2"

[dev-dependencies]             # like devDependencies
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "s"                # optimize for small .wasm file size
lto = true                     # link-time optimization
```

---

## Further Reading

- [The Rust Book](https://doc.rust-lang.org/book/) — the official guide, very well-written
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) — practical patterns with runnable code
- [Rustlings](https://github.com/rust-lang/rustlings) — small exercises to learn Rust step by step
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/) — Rust↔JS bridge reference
- [wasm-pack Docs](https://rustwasm.github.io/docs/wasm-pack/) — build tool for Rust WASM
