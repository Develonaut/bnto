import { generatedHeader } from "../generatedHeader";

/** `catalog/iterationModes.ts`: the valid iteration modes. */
export function iterationModesFile(version: string, modes: string[]): string {
  return `${generatedHeader(version)}

/** Valid iteration modes from the engine's definition schema. */
export const ITERATION_MODES = ${JSON.stringify(modes)} as const;

/** Union type of valid iteration mode strings. */
export type IterationModeValue = (typeof ITERATION_MODES)[number];
`;
}
