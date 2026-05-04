/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

export const GROUP_MODES = ["sequential", "parallel"] as const;

export const IMAGE_FORMATS = ["jpeg", "png", "webp"] as const;

export const INPUT_MODES = ["file-upload", "text", "url"] as const;

export const LOOP_MODES = ["forEach", "times", "while"] as const;

export const OUTPUT_MODES = ["write", "overwrite", "message", "none"] as const;

export const ERROR_STRATEGIES = ["failFast", "collectAll"] as const;
