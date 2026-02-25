/**
 * Creates a test fixture JPEG with EXIF orientation=6 (90° CW rotation).
 *
 * Takes large.jpg (1200x800 landscape) and injects an EXIF APP1 segment
 * with orientation=6. After orientation correction, the image should
 * display as 800x1200 (portrait).
 *
 * This is the most common real-world case: a phone camera held in
 * portrait mode saves a landscape-oriented sensor image with EXIF
 * orientation=6 to indicate it should be displayed rotated 90° CW.
 */
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Read existing non-square fixture (1200x800)
const jpeg = fs.readFileSync(path.join(__dirname, "images/large.jpg"));

// Verify it's a JPEG (starts with FF D8)
if (jpeg[0] !== 0xff || jpeg[1] !== 0xd8) {
  throw new Error("Not a JPEG");
}

// Build EXIF APP1 segment — same structure as Rust test_utils.rs
// Orientation 6 = Rotate 90° CW (portrait from phone camera)
const exifSegment = Buffer.from([
  0xff, 0xe1, // APP1 marker
  0x00, 0x22, // Segment length: 34 bytes
  0x45, 0x78, 0x69, 0x66, 0x00, 0x00, // "Exif\0\0"
  0x4d, 0x4d, // Big-endian (Motorola byte order)
  0x00, 0x2a, // TIFF magic number 42
  0x00, 0x00, 0x00, 0x08, // Offset to IFD0: 8
  0x00, 0x01, // 1 IFD entry
  0x01, 0x12, // Tag: Orientation (0x0112)
  0x00, 0x03, // Type: SHORT (16-bit unsigned)
  0x00, 0x00, 0x00, 0x01, // Count: 1
  0x00, 0x06, // Value: 6 (rotate 90° CW)
  0x00, 0x00, // Padding
  0x00, 0x00, 0x00, 0x00, // Next IFD: none
]);

// Assemble: SOI + EXIF + rest of original JPEG
const result = Buffer.concat([
  jpeg.slice(0, 2), // SOI marker (FF D8)
  exifSegment, // EXIF with orientation=6
  jpeg.slice(2), // Rest of original JPEG data
]);

fs.writeFileSync(path.join(__dirname, "images/portrait-rotated.jpg"), result);

console.log("Created portrait-rotated.jpg");
console.log("  Raw pixel dimensions: 1200x800 (landscape)");
console.log("  EXIF orientation: 6 (rotate 90° CW)");
console.log("  After correction: 800x1200 (portrait)");
console.log("  File size:", result.length, "bytes");
