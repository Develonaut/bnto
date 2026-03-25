import type { EditorDebugApi } from "./editorDebugApi";

declare global {
  interface Window {
    __bnto__?: {
      editor?: EditorDebugApi;
      [key: string]: unknown;
    };
  }
}
