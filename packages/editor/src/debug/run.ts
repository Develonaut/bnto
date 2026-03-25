/** Auto-step through the full pipeline with a delay between each step. */

import type { StoreApi } from "zustand";
import type { EditorStore } from "../store/types";
import { step } from "./step";

async function run(store: StoreApi<EditorStore>, delayMs = 800): Promise<void> {
  const wait = (ms: number) => new Promise<void>((r) => setTimeout(r, ms));
  // eslint-disable-next-line no-constant-condition
  while (true) {
    const msg = step(store);
    // eslint-disable-next-line no-console
    console.log(`[bnto] ${msg}`);
    const phase = store.getState().executionPhase;
    if (phase === "completed" || phase === "failed") break;
    if (msg.startsWith("No processing") || msg.startsWith("Already")) break;
    await wait(delayMs);
  }
}

export { run };
