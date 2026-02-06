package image

import (
	"context"
	"fmt"
	stdimage "image"
	"os"

	"github.com/disintegration/imaging"
)

// resize resizes an image.
func (i *Image) resize(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	input, ok := params["input"].(string)
	if !ok {
		return nil, fmt.Errorf("input parameter is required")
	}

	output, ok := params["output"].(string)
	if !ok {
		return nil, fmt.Errorf("output parameter is required")
	}

	width := getIntParam(params, "width", 0)
	height := getIntParam(params, "height", 0)
	scale := getIntParam(params, "scale", 100) // percentage scale (100 = no change)
	maintainAspect := getBoolParam(params, "maintainAspect", true)

	img, err := imaging.Open(input)
	if err != nil {
		return nil, fmt.Errorf("failed to load image: %w", err)
	}

	var resized stdimage.Image
	// If both width and height are 0, keep original dimensions (just convert format)
	if width == 0 && height == 0 {
		resized = img
	} else if maintainAspect {
		resized = imaging.Resize(img, width, height, imaging.Lanczos)
	} else {
		if width == 0 {
			width = img.Bounds().Dx()
		}
		if height == 0 {
			height = img.Bounds().Dy()
		}
		resized = imaging.Resize(img, width, height, imaging.Lanczos)
	}

	// Apply percentage scale if not 100%
	if scale > 0 && scale < 100 {
		currentWidth := resized.Bounds().Dx()
		currentHeight := resized.Bounds().Dy()
		newWidth := currentWidth * scale / 100
		newHeight := currentHeight * scale / 100
		resized = imaging.Resize(resized, newWidth, newHeight, imaging.Lanczos)
	}

	if err := i.exportImage(resized, output, params); err != nil {
		return nil, err
	}

	fileInfo, _ := os.Stat(output)
	var size int64
	if fileInfo != nil {
		size = fileInfo.Size()
	}

	return map[string]interface{}{
		"path": output,
		"size": size,
		"dimensions": map[string]int{
			"width":  resized.Bounds().Dx(),
			"height": resized.Bounds().Dy(),
		},
	}, nil
}
