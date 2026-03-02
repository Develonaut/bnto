"use client";

import dynamic from "next/dynamic";

import { core } from "@bnto/core";

import { AppShell } from "@/components/ui/AppShell";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Tabs } from "@/components/ui/Tabs";
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

/* ── Dashboard Page ──────────────────────────────────────────── */

export default function WorkflowsPage() {
  const { isAuthenticated, isLoading } = core.auth.useAuth();

  return (
    <AppShell.Content>
      <Stack className="gap-8">
        <Stack className="gap-1">
          <Heading level={1} size="lg">
            Dashboard
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

            <Tabs defaultValue="recent">
              <Tabs.List>
                <Tabs.Trigger value="recent">Recent</Tabs.Trigger>
                <Tabs.Trigger value="saved">Saved</Tabs.Trigger>
              </Tabs.List>

              <Tabs.Content value="recent">
                <Stack className="gap-3 pt-4">
                  <RecentExecutions />
                </Stack>
              </Tabs.Content>

              <Tabs.Content value="saved">
                <Stack className="gap-3 pt-4">
                  <WorkflowGrid />
                </Stack>
              </Tabs.Content>
            </Tabs>
          </>
        )}
      </Stack>
    </AppShell.Content>
  );
}
