/**
 * NodeProgressFill — horizontal progress overlay inside a node card.
 *
 * Uses `transform: scaleX()` for compositor-only animation (no reflow).
 * Color is the primary (terracotta) at low opacity so the fill is
 * visible on white cards and blends on colored variants.
 * Renders null when progress is undefined or 0.
 */

interface NodeProgressFillProps {
  /** Progress percentage (0–100). Undefined = no fill. */
  progress?: number;
}

function NodeProgressFill({ progress }: NodeProgressFillProps) {
  if (progress === undefined || progress <= 0) return null;

  return (
    <div
      className="node-progress-fill absolute inset-0 rounded-xl origin-left pointer-events-none motion-safe:transition-transform duration-fast ease-out"
      style={{ transform: `scaleX(${progress / 100})` }}
      aria-hidden="true"
    />
  );
}

export { NodeProgressFill };
