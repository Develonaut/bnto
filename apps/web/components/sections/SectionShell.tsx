import type { ReactNode } from "react";
import { Container, Stack } from "@bnto/ui";

interface SectionShellProps {
  muted?: boolean;
  size?: "md" | "lg";
  children: ReactNode;
}

export function SectionShell({ muted, size = "lg", children }: SectionShellProps) {
  if (muted) {
    return (
      <div
        className="rounded-lg border bg-muted p-3 lg:p-4"
        style={{ borderColor: "var(--surface-muted-wall)" }}
      >
        <Stack gap="lg">{children}</Stack>
      </div>
    );
  }

  return (
    <Container size={size}>
      <Stack gap="lg">{children}</Stack>
    </Container>
  );
}
