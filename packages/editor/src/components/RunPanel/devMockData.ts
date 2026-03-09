/**
 * Mock data for DevTab — realistic BrowserFileResult[] for testing
 * the completed state in the editor's RunPanel.
 *
 * Blob sizes and originalSize values are chosen to produce visible
 * savings percentages in ResultRow's useFileResultProps hook.
 */

import type { BrowserFileResult } from "@bnto/core";

export const MOCK_RESULTS: BrowserFileResult[] = [
  {
    blob: new Blob([new ArrayBuffer(98_000)], { type: "image/jpeg" }),
    filename: "photo-compressed.jpg",
    mimeType: "image/jpeg",
    metadata: { originalSize: 245_000 },
  },
  {
    blob: new Blob([new ArrayBuffer(480_000)], { type: "image/png" }),
    filename: "screenshot-compressed.png",
    mimeType: "image/png",
    metadata: { originalSize: 1_200_000 },
  },
];
