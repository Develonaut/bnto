/** Detect whether we're running inside a Wails v2 webview. */
export function isWailsEnvironment(): boolean {
  return (
    typeof window !== "undefined" &&
    "go" in window &&
    typeof (window as Record<string, unknown>).go === "object"
  );
}
