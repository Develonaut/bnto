/** Apply pipeline settings to the current definition in the store. */

import type { StoreApi } from "zustand";
import type { PipelineSettings } from "@bnto/core";
import type { EditorStore } from "../store/types";

function applySettings(storeApi: StoreApi<EditorStore>, settings: PipelineSettings) {
  const { definition } = storeApi.getState();
  if (!definition) return;
  storeApi.setState({
    definition: { ...definition, settings },
    isDirty: true,
  });
}

export { applySettings };
