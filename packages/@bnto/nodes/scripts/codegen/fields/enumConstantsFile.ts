import { generatedHeader } from "../generatedHeader";
import type { CatalogEnvelope, RawNodeType } from "../types";

/** Known enum constant mappings: nodeType.paramName → exported constant name. */
const ENUM_CONSTANT_MAP: Record<string, Record<string, string>> = {
  input: { mode: "INPUT_MODES" },
  output: { mode: "OUTPUT_MODES" },
  loop: { mode: "LOOP_MODES" },
  group: { mode: "GROUP_MODES" },
  parallel: { errorStrategy: "ERROR_STRATEGIES" },
  "image-convert": { format: "IMAGE_FORMATS" },
};

/** The `export const X = [...] as const;` lines for one node type's mapped params. */
function constantsFor(nt: RawNodeType, paramMap: Record<string, string>): string[] {
  return (nt.params ?? []).flatMap((param) => {
    const constName = paramMap[param.name];
    const options = param.paramType.options;
    if (!constName || !options || options.length === 0) return [];
    const values = options.map((o) => JSON.stringify(o.value)).join(", ");
    return [`export const ${constName} = [${values}] as const;`];
  });
}

/** `enumConstants.ts`: named option lists for the params in `ENUM_CONSTANT_MAP`. */
export function enumConstantsFile(catalog: CatalogEnvelope): string {
  const sections = catalog.nodeTypes.flatMap((nt) => {
    const paramMap = ENUM_CONSTANT_MAP[nt.name];
    return paramMap ? constantsFor(nt, paramMap) : [];
  });

  return `${generatedHeader(catalog.version)}

${sections.join("\n\n")}
`;
}
