"use client";

import { ExecutionHistory } from "./ExecutionHistory";
import { RecipeGrid } from "./RecipeGrid";
import { UsageStats } from "./UsageStats";
import { Tabs } from "@/components/ui/Tabs";
import { Stack } from "@/components/ui/Stack";

/**
 * Interactive tabs + data leaves for the My Recipes page.
 *
 * This is the client boundary — the parent page.tsx is a server component
 * that renders the heading and description statically. This component owns
 * the Tabs state and renders the self-fetching data leaves directly (no
 * dynamic imports, no ssr: false — each leaf handles its own loading).
 */
export function MyRecipesTabs() {
  return (
    <Stack className="gap-8">
      <UsageStats />

      <Tabs defaultValue="saved">
        <Tabs.List>
          <Tabs.Trigger value="saved">Saved</Tabs.Trigger>
          <Tabs.Trigger value="history">History</Tabs.Trigger>
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
            value="saved"
            forceMount
            className="pt-4 data-[state=inactive]:invisible data-[state=inactive]:absolute data-[state=inactive]:inset-0"
          >
            <RecipeGrid />
          </Tabs.Content>

          <Tabs.Content
            value="history"
            forceMount
            className="pt-4 data-[state=inactive]:invisible data-[state=inactive]:absolute data-[state=inactive]:inset-0"
          >
            <ExecutionHistory />
          </Tabs.Content>
        </div>
      </Tabs>
    </Stack>
  );
}
