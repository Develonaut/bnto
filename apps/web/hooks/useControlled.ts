import { useState, useCallback } from "react";

/**
 * Manages controlled vs uncontrolled state for a component prop.
 *
 * When `controlledValue` is defined, the component is controlled — the parent
 * owns the state and the hook forwards changes via `onChange`. When undefined,
 * the hook manages its own internal state.
 *
 * Follows the same pattern as MUI's useControlled hook.
 *
 * @example
 * ```tsx
 * function Toggle({ open, onOpenChange, defaultOpen = false }: Props) {
 *   const [value, setValue] = useControlled(open, defaultOpen, onOpenChange);
 *   return <button onClick={() => setValue(!value)}>{value ? "On" : "Off"}</button>;
 * }
 * ```
 */
export function useControlled<T>(
  controlledValue: T | undefined,
  defaultValue: T,
  onChange?: (value: T) => void,
): [T, (next: T) => void] {
  const isControlled = controlledValue !== undefined;

  /* Track the initial controlled mode via state — React 19's react-hooks/refs
   * rule forbids reading ref.current during render. State is the safe alternative. */
  const [initiallyControlled] = useState(isControlled);

  if (process.env.NODE_ENV !== "production") {
    if (initiallyControlled !== isControlled) {
      console.error(
        "useControlled: component switched between controlled and uncontrolled. " +
        "Decide between using a controlled or uncontrolled value for the lifetime of the component.",
      );
    }
  }

  const [internalValue, setInternalValue] = useState(defaultValue);
  const value = isControlled ? controlledValue : internalValue;

  const setValue = useCallback(
    (next: T) => {
      if (!isControlled) {
        setInternalValue(next);
      }
      onChange?.(next);
    },
    [isControlled, onChange],
  );

  return [value, setValue];
}
