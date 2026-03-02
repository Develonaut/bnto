"use client";

import dynamic from "next/dynamic";

import { AppShell } from "@/components/ui/AppShell";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

const ExecutionList = dynamic(
  () =>
    import("./_components/ExecutionList").then((m) => ({
      default: m.ExecutionList,
    })),
  { ssr: false },
);

/* ── Execution History Page ──────────────────────────────────── */

export default function ExecutionsPage() {
  return (
    <AppShell.Content>
      <Stack className="gap-8">
        <Stack className="gap-1">
          <Heading level={1} size="lg">
            Execution History
          </Heading>
          <Text color="muted">All your recipe runs, most recent first.</Text>
        </Stack>

        <ExecutionList />
      </Stack>
    </AppShell.Content>
  );
}
