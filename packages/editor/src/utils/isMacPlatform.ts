/**
 * Platform detection helper for keyboard shortcut display.
 */

/**
 * Detect whether the user is on macOS.
 *
 * Uses `navigator.platform` with a `navigator.userAgentData` fallback.
 * Returns false during SSR.
 */
export function isMacPlatform(): boolean {
  if (typeof navigator === "undefined") return false;

  // Modern API (Chromium 93+)
  const uaData = (navigator as Navigator & { userAgentData?: { platform?: string } }).userAgentData;
  if (uaData?.platform) return uaData.platform === "macOS";

  // Legacy fallback
  return /Mac|iPhone|iPad|iPod/.test(navigator.platform);
}
