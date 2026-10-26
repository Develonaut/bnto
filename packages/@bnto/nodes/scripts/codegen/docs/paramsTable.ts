import type { RawParameter, RawProcessor } from "../types";

function formatRange(p: RawParameter): string {
  if (p.constraints?.min !== undefined && p.constraints?.max !== undefined) {
    return `${p.constraints.min}-${p.constraints.max}`;
  }
  if (p.constraints?.min !== undefined) return `≥ ${p.constraints.min}`;
  if (p.constraints?.max !== undefined) return `≤ ${p.constraints.max}`;
  return "-";
}

function formatDefault(p: RawParameter): string {
  if (p.default === undefined) return "-";
  return String(p.default);
}

/** Text safe inside a Markdown table cell: pipes escaped, line breaks as `\n`. */
function tableCell(text: string): string {
  return text.replace(/\|/g, "\\|").replace(/\r?\n/g, "\\n");
}

function paramRow(param: RawParameter): string {
  const type = param.paramType.options ? "enum" : param.paramType.type;
  const range = param.paramType.options
    ? param.paramType.options.map((o) => o.value).join(", ")
    : formatRange(param);
  return `| ${param.name} | ${type} | ${tableCell(formatDefault(param))} | ${range} | ${tableCell(param.description)} |`;
}

/** A node README's "Parameters" section, or nothing when it has none. */
export function paramsTable(processor: RawProcessor): string[] {
  if (processor.parameters.length === 0) return [];
  return [
    "## Parameters",
    "",
    "| Parameter | Type | Default | Range | Description |",
    "|-----------|------|---------|-------|-------------|",
    ...processor.parameters.map(paramRow),
    "",
  ];
}
