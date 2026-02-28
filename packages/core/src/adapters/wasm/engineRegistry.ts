/**
 * Browser engine registry — singleton pattern.
 *
 * The app (apps/web) registers its BntoWorker as the browser engine
 * at startup. The adapter uses the registered engine to process files.
 *
 * Separation of concerns: @bnto/core defines the BrowserEngine interface,
 * apps/web provides the implementation (BntoWorker wrapping WASM).
 */

import type { BrowserEngine } from "../../types/wasm";

/** Singleton engine reference. Set by registerBrowserEngine(). */
let engine: BrowserEngine | null = null;

/** Register a browser engine implementation (called once at app startup). */
export function registerBrowserEngine(e: BrowserEngine): void {
  engine = e;
}

/** Get the registered browser engine, or null if none registered. */
export function getBrowserEngine(): BrowserEngine | null {
  return engine;
}

/** Check if a browser engine has been registered. */
export function hasBrowserEngine(): boolean {
  return engine !== null;
}
