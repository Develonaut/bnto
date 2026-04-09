"use client";

import { ScaleIn, Tabs, TabsList, TabsTrigger } from "@bnto/ui";

import { CodeEditor } from "@/components/blocks/CodeEditor";

import type { RecipeSnippet } from "./recipeSnippets";
import { RECIPE_SNIPPETS } from "./recipeSnippets";

interface BuildYourOwnEditorProps {
  selectedId: string;
  current: RecipeSnippet;
  onSelectionChange: (id: string) => void;
}

/** Interactive recipe selector + animated code editor preview. */
export function BuildYourOwnEditor({
  selectedId,
  current,
  onSelectionChange,
}: BuildYourOwnEditorProps) {
  return (
    <ScaleIn from={0.95} easing="spring" className="min-w-0">
      <div className="flex w-full min-w-0 flex-col gap-4">
        <Tabs value={selectedId} onValueChange={onSelectionChange}>
          <TabsList fullWidth>
            {RECIPE_SNIPPETS.map((snippet) => (
              <TabsTrigger key={snippet.id} value={snippet.id}>
                {snippet.label}
              </TabsTrigger>
            ))}
          </TabsList>
        </Tabs>

        {/* Animated code editor — key swap forces animation replay */}
        <CodeEditor key={selectedId} lines={current.lines} filename={current.filename} />
      </div>
    </ScaleIn>
  );
}
