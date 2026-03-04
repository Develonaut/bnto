"use client";

import { useMemo } from "react";
import { ReactFlowProvider } from "@xyflow/react";
import { getRecipeBySlug } from "@bnto/nodes";
import { EditorProvider } from "@/editor";
import type { RecipeMetadata } from "@/editor";
import { EditorCanvas } from "./EditorCanvas";

/**
 * RecipeEditor — canvas-first editor with floating panels.
 *
 * The canvas fills the entire container. Sidebar (left) and config
 * panel (right) float on top as elevated Panel surfaces. Toolbar
 * floats bottom-center.
 *
 * Two entry modes:
 *   <RecipeEditor slug="compress-images" />  — loads predefined recipe
 *   <RecipeEditor />                         — blank canvas
 */

interface RecipeEditorProps {
  slug?: string;
}

function RecipeEditorRoot({ slug }: RecipeEditorProps) {
  const initialMetadata = useMemo((): RecipeMetadata | undefined => {
    if (!slug) return undefined;
    const recipe = getRecipeBySlug(slug);
    if (!recipe) return undefined;
    const def = recipe.definition;
    return { id: def.id, name: def.name, type: def.type, version: def.version };
  }, [slug]);

  return (
    <EditorProvider initialMetadata={initialMetadata}>
      <ReactFlowProvider>
        <EditorCanvas initialSlug={slug} />
      </ReactFlowProvider>
    </EditorProvider>
  );
}

export { RecipeEditorRoot };
