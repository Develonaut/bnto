/**
 * Download a Blob as a file in the browser.
 *
 * Creates a temporary object URL, triggers a download via a hidden
 * anchor element, then revokes the URL to free memory.
 *
 * Used for browser execution results — files processed via WASM
 * live in memory as Blobs, not on R2.
 */
export function downloadBlob(blob: Blob, filename: string): void {
  const url = URL.createObjectURL(blob);
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = filename;
  anchor.style.display = "none";
  document.body.appendChild(anchor);
  anchor.click();
  document.body.removeChild(anchor);
  URL.revokeObjectURL(url);
}
