import type { RawNodeTypeParam } from "../types";
import { serializeCondition } from "./serializeCondition";

type Entry = (p: RawNodeTypeParam) => string | undefined;

function presets(p: RawNodeTypeParam): string | undefined {
  if (!p.presets) return undefined;
  const list = p.presets.map((x) => `{ value: ${x.value}, label: ${JSON.stringify(x.label)} }`);
  return `presets: [${list.join(", ")}]`;
}

/** Options for enum params with option labels. */
function options(p: RawNodeTypeParam): string | undefined {
  const opts = p.paramType.options;
  if (!opts || opts.length === 0) return undefined;
  const list = opts.map(
    (o) => `{ value: ${JSON.stringify(o.value)}, label: ${JSON.stringify(o.label)} }`,
  );
  return `options: [${list.join(", ")}]`;
}

// In output order.
const ENTRIES: Entry[] = [
  (p) => (p.suffix ? `suffix: ${JSON.stringify(p.suffix)}` : undefined),
  (p) => (p.control ? `control: ${JSON.stringify(p.control)}` : undefined),
  presets,
  (p) => (p.group ? `group: ${JSON.stringify(p.group)}` : undefined),
  (p) => (p.visibleWhen ? `visibleWhen: ${serializeCondition(p.visibleWhen)}` : undefined),
  (p) => (p.requiredWhen ? `requiredWhen: ${serializeCondition(p.requiredWhen)}` : undefined),
  (p) => (p.inverted ? `inverted: true` : undefined),
  (p) => (p.paramType.accept ? `accept: ${JSON.stringify(p.paramType.accept)}` : undefined),
  options,
];

/** Generate a NodeParamField literal for a single param. Returns empty string if no fields. */
export function paramFieldLiteral(param: RawNodeTypeParam): string {
  const entries = ENTRIES.map((entry) => entry(param)).filter((e) => e !== undefined);
  if (entries.length === 0) return "";
  return `    ${param.name}: { ${entries.join(", ")} }`;
}
