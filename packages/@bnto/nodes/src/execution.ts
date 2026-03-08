/** Execution context types — data passed between nodes during recipe runs. */

/**
 * Data passed between nodes during recipe execution.
 *
 * The engine manages this context — each node's result is merged into
 * `data`, making it available to downstream nodes.
 */
export interface ExecutionContext {
  /**
   * Accumulated results from all previous nodes.
   *
   * When a node executes, its result is merged into this map.
   * Downstream nodes access upstream results via this data.
   *
   * Example: If node "fetch-user" returns `{ user: { id: 123 } }`,
   * downstream nodes access it via `data["fetch-user"]`.
   */
  data: Record<string, unknown>;

  /** ID of the currently executing node. */
  nodeId: string;

  /**
   * Nesting depth for recursive group/loop execution.
   *
   * Top-level = 0, inside a group = 1, nested group = 2, etc.
   */
  depth: number;
}

/**
 * Result from a single node execution.
 *
 * Each node type produces output in its own shape. This is the
 * common wrapper that the engine uses.
 */
export interface NodeResult {
  /** The node's output data. Shape depends on node type. */
  output: unknown;

  /** Duration in milliseconds. */
  durationMs: number;

  /** Error message if the node failed. */
  error?: string;
}
