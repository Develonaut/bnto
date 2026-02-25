"use client";

/**
 * Browser execution service — orchestrates file processing via WASM.
 *
 * The browser execution path is fundamentally different from cloud:
 * - No R2 upload/download (files stay in memory)
 * - No Convex execution record (state is local)
 * - No Go API call (WASM processes in the browser)
 * - Progress comes from the Web Worker, not polling
 *
 * The service delegates to a registered BrowserEngine (typically
 * the BntoWorker wrapping WASM) and provides a clean imperative API.
 */

import {
  getBrowserEngine,
  registerBrowserEngine,
  hasBrowserEngine,
} from "../adapters/browser/engineRegistry";
import {
  isBrowserCapable,
  getBrowserNodeType,
  getBrowserCapableSlugs,
} from "../adapters/browser/slugCapability";
import { downloadBlob } from "../adapters/browser/downloadBlob";
import { createZipBlob } from "../adapters/browser/createZipBlob";
import type {
  BrowserEngine,
  BrowserFileResult,
  BrowserFileProgress,
} from "../types/browser";

export function createBrowserExecutionService() {
  return {
    // ── Capability Detection ───────────────────────────────────────
    /** Check if a slug has a working browser execution path. */
    isCapable: (slug: string) => isBrowserCapable(slug) && hasBrowserEngine(),

    /** Check if a slug has a browser implementation (ignores engine registration). */
    hasImplementation: (slug: string) => isBrowserCapable(slug),

    /** Get all slugs with browser implementations. */
    getCapableSlugs: () => getBrowserCapableSlugs(),

    // ── Engine Registration ────────────────────────────────────────
    /** Register the browser engine (called once at app startup). */
    registerEngine: (engine: BrowserEngine) => registerBrowserEngine(engine),

    /** Whether a browser engine has been registered. */
    hasEngine: () => hasBrowserEngine(),

    // ── Execution ──────────────────────────────────────────────────
    /**
     * Execute a bnto in the browser.
     *
     * Initializes the engine if needed, processes all files through
     * the WASM worker, and returns results as in-memory blobs.
     *
     * @param slug - The bnto slug (e.g., "compress-images").
     * @param files - Files to process (from FileDropZone).
     * @param params - Node-specific config (e.g., { quality: 80 }).
     * @param onProgress - Callback for per-file progress updates.
     * @returns Array of processed file results.
     */
    execute: async (
      slug: string,
      files: File[],
      params: Record<string, unknown> = {},
      onProgress?: (progress: BrowserFileProgress) => void,
    ): Promise<BrowserFileResult[]> => {
      const engine = getBrowserEngine();
      if (!engine) {
        throw new Error(
          "No browser engine registered. " +
            "Call core.browser.registerEngine() at app startup.",
        );
      }

      const nodeType = getBrowserNodeType(slug);
      if (!nodeType) {
        throw new Error(
          `No browser implementation for slug "${slug}". ` +
            `Available: ${getBrowserCapableSlugs().join(", ")}`,
        );
      }

      // Initialize the engine (no-op if already initialized).
      await engine.init();

      // Process all files, forwarding progress updates.
      return engine.processFiles(
        files,
        nodeType,
        params,
        onProgress
          ? (fileIndex, percent, message) =>
              onProgress({
                fileIndex,
                totalFiles: files.length,
                percent,
                message,
              })
          : undefined,
      );
    },

    // ── Download ───────────────────────────────────────────────────
    /** Download a browser execution result as a file. */
    downloadResult: (result: BrowserFileResult) => {
      downloadBlob(result.blob, result.filename);
    },

    /**
     * Download all browser execution results as a single ZIP archive.
     *
     * @param results - Processed files to bundle.
     * @param slug - The bnto slug, used for the archive filename.
     */
    downloadAllResults: async (
      results: BrowserFileResult[],
      slug?: string,
    ) => {
      const zipBlob = await createZipBlob(results);
      const name = slug ? `${slug}-results.zip` : "bnto-results.zip";
      downloadBlob(zipBlob, name);
    },
  } as const;
}

export type BrowserExecutionService = ReturnType<
  typeof createBrowserExecutionService
>;
