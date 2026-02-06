// Package image provides image processing operations using pure Go libraries.
//
// The image neta supports:
//   - Resize (with aspect ratio preservation)
//   - Format conversion (PNG, JPEG, WebP)
//   - Optimization (quality settings)
//   - Batch processing
//
// CRITICAL FOR PHASE 8: Used to optimize Blender PNG outputs to WebP.
//
// Pure Go implementation using:
// - disintegration/imaging: Image resizing and format conversion
// - gen2brain/webp: WebP encoding (no CGO required)
//
// This provides true cross-platform portability with no C dependencies,
// making `go install` work seamlessly on Windows, macOS, and Linux.
//
// Performance is acceptable for CLI use (~200-250ms per 1920x1080 PNG → WebP)
// while maintaining the portability promise in README.
//
// Example WebP optimization:
//
//	params := map[string]interface{}{
//	    "operation": "convert",
//	    "input": "render.png",
//	    "output": "render.webp",
//	    "format": "webp",
//	    "quality": 80,
//	}
//	result, err := imageNeta.Execute(ctx, params)
package image

import (
	"context"
	"fmt"
)

// Image implements the image neta for image processing operations.
type Image struct{}

// New creates a new image neta instance.
func New() *Image {
	return &Image{}
}

// Execute runs image processing operations.
//
// Parameters:
//   - operation: "resize", "convert", "optimize", "batch", or "composite"
//   - input: input file path (or inputs for batch)
//   - output: output file path
//   - format: output format (webp, jpeg, png)
//   - quality: quality setting (1-100, default 80)
//   - width: target width for resize
//   - height: target height for resize (optional if maintainAspect=true)
//   - maintainAspect: preserve aspect ratio (default true)
//   - base: base image path (for composite)
//   - overlay: overlay image path (for composite)
//   - position: overlay position "center" or use x/y offsets (for composite)
//
// Returns:
//   - path: output file path
//   - size: output file size in bytes
//   - dimensions: width and height (for resize)
//   - processed: number of files processed (for batch)
func (i *Image) Execute(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	operation, ok := params["operation"].(string)
	if !ok {
		return nil, fmt.Errorf("operation parameter is required (resize, convert, optimize, composite, or batch)")
	}

	switch operation {
	case "resize":
		return i.resize(ctx, params)
	case "convert":
		return i.convert(ctx, params)
	case "optimize":
		return i.optimize(ctx, params)
	case "composite":
		return i.composite(ctx, params)
	case "batch":
		return i.batch(ctx, params)
	default:
		return nil, fmt.Errorf("invalid operation: %s", operation)
	}
}
