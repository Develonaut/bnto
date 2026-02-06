package image

import (
	"context"
	"fmt"
	"image"
	"os"

	"github.com/disintegration/imaging"
)

// composite overlays one image on top of another.
//
// Parameters:
//   - base: path to base image (background)
//   - overlay: path to overlay image (foreground)
//   - output: path for output composite image
//   - x: horizontal offset for overlay (default: 0, centered if position="center")
//   - y: vertical offset for overlay (default: 0, centered if position="center")
//   - position: "center" to center overlay, otherwise uses x/y offsets (default: "center")
//   - format: output format (png, jpeg, webp - default: png)
//   - quality: quality setting for output (1-100, default: 95 for composites)
//
// Returns:
//   - path: output file path
//   - size: output file size in bytes
//   - dimensions: width and height of composite
func (i *Image) composite(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	// Get required parameters
	basePath, ok := params["base"].(string)
	if !ok {
		return nil, fmt.Errorf("base parameter is required (path to base image)")
	}

	overlayPath, ok := params["overlay"].(string)
	if !ok {
		return nil, fmt.Errorf("overlay parameter is required (path to overlay image)")
	}

	output, ok := params["output"].(string)
	if !ok {
		return nil, fmt.Errorf("output parameter is required (path for output image)")
	}

	// Load base image
	baseImg, err := imaging.Open(basePath)
	if err != nil {
		return nil, fmt.Errorf("failed to load base image: %w", err)
	}

	// Load overlay image
	overlayImg, err := imaging.Open(overlayPath)
	if err != nil {
		return nil, fmt.Errorf("failed to load overlay image: %w", err)
	}

	// Get position settings
	position := getStringParam(params, "position", "center")
	x := getIntParam(params, "x", 0)
	y := getIntParam(params, "y", 0)

	// Calculate position if centering
	if position == "center" {
		baseBounds := baseImg.Bounds()
		overlayBounds := overlayImg.Bounds()
		x = (baseBounds.Dx() - overlayBounds.Dx()) / 2
		y = (baseBounds.Dy() - overlayBounds.Dy()) / 2
	}

	// Composite the images
	result := imaging.Overlay(baseImg, overlayImg, image.Pt(x, y), 1.0)

	// Get format and quality settings
	format := getStringParam(params, "format", "png")
	quality := getIntParam(params, "quality", 95) // Higher default quality for composites

	// Export the composite
	exportParams := map[string]interface{}{
		"_targetFormat": format,
		"quality":       quality,
	}
	if err := i.exportImage(result, output, exportParams); err != nil {
		return nil, err
	}

	// Get output file info
	fileInfo, _ := os.Stat(output)
	var size int64
	if fileInfo != nil {
		size = fileInfo.Size()
	}

	bounds := result.Bounds()

	return map[string]interface{}{
		"path": output,
		"size": size,
		"dimensions": map[string]int{
			"width":  bounds.Dx(),
			"height": bounds.Dy(),
		},
		"format": format,
	}, nil
}
