import { useCallback } from "react";

/**
 * Generic config field change handler.
 *
 * Returns a stable callback that spreads the current config and
 * replaces a single field. Eliminates per-config handler boilerplate.
 *
 * ```ts
 * const change = useConfigChange(value, onChange);
 * change("format", "webp");
 * change("quality", 80);
 * ```
 */
export function useConfigChange<T extends object>(value: T, onChange: (config: T) => void) {
  return useCallback(
    <K extends keyof T>(field: K, next: T[K]) => onChange({ ...value, [field]: next }),
    [value, onChange],
  );
}
