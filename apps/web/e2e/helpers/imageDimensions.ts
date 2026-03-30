/**
 * Image dimension parsers — extract width/height from JPEG, PNG, and WebP
 * binary data without external dependencies.
 */

/**
 * Extract dimensions from a JPEG by scanning for SOF markers (FF C0/C1/C2).
 */
export function getJpegDimensions(data: Buffer): { width: number; height: number } {
  for (let i = 0; i < data.length - 9; i++) {
    if (
      data[i] === 0xff &&
      (data[i + 1] === 0xc0 || data[i + 1] === 0xc1 || data[i + 1] === 0xc2)
    ) {
      const height = data.readUInt16BE(i + 5);
      const width = data.readUInt16BE(i + 7);
      return { width, height };
    }
  }
  throw new Error("No SOF marker found in JPEG data");
}

/**
 * Extract dimensions from a PNG IHDR chunk (offsets 16 and 20).
 */
export function getPngDimensions(data: Buffer): { width: number; height: number } {
  return { width: data.readUInt32BE(16), height: data.readUInt32BE(20) };
}

/**
 * Extract dimensions from a WebP file.
 *
 * VP8 (lossy): width at offset 26 (little-endian u16, mask 0x3FFF),
 *              height at offset 28.
 * VP8L (lossless): 14-bit width and height packed in a u32 at offset 21.
 * VP8X (extended): 24-bit width at offset 24, height at offset 27.
 */
export function getWebPDimensions(data: Buffer): { width: number; height: number } {
  const chunk = data.toString("ascii", 12, 16);
  if (chunk === "VP8 ") {
    const width = data.readUInt16LE(26) & 0x3fff;
    const height = data.readUInt16LE(28) & 0x3fff;
    return { width, height };
  }
  if (chunk === "VP8L") {
    const bits = data.readUInt32LE(21);
    const width = (bits & 0x3fff) + 1;
    const height = ((bits >> 14) & 0x3fff) + 1;
    return { width, height };
  }
  if (chunk === "VP8X") {
    const width = (data[24] | (data[25] << 8) | (data[26] << 16)) + 1;
    const height = (data[27] | (data[28] << 8) | (data[29] << 16)) + 1;
    return { width, height };
  }
  throw new Error(`Unknown WebP chunk type: ${chunk}`);
}

/**
 * Dispatch to the correct dimension parser based on magic bytes.
 */
export function getImageDimensions(data: Buffer): { width: number; height: number } {
  if (data[0] === 0xff && data[1] === 0xd8) return getJpegDimensions(data);
  if (data[0] === 0x89 && data[1] === 0x50) return getPngDimensions(data);
  if (data.toString("ascii", 0, 4) === "RIFF" && data.toString("ascii", 8, 12) === "WEBP") {
    return getWebPDimensions(data);
  }
  throw new Error("Unknown image format");
}
