# Test Mocks

This directory contains mock implementations of external services used in Phase 8 integration tests.

## Purpose

Mocks allow us to:
- **Test quickly**: Mock Blender renders complete in ~2s instead of 5+ minutes
- **Test reliably**: No external API rate limits or network issues
- **Test reproducibly**: Same output every time
- **Test edge cases**: Simulate errors and failures easily

## Available Mocks

### 1. Figma API Mock Server (`figma_server.go`)

Mock HTTP server that simulates the Figma API for testing.

**Usage:**

```go
import "github.com/Develonaut/bento/tests/mocks"

func TestMyBento(t *testing.T) {
    server := mocks.NewFigmaServer()
    defer server.Close()

    // Use server.URL in your test
    // Set env var: FIGMA_API_URL=server.URL
}
```

**Behavior:**
- Returns 200 with mock image URL if `X-Figma-Token` header is present
- Returns 401 if token is missing
- Response format matches real Figma API: `{"images": {"node-id": "url"}}`

**Differences from Real API:**
- Doesn't validate file IDs or component IDs
- Always returns the same mock URL
- No rate limiting

### 2. Blender Mock Script (`blender-mock.sh`)

Shell script that simulates Blender rendering without requiring Blender installation.

**Usage:**

```bash
./tests/mocks/blender-mock.sh -- --sku PROD-001 --overlay overlay.png --output ./render
```

This creates:
- `./render-1.png`
- `./render-2.png`
- `./render-3.png`
- `./render-4.png`
- `./render-5.png`
- `./render-6.png`
- `./render-7.png`
- `./render-8.png`

**Output:**
```
Blender 3.6.0
Rendering product: PROD-001
Overlay: overlay.png
Output: ./render

Fra:1 Mem:12.00M (Peak 12.00M) | Rendering 1/8
Fra:2 Mem:12.00M (Peak 12.00M) | Rendering 2/8
...
✓ Rendered 8 photos for PROD-001
```

**Differences from Real Blender:**
- Completes in ~2 seconds instead of 5+ minutes
- Creates test images by rotating and annotating a 600x600 PNG fixture instead of 3D rendering
- Uses the realistic `tests/fixtures/images/Product_Render.png` as base template (600x600)
- Rotates the base image to 8 different angles (0°, 45°, 90°, 135°, 180°, 225°, 270°, 315°)
- Adds text annotation showing SKU and angle number to each render
- Uses ImageMagick `magick` command (requires ImageMagick installation)
- Doesn't actually process the overlay image or do 3D rendering
- No GPU/CPU 3D rendering, just image rotation and annotation
- Produces realistic 600x600 PNG files (~400-500KB each) suitable for testing

## Testing the Mocks

### Test Figma Mock

```bash
go test ./tests/mocks -v -run TestFigmaServer
```

Expected output:
```
=== RUN   TestFigmaServer_ValidRequest
--- PASS: TestFigmaServer_ValidRequest
=== RUN   TestFigmaServer_MissingToken
--- PASS: TestFigmaServer_MissingToken
=== RUN   TestFigmaServer_ResponseFormat
--- PASS: TestFigmaServer_ResponseFormat
PASS
```

### Test Blender Mock

**Prerequisites:**
- ImageMagick must be installed (`brew install imagemagick` on macOS)
- Base image fixture must exist: `tests/fixtures/images/Product_Render.png`

```bash
./tests/mocks/blender-mock.sh -- --sku TEST --overlay test.png --output /tmp/render
ls -lh /tmp/render-*.png
```

Should create 8 PNG files in `/tmp/` (each ~400-500KB, 600x600 pixels).

## Using Mocks in Integration Tests

Example integration test:

```go
package integration_test

import (
    "testing"
    "github.com/Develonaut/bento/tests/mocks"
    "github.com/Develonaut/bento/tests/integration"
)

func TestProductBento(t *testing.T) {
    // Start mock Figma server
    figmaServer := mocks.NewFigmaServer()
    defer figmaServer.Close()

    // Create temp output directory
    tmpDir, cleanup := integration.CreateTempDir(t, "product-test-")
    defer cleanup()

    // Run bento with mocks
    envVars := map[string]string{
        "FIGMA_API_URL": figmaServer.URL,
        "BLENDER_CMD": "./tests/mocks/blender-mock.sh",
        "OUTPUT_DIR": tmpDir,
    }

    output, err := integration.RunBento(t, "product.bento.json", envVars)
    if err != nil {
        t.Fatalf("Bento failed: %v\nOutput: %s", err, output)
    }

    // Verify outputs
    integration.VerifyFileExists(t, tmpDir + "/PROD-001/overlay.png")
    integration.VerifyFileCount(t, tmpDir + "/PROD-001", "*.webp", 8)
}
```

## Mock Design Principles

Following the **Bento Box Principle**:

1. **Single Responsibility**: Each mock does ONE thing (mock Figma OR mock Blender)
2. **Clear Boundaries**: Mocks expose same interface as real services
3. **Composable**: Mocks can be used independently or together
4. **No Grab Bags**: Each mock is in its own file with clear purpose
5. **YAGNI**: Only mock what we actually need for tests

## Next Steps

After Phase 8.1, these mocks will be used in:
- Phase 8.2: CSV Reader bento tests
- Phase 8.3: HTTP Request neta tests (with Figma mock)
- Phase 8.4: Blender integration tests (with Blender mock)
- Phase 8.5: Full end-to-end product automation test

## Maintenance

When updating mocks:
1. Update the mock implementation
2. Update corresponding tests
3. Verify all integration tests still pass
4. Update this README if behavior changes

## Questions?

See `.claude/strategy/phase-8.1-test-infrastructure.md` for full context.
