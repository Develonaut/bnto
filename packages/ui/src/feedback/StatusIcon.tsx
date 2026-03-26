import { CheckCircle2Icon, LoaderIcon, XCircleIcon } from "../icons";
import { cn } from "../utils/cn";

type StatusIconStatus = "pending" | "running" | "completed" | "failed";

interface StatusIconProps {
  status: StatusIconStatus;
  /** sm = size-4, md = size-5. Default "md". */
  size?: "sm" | "md";
  className?: string;
}

const SIZE_CN = { sm: "size-4", md: "size-5" } as const;

/**
 * StatusIcon — maps a status to its canonical icon, color, and animation.
 *
 * - pending: static loader (muted)
 * - running: spinning loader (primary)
 * - completed: checkmark (success)
 * - failed: X circle (destructive)
 */
function StatusIcon({ status, size = "md", className }: StatusIconProps) {
  const sizeCn = SIZE_CN[size];

  switch (status) {
    case "pending":
      return <LoaderIcon className={cn(sizeCn, "shrink-0 text-muted-foreground", className)} />;
    case "running":
      return (
        <LoaderIcon
          className={cn(sizeCn, "shrink-0 text-primary motion-safe:animate-spin", className)}
        />
      );
    case "completed":
      return <CheckCircle2Icon className={cn(sizeCn, "shrink-0 text-success", className)} />;
    case "failed":
      return <XCircleIcon className={cn(sizeCn, "shrink-0 text-destructive", className)} />;
  }
}

export { StatusIcon };
export type { StatusIconStatus, StatusIconProps };
