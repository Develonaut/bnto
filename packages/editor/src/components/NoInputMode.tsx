import {
  CircleOffIcon,
  EmptyState,
  EmptyStateDescription,
  EmptyStateIcon,
  EmptyStateTitle,
} from "@bnto/ui";

/** Input state for `none`-mode recipes — they run without files or other input. */
export function NoInputMode() {
  return (
    <EmptyState size="sm" className="rounded-lg border border-dashed border-border px-4">
      <EmptyStateIcon size="sm">
        <CircleOffIcon />
      </EmptyStateIcon>
      <EmptyStateTitle>No input needed</EmptyStateTitle>
      <EmptyStateDescription>This recipe runs without files or other input.</EmptyStateDescription>
    </EmptyState>
  );
}
