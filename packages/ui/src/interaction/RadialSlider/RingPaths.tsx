interface RingPathsProps {
  ringPath: string;
  trackClassName: string;
  strokeWidth: number;
}

/** Background ring behind the active track arc. */
export function RingPaths({ ringPath, trackClassName, strokeWidth }: RingPathsProps) {
  return (
    <>
      <path
        d={ringPath}
        fill="none"
        stroke="currentColor"
        className="text-border"
        strokeWidth={strokeWidth + 2}
        strokeLinecap="round"
        opacity={0.3}
      />
      <path
        d={ringPath}
        fill="none"
        stroke="currentColor"
        className={trackClassName}
        strokeWidth={strokeWidth}
        strokeLinecap="round"
        opacity={0.3}
      />
    </>
  );
}
