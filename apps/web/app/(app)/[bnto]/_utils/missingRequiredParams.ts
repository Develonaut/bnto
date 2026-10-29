/**
 * Required params a run would leave without a value. Mirrors the engine's
 * pre-flight rule (`bnto_core::required_params`): a required param left out
 * has its default, if any; given as null or blank text, it has nothing. An
 * `undefined` value counts as left out: the JSON the engine gets drops it.
 */

import type { Definition } from "@bnto/core";
import {
  getNodeParamFields,
  getNodeSchema,
  getParamConstraints,
  getProcessorDefaults,
} from "@bnto/core";
import { extractProcessingNodes } from "./extractProcessingNodes";
import type { ProcessingNode } from "./extractProcessingNodes";

export interface MissingParam {
  nodeId: string;
  param: string;
  /** What the config form calls the param. */
  label: string;
}

/** Every required param with no value, in step order, then declared order. */
export function missingRequiredParams(
  definition: Definition,
  config: Record<string, Record<string, unknown>>,
): MissingParam[] {
  return extractProcessingNodes(definition).flatMap((node) =>
    missingInNode(node, { ...node.parameters, ...config[node.id] }),
  );
}

function missingInNode(node: ProcessingNode, values: Record<string, unknown>): MissingParam[] {
  const params = getNodeSchema(node.type)?.params ?? {};
  const defaults = getProcessorDefaults(node.type);
  const fields = getNodeParamFields(node.type);
  return Object.entries(params)
    .filter(([name]) => getParamConstraints(node.type, name)?.required)
    .filter(([name]) => (values[name] === undefined ? !(name in defaults) : isBlank(values[name])))
    .map(([name, meta]) => ({
      nodeId: node.id,
      param: name,
      label: fields?.[name]?.label ?? meta.label,
    }));
}

function isBlank(value: unknown): boolean {
  return value === null || (typeof value === "string" && !value.trim());
}
