import { cn } from "../../utils/cn";
import { Stack } from "../../layout/Stack";

export function EmptyStateSkeleton({ className }: { className?: string }) {
  return (
    <Stack align="center" gap="sm" className={cn("py-10", className)}>
      <div className="size-10 rounded-full bg-muted motion-safe:animate-pulse" />
      <div className="h-5 w-32 rounded-md bg-muted motion-safe:animate-pulse" />
      <div className="h-4 w-48 rounded-md bg-muted motion-safe:animate-pulse" />
    </Stack>
  );
}
