# Shared Test Fixtures

Test assets shared across all engines and packages: Go engine, Rust WASM engine, TypeScript packages, and Playwright E2E tests.

## Images

Generic landscape and gradient photos from [Unsplash](https://unsplash.com), free to use under the [Unsplash License](https://unsplash.com/license). Converted to multiple formats using ImageMagick and cwebp.

| File | Dimensions | Size | Purpose |
|------|-----------|------|---------|
| `small.{jpg,png,webp}` | 100x100 | 2-17 KB | Fast unit tests |
| `medium.{jpg,png,webp}` | 400x400 | 13-187 KB | Realistic test cases |
| `large.{jpg,png,webp}` | 1200x800 | 85 KB-1 MB | Stress tests, compression ratio verification |
| `gradient.{jpg,png}` | 200x200 | 5-16 KB | Compression-friendly (smooth gradients) |

## Usage

### Rust (compile-time embed)
```rust
const TEST_JPEG: &[u8] = include_bytes!("../../../../test-fixtures/images/small.jpg");
```

### Go (runtime read)
```go
data, err := os.ReadFile("../../test-fixtures/images/small.jpg")
```

### TypeScript (runtime read)
```typescript
import { readFileSync } from "fs";
const data = readFileSync("../../test-fixtures/images/small.jpg");
```

## Regenerating

Requires ImageMagick (`brew install imagemagick`) and cwebp (`brew install webp`):

```bash
# Source: any Unsplash photo URL
SRC_URL="https://images.unsplash.com/photo-1506744038136-46273834b3fb"
OUT="test-fixtures/images"

# Download at different sizes
curl -L -o "$OUT/small.jpg" "${SRC_URL}?w=100&h=100&fit=crop&auto=format&q=80"
curl -L -o "$OUT/medium.jpg" "${SRC_URL}?w=400&h=400&fit=crop&auto=format&q=80"
curl -L -o "$OUT/large.jpg" "${SRC_URL}?w=1200&h=800&fit=crop&auto=format&q=85"

# Convert to PNG
sips -s format png "$OUT/small.jpg" --out "$OUT/small.png"
sips -s format png "$OUT/medium.jpg" --out "$OUT/medium.png"
sips -s format png "$OUT/large.jpg" --out "$OUT/large.png"

# Convert to WebP
magick "$OUT/small.jpg" "$OUT/small.webp"
magick "$OUT/medium.jpg" "$OUT/medium.webp"
magick "$OUT/large.jpg" "$OUT/large.webp"
```
