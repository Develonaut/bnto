"use client";

import { useMemo } from "react";
import { core, deriveAcceptedTypes } from "@bnto/core";
import { toDropzoneAccept } from "@bnto/ui";

/**
 * Derives the recipe definition and I/O config from the input node.
 *
 * Returns the definition, file accept label, and dropzone accept
 * config for the given slug. Also determines whether the recipe
 * can run in the browser (all predefined recipes are browser-capable).
 */
export function useRecipeDefinition(slug: string) {
  const recipe = core.registry.getRecipeBySlug(slug);
  const definition = recipe?.definition;
  const isBrowserPath = !!recipe;

  const { acceptLabel, dropzoneAccept } = useMemo(() => {
    if (!definition) return { acceptLabel: "files", dropzoneAccept: undefined };
    const { accept, label } = deriveAcceptedTypes(definition);
    return { acceptLabel: label, dropzoneAccept: toDropzoneAccept(accept) };
  }, [definition]);

  return { recipe, definition, isBrowserPath, acceptLabel, dropzoneAccept };
}
