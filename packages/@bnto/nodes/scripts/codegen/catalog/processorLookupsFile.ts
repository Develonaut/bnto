import { generatedHeader } from "../generatedHeader";

// The whole file but its header: nothing in it depends on the catalog.
const BODY = `import type { ProcessorDef, ProcessorParam } from "./types";
import { PROCESSORS } from "./processors";

/** Map keyed by nodeType for O(1) lookup. */
export const PROCESSOR_MAP = new Map<string, ProcessorDef>(
  PROCESSORS.map((p) => [p.nodeType, p]),
);

/** Get the engine defaults for a node type. */
export function getProcessorDefaults(
  nodeType: string,
): Record<string, unknown> {
  const proc = PROCESSOR_MAP.get(nodeType);
  if (!proc) return {};
  const defaults: Record<string, unknown> = {};
  for (const param of proc.parameters) {
    if (param.default !== undefined) {
      defaults[param.name] = param.default;
    }
  }
  return defaults;
}

/** Get the engine constraints for a specific parameter. */
export function getParamConstraints(
  nodeType: string,
  paramName: string,
): ProcessorParam["constraints"] | undefined {
  const proc = PROCESSOR_MAP.get(nodeType);
  if (!proc) return undefined;
  const param = proc.parameters.find((p) => p.name === paramName);
  return param?.constraints;
}

/** Get the accepted MIME types for a node type. */
export function getProcessorAccepts(
  nodeType: string,
): readonly string[] {
  const proc = PROCESSOR_MAP.get(nodeType);
  return proc?.accepts ?? [];
}
`;

/** `catalog/processorLookups.ts`: lookups over `PROCESSORS`. */
export function processorLookupsFile(version: string): string {
  return `${generatedHeader(version)}

${BODY}`;
}
