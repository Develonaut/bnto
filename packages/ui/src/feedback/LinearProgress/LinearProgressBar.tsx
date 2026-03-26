interface LinearProgressBarProps {
  clamped: number;
}

/** The animated track + fill bar. */
export function LinearProgressBar({ clamped }: LinearProgressBarProps) {
  return (
    <div
      role="progressbar"
      aria-valuenow={clamped}
      aria-valuemin={0}
      aria-valuemax={100}
      className="relative h-4 w-full overflow-hidden rounded-full border border-border bg-input"
    >
      <div
        className="h-full rounded-full bg-primary motion-safe:transition-[width] motion-safe:duration-fast"
        style={{ width: `${clamped}%` }}
      />
    </div>
  );
}
