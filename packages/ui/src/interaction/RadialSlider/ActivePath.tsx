interface ActivePathProps {
  activePath: string;
  activeStroke?: string;
  activeClassName: string;
  strokeWidth: number;
}

/** Colored arc representing the current value. */
export function ActivePath({
  activePath,
  activeStroke,
  activeClassName,
  strokeWidth,
}: ActivePathProps) {
  if (!activePath) return null;
  return (
    <path
      d={activePath}
      fill="none"
      stroke={activeStroke ?? "currentColor"}
      className={activeStroke ? undefined : activeClassName}
      strokeWidth={strokeWidth}
      strokeLinecap="round"
    />
  );
}
