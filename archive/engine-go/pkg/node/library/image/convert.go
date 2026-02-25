package image

import (
	"context"
	"fmt"
	"os"

	"github.com/disintegration/imaging"
)

// convert converts image format.
func (i *Image) convert(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	input, ok := params["input"].(string)
	if !ok {
		return nil, fmt.Errorf("input parameter is required")
	}

	output, ok := params["output"].(string)
	if !ok {
		return nil, fmt.Errorf("output parameter is required")
	}

	format, ok := params["format"].(string)
	if !ok {
		return nil, fmt.Errorf("format parameter is required (webp, jpeg, png)")
	}

	img, err := imaging.Open(input)
	if err != nil {
		return nil, fmt.Errorf("failed to load image: %w", err)
	}

	params["_targetFormat"] = format
	if err := i.exportImage(img, output, params); err != nil {
		return nil, err
	}

	fileInfo, _ := os.Stat(output)
	var size int64
	if fileInfo != nil {
		size = fileInfo.Size()
	}

	return map[string]interface{}{
		"path":   output,
		"size":   size,
		"format": format,
	}, nil
}

// optimize optimizes image (convert to WebP with quality setting).
func (i *Image) optimize(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	// Optimization is essentially conversion to WebP
	params["format"] = "webp"
	if _, ok := params["quality"]; !ok {
		params["quality"] = 80 // Default quality for optimization
	}
	return i.convert(ctx, params)
}
