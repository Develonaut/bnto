package image

import (
	"bytes"
	"fmt"
	stdimage "image"
	"image/draw"
	"image/jpeg"
	"image/png"
	"os"
	"path/filepath"
	"strings"

	"github.com/kolesa-team/go-webp/encoder"
	"github.com/kolesa-team/go-webp/webp"
)

// exportImage exports image to file in the specified format.
func (i *Image) exportImage(img stdimage.Image, output string, params map[string]interface{}) error {
	quality := getIntParam(params, "quality", 80)
	format := determineFormat(output, params)

	// Check .bntoignore in the target directory
	dir := filepath.Dir(output)
	bntoIgnore, err := loadBntoIgnore(dir)
	if err != nil {
		// If we can't load .bntoignore, log warning but continue
		fmt.Fprintf(os.Stderr, "Warning: failed to load .bntoignore: %v\n", err)
	} else if bntoIgnore.shouldIgnore(output) {
		return fmt.Errorf("file %s is protected by .bntoignore and cannot be overwritten", output)
	}

	data, err := encodeImage(img, format, quality)
	if err != nil {
		return err
	}

	if err := os.WriteFile(output, data, 0644); err != nil {
		return fmt.Errorf("failed to write output file: %w", err)
	}

	return nil
}

// determineFormat determines output format from params or file extension.
func determineFormat(output string, params map[string]interface{}) string {
	format := getStringParam(params, "_targetFormat", "")
	if format != "" {
		return format
	}

	ext := strings.ToLower(filepath.Ext(output))
	switch ext {
	case ".webp":
		return "webp"
	case ".jpg", ".jpeg":
		return "jpeg"
	case ".png":
		return "png"
	default:
		return "webp"
	}
}

// encodeImage encodes image in the specified format.
func encodeImage(img stdimage.Image, format string, quality int) ([]byte, error) {
	var buf bytes.Buffer
	var err error

	switch format {
	case "webp":
		err = exportWebp(&buf, img, quality)
	case "jpeg":
		err = exportJpeg(&buf, img, quality)
	case "png":
		err = exportPng(&buf, img)
	default:
		return nil, fmt.Errorf("unsupported format: %s", format)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to export image: %w", err)
	}

	return buf.Bytes(), nil
}

// webpMaxQuality is the maximum quality for WebP encoding.
// WebP quality above ~85 often produces larger files than source JPEGs because
// it tries to preserve more detail than existed in the already-compressed source.
// WebP Q85 is visually equivalent to JPEG Q95+ for most images.
const webpMaxQuality = 85

// exportWebp exports image as WebP using kolesa-team/go-webp (libwebp wrapper).
func exportWebp(buf *bytes.Buffer, img stdimage.Image, quality int) error {
	bounds := img.Bounds()

	// Cap quality at webpMaxQuality to prevent output larger than source
	if quality > webpMaxQuality {
		quality = webpMaxQuality
	}

	// Convert to opaque RGB to avoid encoding unnecessary alpha channel.
	// When imaging.Open() reads a JPEG, it returns NRGBA with all alpha=255.
	// Without this conversion, WebP encodes the alpha channel, increasing file size.
	opaqueImg := toOpaqueRGB(img)

	// Create encoder options optimized for photos with higher compression effort
	options, err := encoder.NewLossyEncoderOptions(encoder.PresetPhoto, float32(quality))
	if err != nil {
		return fmt.Errorf("failed to create webp options: %w", err)
	}

	// Method 6 = slowest but best compression
	options.Method = 6
	options.UseSharpYuv = true

	// Encode the opaque image
	if err := webp.Encode(buf, opaqueImg, options); err != nil {
		return fmt.Errorf("webp encode failed (dimensions: %dx%d, quality: %d): %w",
			bounds.Dx(), bounds.Dy(), quality, err)
	}
	return nil
}

// toOpaqueRGB converts an image to an opaque RGB format by compositing onto white.
// This removes any alpha channel data, reducing WebP file size for photos.
func toOpaqueRGB(img stdimage.Image) *stdimage.RGBA {
	bounds := img.Bounds()
	rgba := stdimage.NewRGBA(bounds)

	// Fill with white background using Uniform (efficient)
	draw.Draw(rgba, bounds, stdimage.White, stdimage.Point{}, draw.Src)

	// Draw source image over white background (composites alpha)
	draw.Draw(rgba, bounds, img, bounds.Min, draw.Over)

	return rgba
}

// exportJpeg exports image as JPEG using standard library.
func exportJpeg(buf *bytes.Buffer, img stdimage.Image, quality int) error {
	options := &jpeg.Options{
		Quality: quality,
	}
	return jpeg.Encode(buf, img, options)
}

// exportPng exports image as PNG using standard library.
func exportPng(buf *bytes.Buffer, img stdimage.Image) error {
	encoder := &png.Encoder{
		CompressionLevel: png.DefaultCompression,
	}
	return encoder.Encode(buf, img)
}
