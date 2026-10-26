import { generatedHeader } from "../generatedHeader";
import { toCamelCase } from "../toCamelCase";
import type { CatalogEnvelope } from "../types";

/** `catalog/processors/index.ts`: the `PROCESSORS` list. */
export function processorsBarrel(catalog: CatalogEnvelope): string {
  const sorted = [...catalog.processors].sort((a, b) => a.nodeType.localeCompare(b.nodeType));
  const names = sorted.map((p) => toCamelCase(p.nodeType));
  const imports = names.map((n) => `import { ${n}Processor } from "./${n}";`).join("\n");
  const entries = names.map((n) => `  ${n}Processor`).join(",\n");

  return `${generatedHeader(catalog.version)}

import type { ProcessorDef } from "../types";

${imports}

export const PROCESSORS: readonly ProcessorDef[] = [
${entries},
] as const;
`;
}
