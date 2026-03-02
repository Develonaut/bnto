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
  { ssr: false, loading: () => <UsageStatsFallback /> },
);

const WorkflowGrid = dynamic(
  () => import("./_components/WorkflowGrid").then((m) => ({ default: m.WorkflowGrid })),
  { ssr: false, loading: () => <TabPanelFallback /> },
);

const RecentExecutions = dynamic(
  () => import("./_components/RecentExecutions").then((m) => ({ default: m.RecentExecutions })),
  { ssr: false, loading: () => <TabPanelFallback /> },
);

/**
 * Inline fallback for UsageStats — 3 stat cards matching the loaded layout.
 * Keeps height stable while the dynamic import resolves.
 */
function UsageStatsFallback() {
  return (
    <div className="flex flex-wrap gap-3">
      {Array.from({ length: 3 }).map((_, i) => (
        <div key={i} className="flex-1 rounded-lg border border-border bg-card px-4 py-3 shadow-sm">
          <div className="flex flex-col gap-1.5">
            <div className="h-3 w-16 rounded-md bg-muted motion-safe:animate-pulse" />
            <div className="h-6 w-12 rounded-md bg-muted motion-safe:animate-pulse" />
          </div>
        </div>
      ))}
    </div>
  );
}

/**
 * Inline fallback for tab panels during dynamic import.
 * Reserves the same min-height as the loaded tab content
 * so the page doesn't collapse then expand.
 */
function TabPanelFallback() {
  return <div className="min-h-[280px]" />;
}

/* ── Dashboard Page ──────────────────────────────────────────── */

export default function WorkflowsPage() {
  const { isAuthenticated, isLoading } = core.auth.useAuth();

  return (
    <AppShell.Content>
      <Stack className="gap-8">
        <Stack className="gap-1">
          <Heading level={1} size="lg">
            My Recipes
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

              {/*
               * forceMount + CSS visibility keeps both panels in the DOM.
               * This prevents remount/refetch on tab switch and preserves
               * scroll position. React Query caches keep data warm.
               */}
              <div className="relative min-h-[280px]">
                <Tabs.Content value="recent" forceMount className="pt-4 data-[state=inactive]:hidden">
                  <RecentExecutions />
                </Tabs.Content>

                <Tabs.Content value="saved" forceMount className="pt-4 data-[state=inactive]:hidden">
                  <WorkflowGrid />
                </Tabs.Content>
              </div>
            </Tabs>
          </>
        )}
      </Stack>
    </AppShell.Content>
  );
}
