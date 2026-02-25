package image

import (
	"context"
	"fmt"
	"path/filepath"
	"strings"
)

// batch processes multiple images.
func (i *Image) batch(ctx context.Context, params map[string]interface{}) (interface{}, error) {
	inputs, err := parseInputs(params)
	if err != nil {
		return nil, err
	}

	format := getStringParam(params, "format", "webp")
	quality := getIntParam(params, "quality", 80)

	outputs := make([]string, 0, len(inputs))
	processed := 0

	for _, input := range inputs {
		// Check context cancellation
		if err := ctx.Err(); err != nil {
			return nil, err
		}

		output := generateOutputPath(input, format)

		// Convert image
		convertParams := map[string]interface{}{
			"input":   input,
			"output":  output,
			"format":  format,
			"quality": quality,
		}

		if _, err := i.convert(ctx, convertParams); err != nil {
			// Log error but continue with other images
			continue
		}

		outputs = append(outputs, output)
		processed++
	}

	return map[string]interface{}{
		"processed": processed,
		"outputs":   outputs,
	}, nil
}

// parseInputs extracts and validates the inputs parameter.
func parseInputs(params map[string]interface{}) ([]string, error) {
	inputs, ok := params["inputs"].([]string)
	if ok {
		return inputs, nil
	}

	// Try []interface{} conversion
	inputsInterface, ok := params["inputs"].([]interface{})
	if !ok {
		return nil, fmt.Errorf("inputs parameter is required and must be an array of strings")
	}

	inputs = make([]string, len(inputsInterface))
	for idx, item := range inputsInterface {
		str, ok := item.(string)
		if !ok {
			return nil, fmt.Errorf("inputs must be array of strings")
		}
		inputs[idx] = str
	}

	return inputs, nil
}

// generateOutputPath creates output path by replacing extension.
func generateOutputPath(input, format string) string {
	ext := filepath.Ext(input)
	return strings.TrimSuffix(input, ext) + "." + format
}
