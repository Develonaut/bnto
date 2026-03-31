"use client";

import { createContext, useContext, type ReactNode } from "react";
import { useStore } from "zustand";
import type { StoreApi } from "zustand/vanilla";

// ---------------------------------------------------------------------------
// Public interface
// ---------------------------------------------------------------------------

interface StoreContext<T> {
  /** The React Context holding the store instance. */
  Context: React.Context<StoreApi<T> | null>;
  /** Provider component — wraps children, provides the store. */
  Provider: React.FC<{ store: StoreApi<T>; children: ReactNode }>;
  /** Reactive selector hook — re-renders only when selected slice changes. */
  useStore: <U>(selector: (state: T) => U) => U;
  /** Raw StoreApi access — for imperative reads in callbacks/effects. */
  useStoreApi: () => StoreApi<T>;
}

// ---------------------------------------------------------------------------
// Factory
// ---------------------------------------------------------------------------

/**
 * Create a type-safe Context + consumer hooks for a Zustand store.
 *
 * Codifies the official Zustand recommendation for Next.js:
 * createStore() factory + React Context + useStore() consumer.
 *
 * Usage:
 *   const { Provider, useStore, useStoreApi } = createStoreContext<MyState>("MyFeature");
 *
 *   // Provide (in your feature root)
 *   const storeRef = useRef<StoreApi<MyState>>(null);
 *   if (!storeRef.current) storeRef.current = createEnhancedStore<MyState>()((...) => ({ ... }));
 *   return <Provider store={storeRef.current}>{children}</Provider>;
 *
 *   // Consume (in any descendant)
 *   const count = useStore(s => s.count);
 *   const api = useStoreApi();
 */
export function createStoreContext<T>(displayName: string): StoreContext<T> {
  const Ctx = createContext<StoreApi<T> | null>(null);
  Ctx.displayName = `${displayName}StoreContext`;

  function Provider({ store, children }: { store: StoreApi<T>; children: ReactNode }) {
    return <Ctx value={store}>{children}</Ctx>;
  }

  function useStoreApi(): StoreApi<T> {
    const store = useContext(Ctx);
    if (!store) throw new Error(`use${displayName}Store must be used within <${displayName}>`);
    return store;
  }

  function useStoreHook<U>(selector: (state: T) => U): U {
    return useStore(useStoreApi(), selector);
  }

  return { Context: Ctx, Provider, useStore: useStoreHook, useStoreApi };
}

export type { StoreContext };
