"use client";

import { useCallback, useState } from "react";

import { InView, SlideUp } from "@bnto/ui";

import { BuildYourOwnCopy } from "./BuildYourOwnCopy";
import { BuildYourOwnEditor } from "./BuildYourOwnEditor";
import { RECIPE_SNIPPETS } from "./recipeSnippets";

/** "Build your own" section — copy left, editor right. */
export function BuildYourOwnSection() {
  const [selectedId, setSelectedId] = useState(RECIPE_SNIPPETS[0].id);
  const current = RECIPE_SNIPPETS.find((s) => s.id === selectedId) ?? RECIPE_SNIPPETS[0];
  const handleChange = useCallback((v: string) => setSelectedId(v), []);

  return (
    <InView>
      <div className="grid items-center gap-12 lg:grid-cols-[1fr_2fr] lg:gap-20">
        <SlideUp>
          <BuildYourOwnCopy mascot={current.mascot} slug={current.id} />
        </SlideUp>
        <BuildYourOwnEditor
          selectedId={selectedId}
          current={current}
          onSelectionChange={handleChange}
        />
      </div>
    </InView>
  );
}
