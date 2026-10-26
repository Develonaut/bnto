import { toCamelCase } from "../toCamelCase";
import type { CatalogEnvelope, RawParameter } from "../types";

type Strings = Record<string, string>;

function nodeTypeStrings(catalog: CatalogEnvelope) {
  const nodeTypes: Record<string, Strings> = {};
  const categories: Strings = {};
  for (const nt of catalog.nodeTypes) {
    nodeTypes[toCamelCase(nt.name)] = { label: nt.label, description: nt.description };
    if (!categories[nt.category]) {
      categories[nt.category] = nt.category.charAt(0).toUpperCase() + nt.category.slice(1);
    }
  }
  return { nodeTypes, categories };
}

function paramStrings(parameters: RawParameter[]): Record<string, Strings> {
  const entries: Record<string, Strings> = {};
  for (const p of parameters) {
    const entry: Strings = { label: p.label, description: p.description };
    if (p.placeholder) entry.placeholder = p.placeholder;
    entries[p.name] = entry;
  }
  return entries;
}

/** `@bnto/i18n`'s `nodes.json`: node type, param and category strings. */
export function i18nStringsFile(catalog: CatalogEnvelope): string {
  const { nodeTypes, categories } = nodeTypeStrings(catalog);
  const params: Record<string, Record<string, Strings>> = {};
  for (const proc of catalog.processors) {
    if (proc.parameters.length === 0) continue;
    params[toCamelCase(proc.nodeType)] = paramStrings(proc.parameters);
  }

  const output = {
    _generatedBy: `generate-from-catalog.ts - DO NOT EDIT manually. Engine catalog v${catalog.version}`,
    nodeTypes,
    params,
    categories,
  };
  return JSON.stringify(output, null, 2) + "\n";
}
