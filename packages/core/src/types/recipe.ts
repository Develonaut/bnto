// ---------------------------------------------------------------------------
// Recipe types (transport-agnostic — no Convex imports)
// ---------------------------------------------------------------------------

/** A recipe — unified persistence shape for local and cloud recipes. */
export interface Recipe {
  id: string;
  name: string;
  definition: RecipeDefinition;
  type: string;
  version: string;
  /** Convex document _id. Undefined if local-only (not yet saved to cloud). */
  cloudId?: string;
  savedAt: number;
  /** When last synced to Convex. null = never synced. */
  syncedAt: number | null;
}

/** Projected recipe for list views. */
export interface RecipeListItem {
  id: string;
  name: string;
  nodeCount: number;
  /** Human-readable labels for the distinct processing node types (excludes I/O and containers). */
  nodeTypes: string[];
  updatedAt: number;
  /** When last synced to Convex. null = never synced, undefined = cloud recipe (always synced). */
  syncedAt?: number | null;
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
