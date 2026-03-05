import type { ComponentProps, ElementType, ReactNode } from "react";

import { cn } from "../utils/cn";

import { createCn } from "../utils/createCn";
import { Heading } from "../typography/Heading";
import { Stack } from "../layout/Stack";
import { Text } from "../typography/Text";

/* ── Size variant ─────────────────────────────────────────────── */

type EmptyStateSize = "sm" | "md" | "lg";

const rootCn = createCn({
  base: "flex flex-col items-center text-center",
  variants: {
    size: {
      sm: "gap-2 py-6",
      md: "gap-3 py-10",
      lg: "gap-4 py-16",
    },
  },
  defaultVariants: { size: "md" },
});

const iconSizeMap: Record<EmptyStateSize, string> = {
  sm: "size-8",
  md: "size-10",
  lg: "size-12",
};

/* ── Sub-components ───────────────────────────────────────────── */

function EmptyStateIcon({
  children,
  size = "md",
  className,
}: {
  children: ReactNode;
  size?: EmptyStateSize;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "text-muted-foreground [&>svg]:size-full",
        iconSizeMap[size],
        className,
      )}
    >
      {children}
    </div>
  );
}

function EmptyStateTitle({
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

function EmptyStateDescription({
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

function EmptyStateAction({
  children,
  className,
  as: Tag = "div",
  ...props
}: ComponentProps<"div"> & { as?: ElementType }) {
  return (
    <Tag className={cn("mt-1", className)} {...props}>
      {children}
    </Tag>
  );
}

/* ── Root ─────────────────────────────────────────────────────── */

type EmptyStateRootProps = ComponentProps<"div"> & {
  /** Overall size variant. Controls spacing, icon size, and padding. Default `"md"`. */
  size?: EmptyStateSize;
};

function EmptyStateRoot({
  size = "md",
  className,
  ...props
}: EmptyStateRootProps) {
  return <div className={rootCn({ size }, className)} {...props} />;
}

/* ── Skeleton ─────────────────────────────────────────────────── */

function EmptyStateSkeleton({ className }: { className?: string }) {
  return (
    <Stack align="center" gap="sm" className={cn("py-10", className)}>
      <div className="size-10 rounded-full bg-muted motion-safe:animate-pulse" />
      <div className="h-5 w-32 rounded-md bg-muted motion-safe:animate-pulse" />
      <div className="h-4 w-48 rounded-md bg-muted motion-safe:animate-pulse" />
    </Stack>
  );
}

/* ── Assembly ─────────────────────────────────────────────────── */

export const EmptyState = Object.assign(EmptyStateRoot, {
  Root: EmptyStateRoot,
  Icon: EmptyStateIcon,
  Title: EmptyStateTitle,
  Description: EmptyStateDescription,
  Action: EmptyStateAction,
  Skeleton: EmptyStateSkeleton,
});
