"use client";

import dynamic from "next/dynamic";

import { core } from "@bnto/core";

import { AppShell } from "@/components/ui/AppShell";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Text } from "@/components/ui/Text";

import { SignUpPrompt } from "./_components/SignUpPrompt";

const UsageStats = dynamic(
  () => import("./_components/UsageStats").then((m) => ({ default: m.UsageStats })),
  { ssr: false },
);

const WorkflowGrid = dynamic(
  () => import("./_components/WorkflowGrid").then((m) => ({ default: m.WorkflowGrid })),
  { ssr: false },
);

const RecentExecutions = dynamic(
  () => import("./_components/RecentExecutions").then((m) => ({ default: m.RecentExecutions })),
  { ssr: false },
);

/* ── Workflows (Dashboard) Page ──────────────────────────────── */

export default function WorkflowsPage() {
  const { isAuthenticated, isLoading } = core.auth.useAuth();

  return (
    <AppShell.Content>
      <Stack className="gap-8">
        <Stack className="gap-1">
          <Heading level={1} size="lg">
            Workflows
          </Heading>
          <Text color="muted">
            Your saved recipes and recent activity.
          </Text>
        </Stack>

        {isLoading ? null : !isAuthenticated ? (
          <SignUpPrompt />
        ) : (
          <>
            <UsageStats />

            <Stack className="gap-3">
              <Heading level={2} size="sm">
                Saved workflows
              </Heading>
              <WorkflowGrid />
            </Stack>

            <Stack className="gap-3">
              <Heading level={2} size="sm">
                Recent runs
              </Heading>
              <RecentExecutions />
            </Stack>
          </>
        )}
      </Stack>
    </AppShell.Content>
  );
}
