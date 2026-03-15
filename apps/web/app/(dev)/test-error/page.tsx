"use client";

/**
 * Dev-only page that intentionally throws during render.
 * Used by E2E tests to verify error boundaries catch errors
 * and display the ErrorReport dialog.
 *
 * Only available in dev mode (under (dev) route group with noindex).
 */
export default function TestErrorPage() {
  throw new Error("Intentional test error for E2E verification");
}
