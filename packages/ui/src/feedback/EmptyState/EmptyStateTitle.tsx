import type { ComponentProps } from "react";

import { Heading } from "../../typography/Heading";

export function EmptyStateTitle({
  children,
  className,
  ...props
}: Omit<ComponentProps<typeof Heading>, "level" | "size">) {
  return (
    <Heading level={3} size="xs" className={className} {...props}>
      {children}
    </Heading>
  );
}
