import type { Execution } from "@bnto/core";

import { Badge } from "@bnto/ui";

type ExecutionStatus = Execution["status"];

const STATUS_VARIANT: Record<ExecutionStatus, "default" | "primary" | "secondary" | "destructive"> = {
  pending: "default",
  running: "secondary",
  completed: "primary",
  failed: "destructive",
};

const STATUS_LABEL: Record<ExecutionStatus, string> = {
  pending: "Pending",
  running: "Running",
  completed: "Completed",
  failed: "Failed",
};

interface StatusBadgeProps {
  status: ExecutionStatus;
  className?: string;
}

export function StatusBadge({ status, className }: StatusBadgeProps) {
  return (
    <Badge variant={STATUS_VARIANT[status]} size="sm" className={className}>
      {STATUS_LABEL[status]}
    </Badge>
  );
}
