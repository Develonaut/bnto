import { indent } from "../indent";
import { toCamelCase } from "../toCamelCase";
import type { RawProcessor } from "../types";
import { paramLiteral } from "./paramLiteral";

/** `catalog/processors/<type>.ts`: one processor's definition. */
export function processorFile(p: RawProcessor, header: string): string {
  const params = p.parameters.map((param) => indent(paramLiteral(param), 1)).join(",\n");
  const cardinality = p.inputCardinality ?? "perFile";

  return `${header}

import type { ProcessorDef } from "../types";

/** Processor definition for ${p.nodeType}. */
export const ${toCamelCase(p.nodeType)}Processor: ProcessorDef = {
  nodeType: ${JSON.stringify(p.nodeType)},
  name: ${JSON.stringify(p.name)},
  description: ${JSON.stringify(p.description)},
  category: ${JSON.stringify(p.category)},
  accepts: ${JSON.stringify(p.accepts)} as const,
  platforms: ${JSON.stringify(p.platforms)} as const,
  parameters: [
${params},
  ],
  inputCardinality: ${JSON.stringify(cardinality)} as const,
};
`;
}
