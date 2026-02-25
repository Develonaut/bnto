package testgolden

import (
	"image"
	_ "image/jpeg"
	_ "image/png"
	"os"
	"testing"

	_ "golang.org/x/image/webp"
)

// ImageProperties defines expected properties for image verification.
type ImageProperties struct {
	Width    int    // Expected width (0 to skip check)
	Height   int    // Expected height (0 to skip check)
	Format   string // Expected format: "png", "jpeg", "webp" (empty to skip)
	MinBytes int64  // Minimum file size in bytes (0 to skip)
	MaxBytes int64  // Maximum file size in bytes (0 to skip)
}

// AssertImageProperties verifies an image file matches expected properties.
//
// Uses image.DecodeConfig for dimensions/format and magic bytes for format
// verification. Does not fully decode the image — fast even for large files.
func AssertImageProperties(t *testing.T, path string, expected ImageProperties) {
	t.Helper()

	// Verify file exists and check size
	info, err := os.Stat(path)
	if err != nil {
		t.Fatalf("Image file not found: %s: %v", path, err)
	}

	if expected.MinBytes > 0 && info.Size() < expected.MinBytes {
		t.Errorf("Image too small: %s is %d bytes, want >= %d", path, info.Size(), expected.MinBytes)
	}
	if expected.MaxBytes > 0 && info.Size() > expected.MaxBytes {
		t.Errorf("Image too large: %s is %d bytes, want <= %d", path, info.Size(), expected.MaxBytes)
	}

	// Verify format via magic bytes
	if expected.Format != "" {
		assertMagicBytes(t, path, expected.Format)
	}

	// Verify dimensions via image.DecodeConfig
	if expected.Width > 0 || expected.Height > 0 {
		f, err := os.Open(path)
		if err != nil {
			t.Fatalf("Failed to open image %s: %v", path, err)
		}
		defer f.Close()

		config, _, err := image.DecodeConfig(f)
		if err != nil {
			t.Fatalf("Failed to decode image config %s: %v", path, err)
		}

		if expected.Width > 0 && config.Width != expected.Width {
			t.Errorf("Image width: %s is %d, want %d", path, config.Width, expected.Width)
		}
		if expected.Height > 0 && config.Height != expected.Height {
			t.Errorf("Image height: %s is %d, want %d", path, config.Height, expected.Height)
		}
	}
}

// AssertImageSmaller verifies that the output image is smaller than the input.
func AssertImageSmaller(t *testing.T, inputPath, outputPath string) {
	t.Helper()

	inputInfo, err := os.Stat(inputPath)
	if err != nil {
		t.Fatalf("Input image not found: %s: %v", inputPath, err)
	}

	outputInfo, err := os.Stat(outputPath)
	if err != nil {
		t.Fatalf("Output image not found: %s: %v", outputPath, err)
	}

	if outputInfo.Size() >= inputInfo.Size() {
		t.Errorf("Output not smaller: input=%d bytes, output=%d bytes", inputInfo.Size(), outputInfo.Size())
	}
}

// magicBytes maps format names to their file signature prefixes.
var magicBytes = map[string][]byte{
	"png":  {0x89, 0x50, 0x4E, 0x47}, // \x89PNG
	"jpeg": {0xFF, 0xD8, 0xFF},        // JPEG SOI marker
	"webp": {0x52, 0x49, 0x46, 0x46},  // RIFF (WebP container)
}

func assertMagicBytes(t *testing.T, path, format string) {
	t.Helper()

	expected, ok := magicBytes[format]
	if !ok {
		t.Fatalf("Unknown image format for magic bytes check: %s", format)
	}

	f, err := os.Open(path)
	if err != nil {
		t.Fatalf("Failed to open %s for magic bytes: %v", path, err)
	}
	defer f.Close()

	header := make([]byte, len(expected))
	n, err := f.Read(header)
	if err != nil || n < len(expected) {
		t.Fatalf("Failed to read header from %s: read %d bytes, err=%v", path, n, err)
	}

	for i, b := range expected {
		if header[i] != b {
			t.Errorf("Magic bytes mismatch for %s: expected %s format, got header %x", path, format, header)
			return
		}
	}

	// Extra check for WebP: bytes 8-11 should be "WEBP"
	if format == "webp" {
		extra := make([]byte, 8)
		n, err := f.Read(extra)
		if err != nil || n < 8 {
			t.Fatalf("Failed to read WebP header from %s", path)
		}
		if string(extra[4:8]) != "WEBP" {
			t.Errorf("RIFF container is not WebP: %s, got %q", path, string(extra[4:8]))
		}
	}
}
