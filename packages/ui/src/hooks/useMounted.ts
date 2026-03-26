"use client";

import { useSyncExternalStore } from "react";

const noop = () => {};
const subscribe = () => noop;
const getSnapshot = () => true;
const getServerSnapshot = () => false;

/** Returns `true` on the client after hydration, `false` during SSR. */
export function useMounted() {
  return useSyncExternalStore(subscribe, getSnapshot, getServerSnapshot);
}
