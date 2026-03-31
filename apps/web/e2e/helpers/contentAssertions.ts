/**
 * Content verification assertions — EXIF stripping and byte-identical preservation.
 */

import fs from "fs";
import { expect } from "../fixtures";

/**
 * Assert that a JPEG buffer has no EXIF APP1 marker (FF E1).
 */
export function assertExifStripped(data: Buffer) {
  for (let i = 0; i < data.length - 1; i++) {
    if (data[i] === 0xff && data[i + 1] === 0xe1) {
      throw new Error(`EXIF APP1 marker (FF E1) found at offset ${i} — EXIF not stripped`);
    }
  }
}

/**
 * Assert output bytes are identical to the input file (content preservation).
 */
export function assertContentPreserved(inputPath: string, outputBuffer: Buffer) {
  const inputBuffer = fs.readFileSync(inputPath);
  expect(outputBuffer.length).toBe(inputBuffer.length);
  expect(outputBuffer.equals(inputBuffer)).toBe(true);
}
