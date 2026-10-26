import type { RawNodeType, RawProcessor } from "../types";
import { configExample } from "./configExample";
import { paramsTable } from "./paramsTable";

function titleLines(nodeType: RawNodeType): string[] {
  const platforms = nodeType.platforms.join(", ") || "none";
  const container = nodeType.isContainer ? "yes" : "no";
  return [
    `# ${nodeType.label} Node`,
    "",
    `> ${nodeType.description}`,
    "",
    `**Category:** ${nodeType.category} | **Platforms:** ${platforms} | **Container:** ${container}`,
    "",
  ];
}

function acceptsLines(processor: RawProcessor): string[] {
  if (processor.accepts.length === 0) return [];
  return ["## Accepts", "", ...processor.accepts.map((mime) => `- \`${mime}\``), ""];
}

/** `docs/<type>.md`: a node's README. Only a node with a processor gets the processor sections. */
export function nodeReadme(
  nodeType: RawNodeType,
  processor: RawProcessor | undefined,
  version: string,
): string {
  const processorLines = processor
    ? [...acceptsLines(processor), ...paramsTable(processor), ...configExample(nodeType, processor)]
    : [];
  return [
    ...titleLines(nodeType),
    ...processorLines,
    "---",
    "",
    `*Auto-generated from engine catalog v${version}. Run \`task nodes:generate\` to regenerate.*`,
    "",
  ].join("\n");
}
