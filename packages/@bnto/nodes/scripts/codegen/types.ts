/** Shapes of `engine/catalog.snapshot.json`, the input to node codegen. */

export interface CatalogEnvelope {
  version: string;
  nodeTypes: RawNodeType[];
  processors: RawProcessor[];
  definitionSchema?: Record<string, unknown>;
  recipes?: RawRecipe[];
}

export interface RawRecipe {
  slug: string;
  name: string;
  description: string;
  category: string;
  tags: string[];
  os: string[];
  definition: Record<string, unknown>;
}

export interface RawNodeType {
  name: string;
  label: string;
  description: string;
  category: string;
  isContainer: boolean;
  platforms: string[];
  icon: string;
  params?: RawNodeTypeParam[];
}

/** Condition for visibleWhen/requiredWhen in the catalog. */
export type ParamConditionRaw =
  | { param: string; equals: string }
  | Array<{ param: string; equals: string }>;

/** Extended parameter with presentation metadata from nodeTypes[].params. */
export interface RawNodeTypeParam extends RawParameter {
  suffix?: string;
  control?: string;
  presets?: Array<{ value: number; label: string }>;
  group?: string;
  visibleWhen?: ParamConditionRaw;
  requiredWhen?: ParamConditionRaw;
  inverted?: boolean;
}

export interface RawProcessor {
  nodeType: string;
  name: string;
  description: string;
  category: string;
  accepts: string[];
  platforms: string[];
  parameters: RawParameter[];
  inputCardinality?: string;
}

export interface RawParameter {
  name: string;
  label: string;
  description: string;
  paramType: {
    type: string;
    options?: Array<{ value: string; label: string }>;
    accept?: string[];
  };
  default?: unknown;
  constraints?: { min?: number; max?: number; required?: boolean };
  placeholder?: string;
  surfaceable?: boolean;
}
