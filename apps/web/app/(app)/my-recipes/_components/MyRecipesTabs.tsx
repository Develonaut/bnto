"use client";

import { ExecutionHistory } from "./ExecutionHistory";
import { RecipeGrid } from "./RecipeGrid";
import { UsageStats } from "./UsageStats";
import { Stack, Tabs, TabsList, TabsTrigger, TabsContent } from "@bnto/ui";

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
        <TabsList>
          <TabsTrigger value="saved">Saved</TabsTrigger>
          <TabsTrigger value="history">History</TabsTrigger>
        </TabsList>

        {/*
         * forceMount keeps both panels in the DOM so switching
         * tabs doesn't remount components or refetch data.
         *
         * The inactive panel uses invisible + absolute so it's
         * removed from layout flow but stays mounted. The active
         * panel stays in normal flow and determines container height.
         */}
        <div className="relative">
          <TabsContent
            value="saved"
            forceMount
            className="pt-4 data-[state=inactive]:invisible data-[state=inactive]:absolute data-[state=inactive]:inset-0"
          >
            <RecipeGrid />
          </TabsContent>

          <TabsContent
            value="history"
            forceMount
            className="pt-4 data-[state=inactive]:invisible data-[state=inactive]:absolute data-[state=inactive]:inset-0"
          >
            <ExecutionHistory />
          </TabsContent>
        </div>
      </Tabs>
    </Stack>
  );
}
