/**
 * AUTO-GENERATED from engine/catalog.snapshot.json — DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

/** Valid iteration modes from the engine's definition schema. */
export const ITERATION_MODES = ["auto","explicit"] as const;

/** Union type of valid iteration mode strings. */
export type IterationModeValue = (typeof ITERATION_MODES)[number];
