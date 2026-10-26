import { generatedHeader } from "../generatedHeader";

const ITERATION_MODES_EXPORTS = `
// Iteration modes
export { ITERATION_MODES } from "./iterationModes";
export type { IterationModeValue } from "./iterationModes";`;

/** `catalog/index.ts`: re-exports the catalog's types, lists and lookups. */
export function catalogBarrel(version: string, hasIterationModes: boolean): string {
  return `${generatedHeader(version)}

// Types
export type { NodeTypeName, NodeCategory, NodeTypeInfo, ParamType, ProcessorParam, ProcessorDef } from "./types";

// Node types
export { NODE_TYPES, NODE_TYPE_NAMES, NODE_TYPE_INFO } from "./nodeTypes";

// Processors
export { PROCESSORS } from "./processors";

// Lookup helpers
export { PROCESSOR_MAP, getProcessorDefaults, getParamConstraints, getProcessorAccepts } from "./processorLookups";
${hasIterationModes ? ITERATION_MODES_EXPORTS : ""}
`;
}
