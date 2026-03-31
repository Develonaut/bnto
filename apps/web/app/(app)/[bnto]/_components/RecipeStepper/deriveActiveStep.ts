import type { RunStep } from "./RunButton";

/**
 * Map the unified RunStep + file count to the 3-step stepper position.
 *
 * Step 1 = no files yet (upload prompt)
 * Step 2 = files selected (configure)
 * Step 3 = execution in progress or complete
 */
export function deriveActiveStep(resolvedStep: RunStep, fileCount: number): 1 | 2 | 3 {
  switch (resolvedStep) {
    case "running":
    case "completed":
    case "failed":
      return 3;
    default:
      return fileCount > 0 ? 2 : 1;
  }
}
