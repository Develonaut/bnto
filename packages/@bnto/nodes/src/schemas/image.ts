/**
 * Image node schema — re-exports from engine-generated schemas.
 *
 * The Rust engine is the single source of truth for image node parameters.
 * Zod schemas, UI metadata (visibleWhen, labels, descriptions), and
 * constraints are all generated from the engine catalog snapshot.
 *
 * To change image parameters, edit the processor `metadata()` in
 * `engine/crates/bnto-image/src/`, then run `task nodes:generate`.
 */

export { IMAGE_OPERATIONS, imageParamsSchema, imageNodeSchema } from "../generated/schemas";

export type { ImageParams } from "../generated/schemas";

// Re-derive IMAGE_FORMATS from the generated schema for backward compat
import { PROCESSOR_MAP } from "../generated/catalog";
const convertProc = PROCESSOR_MAP.get("image:convert");
const formatParam = convertProc?.parameters.find((p) => p.name === "format");
export const IMAGE_FORMATS = (formatParam?.options ?? ["jpeg", "png", "webp"]) as readonly string[];
