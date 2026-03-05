/**
 * Trigger a browser file download from a JSON blob.
 *
 * Pure DOM side effect — isolated from hooks so the export hook
 * stays testable without browser APIs.
 */

function triggerDownload(filename: string, json: string): void {
  const blob = new Blob([json], { type: "application/json" });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}

export { triggerDownload };
