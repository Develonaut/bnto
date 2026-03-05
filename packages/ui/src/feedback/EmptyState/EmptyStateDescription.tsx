import type { ComponentProps } from "react";

import { cn } from "../../utils/cn";
import { Text } from "../../typography/Text";

export function EmptyStateDescription({
  children,
  className,
  ...props
}: Omit<ComponentProps<typeof Text>, "size" | "color" | "balance">) {
  return (
    <Text size="sm" color="muted" balance className={cn("max-w-xs", className)} {...props}>
      {children}
    </Text>
  );
}
