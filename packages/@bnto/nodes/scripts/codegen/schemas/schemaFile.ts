import { toCamelCase } from "../toCamelCase";
import { toPascalCase } from "../toPascalCase";
import type { RawParameter } from "../types";
import { nodeParamEntry } from "./nodeParamEntry";
import { zodField } from "./zodField";

function nodeSchemaConst(nodeType: string, params: RawParameter[]): string {
  const camelName = toCamelCase(nodeType);
  return `/** Full schema definition for the ${nodeType} node type. */
export const ${camelName}NodeSchema: NodeSchema = {
  nodeType: ${JSON.stringify(nodeType)},
  schemaVersion: 1,
  schema: ${camelName}ParamsSchema,
  params: {
${params.map((p) => nodeParamEntry(p)).join("\n")}
  },
};`;
}

/**
 * `schemas/<type>.ts`: a node type's Zod params schema, its inferred type and
 * its `NodeSchema`. The params come from the processor, or from
 * `nodeTypes[].params` for a type with no processor.
 */
export function schemaFile(nodeType: string, params: RawParameter[], header: string): string {
  const camelName = toCamelCase(nodeType);
  const zodFields = params.map((p) => `  ${zodField(p)},`);

  return `${header}

import { z } from "zod";
import type { NodeSchema } from "../../schemas/types";

/** Zod schema for ${nodeType} node parameters. */
export const ${camelName}ParamsSchema = z.object({
${zodFields.join("\n")}
});

/** Inferred TypeScript type for ${nodeType} node parameters. */
export type ${toPascalCase(nodeType)}Params = z.infer<typeof ${camelName}ParamsSchema>;

${nodeSchemaConst(nodeType, params)}
`;
}
