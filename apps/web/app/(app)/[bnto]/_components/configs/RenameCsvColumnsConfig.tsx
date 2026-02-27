"use client";

/**
 * Placeholder config for rename-csv-columns.
 *
 * Column mapping requires array-level transform support in the engine,
 * which isn't available yet (see PLAN.md backlog). For now, this renders
 * an informational message explaining the workflow's current behavior.
 */
export function RenameCsvColumnsConfig() {
  return (
    <div className="space-y-2">
      <p className="text-muted-foreground text-sm">
        This tool reads your CSV and writes a cleaned copy. Column remapping
        will be available in a future update.
      </p>
      <p className="text-muted-foreground text-xs">
        Currently preserves all columns and headers as-is.
      </p>
    </div>
  );
}
