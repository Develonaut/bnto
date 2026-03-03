"use client";

import dynamic from "next/dynamic";

import { AppShell } from "@/components/ui/AppShell";
import { Heading } from "@/components/ui/Heading";
import { Stack } from "@/components/ui/Stack";
import { Tabs } from "@/components/ui/Tabs";
import { Text } from "@/components/ui/Text";

const UsageStats = dynamic(
  () => import("./_components/UsageStats").then((m) => ({ default: m.UsageStats })),
  { ssr: false, loading: () => <UsageStatsFallback /> },
);

const RecipeGrid = dynamic(
  () => import("./_components/RecipeGrid").then((m) => ({ default: m.RecipeGrid })),
  { ssr: false, loading: () => <TabPanelFallback /> },
);

const ExecutionHistory = dynamic(
  () => import("./_components/ExecutionHistory").then((m) => ({ default: m.ExecutionHistory })),
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

export default function MyRecipesPage() {
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

        <Stack className="gap-8">
          <UsageStats />

          <Tabs defaultValue="history">
            <Tabs.List>
              <Tabs.Trigger value="history">History</Tabs.Trigger>
              <Tabs.Trigger value="saved">Saved</Tabs.Trigger>
            </Tabs.List>

            {/*
             * forceMount keeps both panels in the DOM so switching
             * tabs doesn't remount components or refetch data.
             *
             * The inactive panel uses invisible + absolute so it's
             * removed from layout flow but stays mounted. The active
             * panel stays in normal flow and determines container height.
             */}
            <div className="relative">
              <Tabs.Content
                value="history"
                forceMount
                className="pt-4 data-[state=inactive]:invisible data-[state=inactive]:absolute data-[state=inactive]:inset-0"
              >
                <ExecutionHistory />
              </Tabs.Content>

              <Tabs.Content
                value="saved"
                forceMount
                className="pt-4 data-[state=inactive]:invisible data-[state=inactive]:absolute data-[state=inactive]:inset-0"
              >
                <RecipeGrid />
              </Tabs.Content>
            </div>
          </Tabs>
        </Stack>
      </Stack>
    </AppShell.Content>
  );
}
