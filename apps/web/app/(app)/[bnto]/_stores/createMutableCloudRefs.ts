import type { BrowserFileResult, FileUploadProgress, Definition } from "@bnto/core";
import type { RecipeFlowRefs as ActionRefs } from "./recipeFlowActions";
import type { RecipeFlowCloudRefs } from "./recipeFlowContext";

// ---------------------------------------------------------------------------
// Mutable cloud refs — updated by sync hook, read lazily by actions
// ---------------------------------------------------------------------------

interface MutableCloudRefs extends RecipeFlowCloudRefs, ActionRefs {
  syncUploadFn(fn: ((files: File[]) => Promise<{ sessionId: string }>) | null): void;
  syncStartCloudExecFn(
    fn:
      | ((args: { slug: string; definition: Definition; sessionId: string }) => Promise<unknown>)
      | null,
  ): void;
  syncResetUploadFn(fn: (() => void) | null): void;
  syncBrowserResults(results: BrowserFileResult[]): void;
  syncUploadProgress(progress: { files: FileUploadProgress[] }): void;
}

function createMutableCloudRefs(): MutableCloudRefs {
  let _uploadFn: ((files: File[]) => Promise<{ sessionId: string }>) | null = null;
  let _startCloudExecFn:
    | ((args: { slug: string; definition: Definition; sessionId: string }) => Promise<unknown>)
    | null = null;
  let _resetUploadFn: (() => void) | null = null;
  let _browserResults: BrowserFileResult[] = [];
  let _uploadProgress: { files: FileUploadProgress[] } = { files: [] };

  return {
    getBrowserResults: () => _browserResults,
    getUploadFn: () => _uploadFn,
    getStartCloudExecFn: () => _startCloudExecFn,
    getResetUploadFn: () => _resetUploadFn,
    getUploadProgress: () => _uploadProgress,
    syncUploadFn: (fn) => void (_uploadFn = fn),
    syncStartCloudExecFn: (fn) => void (_startCloudExecFn = fn),
    syncResetUploadFn: (fn) => void (_resetUploadFn = fn),
    syncBrowserResults: (results) => void (_browserResults = results),
    syncUploadProgress: (progress) => void (_uploadProgress = progress),
  };
}

export { createMutableCloudRefs };
export type { MutableCloudRefs };
