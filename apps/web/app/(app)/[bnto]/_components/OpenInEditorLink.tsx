"use client";

import { useCallback, useState } from "react";
import { useRouter } from "next/navigation";
import { Badge, Button, PenLineIcon, LoaderIcon } from "@bnto/ui";
import { core } from "@bnto/core";
import { editorUrl } from "@/lib/routes";
import { getRecipeBySlug } from "@bnto/nodes";

/**
 * "Open in Editor" — clones the predefined recipe into the personal
 * store and navigates to the editor with the new recipe ID.
 */
export function OpenInEditorLink({ slug }: { slug: string }) {
  const router = useRouter();
  const [cloning, setCloning] = useState(false);

  const handleClick = useCallback(() => {
    const recipe = getRecipeBySlug(slug);
    if (!recipe) return;

    setCloning(true);
    const id = core.recipes.createFromTemplate(recipe);
    router.push(editorUrl(id));
  }, [slug, router]);

  return (
    <Button onClick={handleClick} variant="outline" elevation="sm" disabled={cloning}>
      {cloning ? (
        <LoaderIcon className="size-3.5 motion-safe:animate-spin" />
      ) : (
        <PenLineIcon className="size-3.5" />
      )}
      Open in Editor
      <Badge variant="secondary">Beta</Badge>
    </Button>
  );
}
