import { t } from "./t";

/** Hook wrapper around `t()`. Trivial now; locale-context-aware later. */
export function useT() {
  return t;
}
