import { useSyncExternalStore } from "react";
import { getFlag, type FeatureFlag } from "./featureFlags";

const CHANGE_EVENT = "bnto:flags-changed";
const DEFAULTS = { editor: false } as const;

export function useFeatureFlag(flag: FeatureFlag): boolean {
  return useSyncExternalStore(
    (cb) => {
      window.addEventListener(CHANGE_EVENT, cb);
      window.addEventListener("storage", cb);
      return () => {
        window.removeEventListener(CHANGE_EVENT, cb);
        window.removeEventListener("storage", cb);
      };
    },
    () => getFlag(flag),
    () => DEFAULTS[flag],
  );
}
