// ---------------------------------------------------------------------------
// Recipe types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** Full recipe as returned by detail queries. */
export interface Recipe {
  id: string;
  userId: string;
  name: string;
  definition: RecipeDefinition;
  version: number;
  /** Definition format version (semver) — tracks which format spec the definition uses. */
  formatVersion?: string;
  isPublic: boolean;
  createdAt: number;
  updatedAt: number;
}

/** Projected recipe for list views. */
export interface RecipeListItem {
  id: string;
  name: string;
  nodeCount: number;
  updatedAt: number;
}

/** Input for creating or updating a recipe. */
export interface SaveRecipeInput {
  name: string;
  definition: RecipeDefinition;
  isPublic?: boolean;
}

// ---------------------------------------------------------------------------
// Recipe definition types (matches Go node.Definition JSON)
// ---------------------------------------------------------------------------

export interface RecipeDefinition {
  id: string;
  type: string;
  version: string;
  parentId?: string;
  name: string;
  position: Position;
  metadata: Metadata;
  parameters: Record<string, unknown>;
  fields?: FieldsConfig;
  inputPorts: Port[];
  outputPorts: Port[];
  nodes?: RecipeDefinition[];
  edges?: Edge[];
}

export interface Position {
  x: number;
  y: number;
}

export interface Metadata {
  createdAt?: string;
  updatedAt?: string;
  tags?: string[];
  customData?: Record<string, string>;
}

export interface Port {
  id: string;
  name: string;
  handle?: string;
}

export interface Edge {
  id: string;
  source: string;
  target: string;
  sourceHandle?: string;
  targetHandle?: string;
}

export interface FieldsConfig {
  values: Record<string, unknown>;
  keepOnlySet?: boolean;
}

// ---------------------------------------------------------------------------
// API response types (matches Go JSON responses)
// ---------------------------------------------------------------------------

export interface RecipeSummary {
  name: string;
  nodeCount: number;
}

export interface ValidationResult {
  valid: boolean;
  errors?: string[];
}

export interface RunResponse {
  id: string;
}
