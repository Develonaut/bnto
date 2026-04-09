"use client";

import { useCallback } from "react";
import Image from "next/image";
import { useRouter } from "next/navigation";
import { ArrowRight } from "lucide-react";
import { Button, Heading, Stack, Surface, Text } from "@bnto/ui";
import { core } from "@bnto/core";
import { editorUrl } from "@/lib/routes";

interface BuildYourOwnCopyProps {
  mascot: string;
  slug: string;
}

/** "Build your own" copy block — mascot above body text. */
export function BuildYourOwnCopy({ mascot, slug }: BuildYourOwnCopyProps) {
  const router = useRouter();

  const handleOpenEditor = useCallback(() => {
    const recipe = core.registry.getRecipeBySlug(slug);
    if (!recipe) return;
    const id = core.recipes.createFromDefinition(recipe.definition);
    router.push(editorUrl(id));
  }, [slug, router]);

  return (
    <Stack gap="md">
      <div className="relative h-[280px] w-[280px]">
        <Image src={mascot} alt="" fill className="object-contain" aria-hidden />
      </div>
      <Text size="sm" mono color="muted" className="uppercase tracking-wider">
        Config as code
      </Text>
      <Heading level={2} size="xl">
        Pack your own bento.
      </Heading>
      <Text color="muted" leading="snug">
        Any node, any combination. Recipes are plain JSON. Chain nodes together, save as{" "}
        <Surface asChild elevation="none" variant="muted">
          <code className="font-mono text-foreground rounded-md px-1.5 py-0.5">.bnto.json</code>
        </Surface>
        , run anywhere.
      </Text>
      <div className="pt-2">
        <Button variant="secondary" onClick={handleOpenEditor}>
          Open the editor
          <ArrowRight className="size-4" />
        </Button>
      </div>
    </Stack>
  );
}
