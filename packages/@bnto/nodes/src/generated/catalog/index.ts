/**
 * AUTO-GENERATED from engine/catalog.snapshot.json - DO NOT EDIT.
 * Run `task nodes:generate` to regenerate after engine changes.
 * Engine catalog v1.0.0
 */

// Types
export type {
  NodeTypeName,
  NodeCategory,
  NodeTypeInfo,
  ParamType,
  ProcessorParam,
  ProcessorDef,
} from "./types";

// Node types
export { NODE_TYPES, NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";

// Processors
export { PROCESSORS } from "./processors";

// Lookup helpers
export {
  PROCESSOR_MAP,
  getProcessorDefaults,
  getParamConstraints,
  getProcessorAccepts,
} from "./processorLookups";

// Iteration modes
export { ITERATION_MODES } from "./iterationModes";
export type { IterationModeValue } from "./iterationModes";
