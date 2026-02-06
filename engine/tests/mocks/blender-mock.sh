#!/bin/bash
# Mock Blender script that simulates rendering 8 product photos
# Outputs progress like real Blender and creates PNG files
#
# Usage:
#   blender-mock.sh -- --sku PROD-001 --overlay overlay.png --output ./render
#
# This mock:
# - Parses arguments the same way as real Blender Python script
# - Outputs progress matching real Blender format
# - Creates 8 PNG files (render-1.png through render-8.png) using test fixture
# - Uses tests/fixtures/images/Product_Render.png as base template (600x600)
# - Creates variations by rotating/annotating the base image
# - Completes in ~2 seconds instead of 5+ minutes

# Find the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BASE_IMAGE="$SCRIPT_DIR/../fixtures/images/Product_Render.png"

# Verify base image exists
if [[ ! -f "$BASE_IMAGE" ]]; then
    echo "Error: Base image not found: $BASE_IMAGE"
    echo "Please ensure tests/fixtures/images/Product_Render.png exists"
    exit 1
fi

SKU=""
OVERLAY=""
OUTPUT=""

# Parse arguments (after "--")
while [[ $# -gt 0 ]]; do
    case $1 in
        --sku)
            SKU="$2"
            shift 2
            ;;
        --overlay)
            OVERLAY="$2"
            shift 2
            ;;
        --output)
            OUTPUT="$2"
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

# Validate required arguments
if [[ -z "$SKU" || -z "$OVERLAY" || -z "$OUTPUT" ]]; then
    echo "Error: Missing required arguments"
    echo "Usage: blender-mock.sh -- --sku SKU --overlay OVERLAY --output OUTPUT"
    exit 1
fi

# Output Blender-like header
echo "Blender 3.6.0"
echo "Rendering product: $SKU"
echo "Overlay: $OVERLAY"
echo "Output: $OUTPUT"
echo ""

# Render 8 angles (simulate 0.2s per frame)
for i in {1..8}; do
    # Output progress like real Blender
    echo "Fra:$i Mem:12.00M (Peak 12.00M) | Rendering $i/8"

    # Create render by rotating/annotating the base Product_Render.png
    # Each angle gets a small rotation and annotation to show variation
    ROTATION=$(( (i - 1) * 45 ))  # 0°, 45°, 90°, 135°, 180°, 225°, 270°, 315°

    # Use ImageMagick to create variation from base image
    # - Rotate the image to simulate different camera angle
    # - Add text annotation showing SKU and angle number
    # - Maintain original quality and alpha channel
    magick "$BASE_IMAGE" \
        -rotate $ROTATION \
        -gravity southeast \
        -pointsize 60 \
        -fill white \
        -stroke black \
        -strokewidth 2 \
        -annotate +20+20 "$SKU\nAngle $i" \
        "${OUTPUT}-${i}.png"

    # Simulate render time
    sleep 0.2
done

echo ""
echo "✓ Rendered 8 photos for $SKU"
exit 0
