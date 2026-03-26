interface TrackPathsProps {
  trackPath: string;
  trackStroke?: string;
  trackClassName: string;
  strokeWidth: number;
}

/** Visible track arc (border outline + colored fill). */
export function TrackPaths({
  trackPath,
  trackStroke,
  trackClassName,
  strokeWidth,
}: TrackPathsProps) {
  return (
    <>
      {!trackStroke && (
        <path
          d={trackPath}
          fill="none"
          stroke="currentColor"
          className="text-border"
          strokeWidth={strokeWidth + 2}
          strokeLinecap="round"
        />
      )}
      <path
        d={trackPath}
        fill="none"
        stroke={trackStroke ?? "currentColor"}
        className={trackStroke ? undefined : trackClassName}
        strokeWidth={strokeWidth}
        strokeLinecap="round"
      />
    </>
  );
}
